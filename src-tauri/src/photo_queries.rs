use rusqlite::named_params;
use tauri::State;

use crate::{
    db_schema::{db_get_row, db_get_rows, db_insert_row, with_db},
    entities::{
        photo_record::PhotoRecord,
        pipeline_parameters::{PipelineOutFile, PipelineParameters},
        raw_record::RawRecord,
    },
    get_preview_folder, SharedDbState,
};

#[tauri::command]
pub fn get_dates_with_photos(state: State<'_, SharedDbState>) -> Result<Vec<String>, String> {
    Ok(db_get_rows(
        "SELECT DISTINCT substr(date_taken, 1, 10) AS unique_date
                FROM raws
                WHERE date_taken IS NOT NULL AND date_taken != ''
                ORDER BY unique_date ASC;",
        &[],
        &state,
    )?
    .iter()
    .map(|row| row["unique_date"].clone())
    .collect())
}

#[tauri::command]
pub fn get_raws_by_date(
    date: &str,
    state: State<'_, SharedDbState>,
) -> Result<Vec<RawRecord>, String> {
    Ok(RawRecord::get_by_date(date, &state)?)
}

#[tauri::command]
pub fn get_photos_by_date(
    date: &str,
    state: State<'_, SharedDbState>,
) -> Result<Vec<PhotoRecord>, String> {
    Ok(PhotoRecord::get_by_date(date, &state)?)
}

#[tauri::command]
pub fn get_raw_by_cam_id(
    cam_id: &str,
    state: State<'_, SharedDbState>,
) -> Result<RawRecord, String> {
    RawRecord::from_cam_id(cam_id, &state)
}

pub fn get_all_raws(state: State<'_, SharedDbState>) -> Result<Vec<RawRecord>, String> {
    Ok(Vec::new())

    // let result: Result<Vec<RawRecord>, rusqlite::Error> = with_db(&state, |conn| {
    //     let mut q = conn.prepare("SELECT * FROM raws")?;

    //     let mut rows = q.query([])?;
    //     let mut results = Vec::new();

    //     while let Some(row) = rows.next()? {
    //         results.push(RawRecord::from_row(&row)?);
    //     }
    //     Ok(results)
    // });

    // result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_new_photo(
    cam_id: &str,
    pipeline_params: PipelineParameters,
    state: State<'_, SharedDbState>,
    app_handle: tauri::AppHandle,
) -> Result<PhotoRecord, String> {
    println!("tauri::create_new_photo");
    // Create DB row
    // Save pipelineparams
    // Get id
    // Create a photo, use id for filename
    // Save filename/path to DB where id = id
    // Return PhotoRecord

    println!("Creating a photo");
    let record = PhotoRecord::create(cam_id, &pipeline_params, &state)?;
    record.render(app_handle).await?;
    record.save(&state)?;
    record.create_thumbnail().await?;
    println!("Done");

    Ok(record)
}

#[tauri::command]
pub fn throw_out_raw(cam_id: String, state: State<'_, SharedDbState>) -> Result<(), String> {
    db_insert_row(
        "INSERT INTO trash_bin (type, item_id) VALUES  (:type, :item_id);",
        named_params! { ":type": "raw", ":item_id": cam_id },
        &state,
    )?;

    Ok(())
}

#[tauri::command]
pub fn get_bin_status(cam_id: String, state: State<'_, SharedDbState>) -> Result<String, String> {
    let result = serde_json::to_string(&db_get_row(
        "SELECT EXISTS(SELECT 1 FROM trash_bin WHERE type = :type AND item_id = :item_id)",
        named_params! { ":type": "raw", ":item_id": cam_id },
        &state,
    )?)
    .map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
pub fn get_photo_by_id(id: i64, state: State<'_, SharedDbState>) -> Result<PhotoRecord, String> {
    PhotoRecord::from_id(id, &state)
}

// Run various commands through React in Tauri.
// Development use only.
#[tauri::command]
pub async fn tauri_testing_function(state: tauri::State<'_, SharedDbState>) -> Result<(), String> {
    let photos = PhotoRecord::get_all(&state)?;

    for photo in photos.iter() {
        photo.create_thumbnail().await?;
    }

    Ok(())
}
