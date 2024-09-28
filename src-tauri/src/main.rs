// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod scan;

use database::File;
use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    // Should handle errors instead of unwrapping here
    // app_handle.db(|db| database::add_item(name, db)).unwrap();

    // let items = app_handle.db(|db| database::get_all(db)).unwrap();

    // let items_string = items.join(" | ");

    format!("Your name log: {}", name)
}

#[tauri::command]
fn index(app_handle: AppHandle, root: &str) {
    scan::index_directory(root, |name, path| {
        app_handle.db(|db| {
            let f = File {
                id: 0,
                filename: name,
                path: path.into_os_string().into_string().unwrap(),
                filetype: "".to_owned(),
                date_modified: 0
            };
            database::add_file(&f, db)
        }).unwrap();
        // println!("File {:?} has full path {:?}", name, path);
    },|error| {
        println!("Error {}. Continued scanning", error)
    });
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![greet, index])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}