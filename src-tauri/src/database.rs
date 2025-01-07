use rusqlite::{named_params, params, Connection, Result};
use std::{fs, path::Path};
use tauri::{path::BaseDirectory, AppHandle, Manager};

#[path = "./parse.rs"]
mod parse;

const CURRENT_DB_VERSION: u32 = 2;

#[derive(Debug)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub path: String,
    pub filetype: String,
    pub date_modified: i32,
}

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("search.sqlite");
    // ~/Library/Application Support/com.search.app/search.sqlite
    // rm ~/Library/Application\ Support/com.search.app/search.sqlite

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    let spellfix_path = app_handle
        .path()
        .resolve("resources/spellfix.dylib", BaseDirectory::Resource)
        .expect("failed to resolve spellfix.dylib");

    unsafe {
        load_extension(&db, &spellfix_path.as_path());
    }

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

unsafe fn load_extension(conn: &Connection, path: &Path) -> Result<()> {
    let _guard = rusqlite::LoadExtensionGuard::new(conn)?;
    conn.load_extension(path, None)
}

/// Upgrades the database to the current version.
pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.execute_batch(
            "
        CREATE TABLE files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            path TEXT NOT NULL,
            filetype TEXT NOT NULL,
            date_modified INTEGER NOT NULL
        );",
        )?;
        tx.execute_batch(
            "
        CREATE VIRTUAL TABLE IF NOT EXISTS fts_documents USING fts5(content, id UNINDEXED);
        CREATE VIRTUAL TABLE IF NOT EXISTS spellfix1 USING spellfix1;
        ",
        )?;

        tx.commit()?;
    }

    Ok(())
}

pub fn add_file(f: &File, db: &Connection) -> Result<(), rusqlite::Error> {
    if (!file_indexed(f, db)) {
        let mut statement = db.prepare("INSERT INTO files (filename, path, filetype, date_modified) VALUES (@filename, @path, @filetype, @date_modified);")?;
        statement.execute(named_params! {
            "@filename": f.filename,
            "@path": f.path,
            "@filetype": f.filetype,
            "@date_modified": f.date_modified,
        })?;
        // let id_no = db.execute("SELECT f FROM files WHERE", params)
        let last_id = db.last_insert_rowid();
        db.execute(
            "INSERT INTO fts_documents (content, id) VALUES (?1, ?2)",
            params![f.path.to_string(), last_id.to_string()],
        )?;

        let mut inst_spellfix_stmt = db.prepare("INSERT INTO spellfix1(word) VALUES (?1)")?;
        for seg in parse::split_path(&f.path.to_string()) {
            inst_spellfix_stmt.execute(params![seg])?;
        }
    }

    Ok(())
}

fn file_indexed(f: &File, db: &Connection) -> bool {
    return false;
}

pub fn find_file(query: &str, db: &Connection) -> Result<Vec<String>> {
    let mut spellfix_stmt = db.prepare(
        "
        SELECT word FROM spellfix1
        WHERE word MATCH (?1)
        ORDER BY distance LIMIT 1
    ",
    )?;

    let corrected_word: Option<String> =
        match spellfix_stmt.query_row(params![query], |row| row.get(0)) {
            Ok(word) => Some(word),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => {
                eprintln!("Error executing query: {:?}", e);
                None
            }
        };

    let search_query = corrected_word.unwrap_or(query.to_string());

    // println!("Search Query: {}", search_query);

    let mut stmt = db.prepare("SELECT content FROM fts_documents WHERE content MATCH ?")?;
    let results: Vec<String> = stmt
        .query_map([&search_query], |row| row.get(0))? // Map rows to String (content column)
        .filter_map(Result::ok) // Filter out errors
        .collect(); // Collect results into a vector
    Ok(results)
}
