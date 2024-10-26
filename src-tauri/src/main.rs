// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod scan;

use database::File;
use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

use candle_core::{Device, Tensor};

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
        println!("File {:?} has full path {:?}", name, path);
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
    },|error| {
        println!("Error {}. Continued scanning", error)
    });
}

// if correct, can unwrap otherwise error 

// TODO: get this working with button thing
#[tauri::command]
fn clip(app_handle: AppHandle) -> Result<(), String> {
    println!("touch");
    match clip_helper() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn clip_helper() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    let b = Tensor::randn(0f32, 1., (3, 4), &device)?;

    let c = a.matmul(&b)?;
    println!("{c}");
    println!("Hi");
    Ok(())
}


fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![greet, clip, index])
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