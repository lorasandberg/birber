use regex::Regex;
use rusqlite::named_params;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use std::{collections::HashMap, time::UNIX_EPOCH};
use tauri::State;

use crate::db_schema::{db_execute, with_db};
use crate::entities::raw_record::RawRecord;
use crate::metatable::MetaData;
use crate::photo_file_manager::{create_all_missing_thumbnails, create_thumbnail, PhotoMetadata};
use crate::{entities, photo_file_manager, SharedDbState};
// Gets all photo files in the file system and updates DB to match them.
// In practice, check all RAW file names, take a note of all files that are not in DB and add them.
// TODO: Figure out what to do with the JPG files that have no RAW.
// Do we rely on the camera naming policy to identify photos?

#[tauri::command]
pub async fn sync_all(
    state: State<'_, SharedDbState>,
    app_handle: tauri::AppHandle,
) -> Result<PhotoMap, String> {
    // Manage timestamps
    let last_sync = MetaData::get("last_sync", "0", &state)?;
    let last_sync = last_sync
        .parse::<u64>()
        .expect("Failed to parse last sync.");
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_millis() as u64;

    let time_between_syncs = current_timestamp - last_sync;
    let days_since = time_between_syncs / (1000 * 60 * 60 * 24);
    let hours_since = (time_between_syncs / (1000 * 60 * 60)) % 24;

    // Read through source files, ignoring folders with Last modified time earlier that the previous sync.
    // Get all DSC_X photos together, so JPG and NEF files of the same shots are counted together as one entry.
    let photo_map = file_names_to_photo_map(last_sync);

    println!(
        "Syncing {} raws, previous sync {} days and {} hours ago.",
        photo_map.len(),
        days_since,
        hours_since
    );

    // Go through the NEF/JPG pairs and insert them to DB.
    // Also creates thumbnails, gets metadata for dates etc.
    insert_photomap_to_db(&photo_map, &state, &app_handle).await?;

    // Update the sync timestamp.
    MetaData::set("last_sync", &(current_timestamp).to_string(), &state)?;

    println!("Sync completed");
    Ok(photo_map)
}

async fn insert_photomap_to_db(
    map: &HashMap<String, PhotoGroup>,
    state: &State<'_, SharedDbState>,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    for (id, group) in map {
        println!("id: {}, JPG: {}, NEF: {}", &id, &group.jpg, &group.raw);

        let result = RawRecord::sync_from_file(id, group, state, app_handle).await;

        match result {
            Ok(_) => {
                println!("Raw insert success.");
            }
            Err(e) => {
                println!("Raw insert failed: {}", e.to_string());
            }
        }
    }

    Ok(())
}

impl RawRecord {
    /** After reading files from raw folders, sync files to DB (upsert). */
    pub async fn sync_from_file(
        cam_id: &str,
        group: &PhotoGroup,
        state: &State<'_, SharedDbState>,
        app_handle: &tauri::AppHandle,
    ) -> Result<(), String> {
        // Get path to a valid file, no matter whether NEF or JPG
        let path = if group.jpg.is_empty() {
            &group.raw
        } else {
            &group.jpg
        };

        // Fetch metadata from the file.
        let meta: PhotoMetadata = photo_file_manager::read_photo_metadata(path)?;
        let date = meta.date_taken.unwrap_or_default();

        let sql = "INSERT INTO 
            raws (cam_id, raw_path, jpg_path, date_taken) 
            values(:cam_id, :raw_path, :jpg_path, :date) ON CONFLICT(cam_id) 
            DO UPDATE SET 
            raw_path = excluded.raw_path, 
            jpg_path = excluded.jpg_path,
            date_taken = excluded.date_taken";

        db_execute(
            sql,
            named_params! { ":cam_id": cam_id, ":raw_path": group.raw, ":jpg_path": group.jpg, ":date": date },
            state,
        )?;

        let raw = RawRecord::from_cam_id(cam_id, state)?;
        raw.create_thumbnail(app_handle).await?;

        Ok(())
    }
}
// let result = with_db(state, |conn| {
//     conn.execute("INSERT INTO meta (id, value) values(?1, ?2) ON CONFLICT(id) DO UPDATE SET value = excluded.value;", [&id, &value])
// });

