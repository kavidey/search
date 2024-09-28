use rusqlite::{named_params, Connection, Result};
use std::fs;
use tauri::AppHandle;

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
        .path_resolver()
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

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
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
    }

    Ok(())
}

fn file_indexed(f: &File, db: &Connection) -> bool {
    return false;
}

// pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
//     let mut statement = db.prepare("SELECT * FROM items")?;
//     let mut rows = statement.query([])?;
//     let mut items = Vec::new();
//     while let Some(row) = rows.next()? {
//       let title: String = row.get("title")?;

//       items.push(title);
//     }

//     Ok(items)
// }
