use std::fs;
use std::path::Path;
use tauri::Manager;
use tauri_plugin_log::log::info;

use crate::db_schema::get_db_path;

mod db_schema;
mod metatable;
mod photo_file_manager;
mod sync;

pub struct SharedDbState(db_schema::DbPool);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let db_path = format!("sqlite://{}", get_db_path().unwrap());
    // println!("{}", &db_path);

    let shared_db_pool = db_schema::init();

    tauri::Builder::default()
        // .plugin(
        //     tauri_plugin_sql::Builder::default()
        //         .add_migrations(&db_path, db_schema::get_migrations())
        //         .build(),
        // )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(SharedDbState(shared_db_pool.clone()))
        .invoke_handler(tauri::generate_handler![sync::sync_all])
        .setup(|app| {
            if let Ok(app_dir) = app.path().app_data_dir() {
                println!("Here: {}", app_dir.display());
            }
            Ok(())
        })
        .setup(move |_app| {
            db_schema::boot_server(shared_db_pool.clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn list_unique_files_in_dir(dir: &Path, file_names: &mut Vec<String>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                list_unique_files_in_dir(&path, file_names)?;
            } else {
                if let Some(path_str) = path.to_str() {
                    file_names.push(path_str.to_string());
                }
            }
        }
    }

    Ok(())
}