// match result {
//     Ok(_) => Ok(()),
//     Err(other_error) => Err(other_error.to_string()),
// }

#[derive(Serialize)]
pub struct PhotoGroup {
    pub raw: String,
    pub jpg: String,
}

type PhotoMap = HashMap<String, PhotoGroup>;

fn file_names_to_photo_map(last_sync: u64) -> PhotoMap {
    let mut map = HashMap::new();

    let file_names = list_unique_files(last_sync).unwrap();

    for file_name in file_names {
        if let FileNameMatch::Match { name, suffix } = get_file_name_match(&file_name) {
            let group = map.entry(name).or_insert_with(|| PhotoGroup {
                raw: "".to_string(),
                jpg: "".to_string(),
            });

            match suffix.as_str() {
                "JPG" => group.jpg = file_name.clone(),
                "NEF" => group.raw = file_name.clone(),
                _ => (),
            }
        }
    }

    map
}

enum FileNameMatch {
    NoMatch,
    Match { name: String, suffix: String },
}

// Finds DSC_xxxx.[JPG/NEF] file patterns and returns NoMatch or Match(name, suffix).
fn get_file_name_match(path: &String) -> FileNameMatch {
    let re = Regex::new(r"(DSC_\d+)\.([a-zA-Z0-9]+)$").unwrap();

    if let Some(captures) = re.captures(path) {
        let name = captures.get(1).unwrap().as_str().to_string();
        let suffix = captures.get(2).unwrap().as_str().to_string();
        if suffix == "JPG" || suffix == "NEF" {
            FileNameMatch::Match { name, suffix }
        } else {
            FileNameMatch::NoMatch
        }
    } else {
        FileNameMatch::NoMatch
    }
}

// List all files in Photos
fn list_unique_files(last_sync: u64) -> Result<Vec<String>, String> {
    let path = Path::new("B:/Photos/");

    if !path.is_dir() {
        return Err("Path to Photos is not valid".to_string());
    }

    let mut file_names = Vec::new();

    list_unique_files_in_dir(path, &mut file_names, last_sync).map_err(|e| e.to_string())?;

    println!("{} photos in directory.", file_names.len());

    Ok(file_names)
}

// Helper function to do it recursively.
fn list_unique_files_in_dir(
    dir: &Path,
    file_names: &mut Vec<String>,
    last_sync: u64,
) -> Result<(), String> {
    // Check folder name to ignore specific folders.
    if dir.file_name().unwrap() == "_birber" {
        return Ok(());
    }

    // Get folder Last modified date.
    let metadata =
        fs::metadata(&dir).map_err(|e| format!("Failed to read folder metadata: {}", e))?;
    let modified_time = metadata
        .modified()
        .map_err(|e| format!("Error with folder modified time: {}", e))?;

    let modified_time = modified_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as u64;

    if (modified_time < last_sync) {
        println!("No changes in folder {}", dir.to_str().unwrap());
        return Ok(());
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                list_unique_files_in_dir(&path, file_names, last_sync)?;
            } else {
                if let Some(path_str) = path.to_str() {
                    file_names.push(path_str.to_string());
                }
            }
        }
    }

    Ok(())
}

struct AdvancedPhotoParams {
    pub iso_speed: String,
    pub camera_model: String,
    pub fstop: String,
    pub exposure_time: String,
    pub exposure_bias: String,
    pub exposure_program: String,
}

struct PhotoRecord {
    pub id: String,
    pub raw_path: String,
    pub jpg_path: String,
    pub date_taken: String,
    pub adv_params: AdvancedPhotoParams,
}

fn read_file(path: String) -> Result<Vec<String>, String> {
    let result = Vec::new();

    Ok(result)
}
