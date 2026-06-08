use std::env;
use std::fs;
use tauri::Manager;
use tauri::PhysicalPosition;

mod db_schema;
pub mod entities;
mod metatable;
mod photo_file_manager;
mod photo_queries;
mod raw_processing;
mod sync;

pub struct SharedDbState(db_schema::DbPool);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let db_path = format!("sqlite://{}", get_db_path().unwrap());
    // println!("{}", &db_path);

    dotenvy::dotenv().ok();

    let shared_db_pool = db_schema::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // .plugin(
        //     tauri_plugin_sql::Builder::default()
        //         .add_migrations(&db_path, db_schema::get_migrations())
        //         .build(),
        // )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(SharedDbState(shared_db_pool.clone()))
        .invoke_handler(tauri::generate_handler![
            sync::sync_all,
            photo_queries::get_dates_with_photos,
            photo_queries::get_raws_by_date,
            photo_queries::get_raw_by_cam_id,
            photo_file_manager::trigger_create_thumbnail,
            photo_file_manager::create_all_missing_thumbnails,
            photo_queries::create_new_photo,
            photo_queries::throw_out_raw,
            photo_queries::get_bin_status
        ])
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
        .setup(|_app| {
            create_helper_folders();
            Ok(())
        })
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                if let (Ok(availableMonitors), Ok(Some(current_monitor))) =
                    (window.available_monitors(), window.current_monitor())
                {
                    let secondary = availableMonitors
                        .iter()
                        .find(|m| m.name() != current_monitor.name());

                    if let Some(target_monitor) = secondary {
                        let target_position = target_monitor.position();
                        let _ = window.set_position(PhysicalPosition::new(
                            target_position.x,
                            target_position.y,
                        ));
                    }
                }

                window.show();
                // window.maximize();
                // window.minimize().unwrap();
                // window.show().unwrap();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_helper_folders() -> Result<(), String> {
    fs::create_dir_all(get_thumbnail_folder()).map_err(|e| e.to_string())?;
    fs::create_dir_all(get_preview_folder()).map_err(|e| e.to_string())?;
    Ok(())
}

// Get folder where .exe is located.
pub fn get_exe_folder() -> Result<String, String> {
    let mut exe_path = env::current_exe().expect("Failed to find current exe path");
    exe_path.pop();
    Ok(format!("{}", exe_path.to_string_lossy()))
}

/**
 * Get path to the folder where raws are fetched from and where _birder folder is located.
 */
pub fn get_content_root() -> String {
    env::var("BIRBER_PHOTO_ROOT").unwrap()
}

fn get_birder_folder() -> String {
    get_content_root() + "_birber/"
}

pub fn get_db_path() -> String {
    get_content_root() + "_birber/birber.db"
}

pub fn get_preview_folder() -> String {
    get_birder_folder() + "previews/"
}

pub fn get_thumbnail_folder() -> String {
    get_birder_folder() + "thumbnails/"
}
