use std::collections::HashMap;

use rusqlite::named_params;
use serde::Serialize;
use tauri::State;

use crate::{
    db_schema::{db_execute, db_get_row, db_get_rows, db_insert_row},
    entities::{pipeline_parameters::PipelineParameters, raw_record::RawRecord},
    get_preview_folder, get_thumbnail_folder,
    photo_file_manager::create_thumbnail_from_jpg,
    SharedDbState,
};

#[derive(Serialize)]
pub struct PhotoRecord {
    pub id: i64,
    pub raw: RawRecord,
    pub file_path: String,
    pub pipeline_params: PipelineParameters,
    pub rating: Option<String>,
    pub thumbnail: String,
}

const PHOTOS_COLUMNS: &str = "
    photo.id,
    photo.raw,
    photo.file_path,
    photo.pipeline_params,
    photo.rating,
    raws.date_taken,
    EXISTS(SELECT 1 FROM trash_bin WHERE type = 'photo' AND item_id = photo.id) AS in_trash
";

const PHOTOS_FROM: &str = "
    FROM photos photo
    INNER JOIN raws 
    ON photo.raw = raws.cam_id
";

impl PhotoRecord {
    pub fn from_id(id: i64, state: &State<'_, SharedDbState>) -> Result<PhotoRecord, String> {
        let sql = format!(
            "SELECT {} {} WHERE photo.id = :id",
            PHOTOS_COLUMNS, PHOTOS_FROM
        );
        let data = db_get_row(&sql, named_params! { ":id": id }, state)?;
        PhotoRecord::from_row(&data, state)
    }

    // Cannot use try_from as DB state has to be passed.
    pub fn from_row(
        row: &HashMap<String, String>,
        state: &State<'_, SharedDbState>,
    ) -> Result<PhotoRecord, String> {
        let id = row["id"].parse::<i64>().unwrap();
        Ok(PhotoRecord {
            id: id,
            raw: RawRecord::from_cam_id(&row["raw"], state)?,
            file_path: PhotoRecord::preview_path_by_id(id), //(!row["file_path"].is_empty()).then_some(row["file_path"].clone()),
            pipeline_params: PipelineParameters::from_json(&row["pipeline_params"])?,
            rating: (!row["rating"].is_empty()).then_some(row["rating"].clone()),
            thumbnail: PhotoRecord::thumbnail_path_by_id(id),
        })
    }

    pub fn create(
        raw_cam_id: &str,
        params: &PipelineParameters,
        state: &State<'_, SharedDbState>,
    ) -> Result<PhotoRecord, String> {
        let id = db_insert_row(
            "INSERT INTO photos (raw, pipeline_params) VALUES(:raw_cam_id, :pipeline_params);",
            named_params! { ":raw_cam_id": raw_cam_id, ":pipeline_params": params.to_json()? },
            state,
        )?;
        Ok(PhotoRecord::from_id(id, state)?)
    }

    pub fn save(&self, state: &State<'_, SharedDbState>) -> Result<(), String> {
        db_execute(
            "UPDATE photos SET pipeline_params = :pipeline_params, rating = :rating WHERE id = :id;",
            named_params! { ":pipeline_params": self.pipeline_params.to_json()?, ":rating": self.rating, ":id": self.id },
            state,
        )
    }

    pub fn get_preview_path(&self) -> String {
        PhotoRecord::preview_path_by_id(self.id)
    }

    pub fn preview_path_by_id(id: i64) -> String {
        format!("{}/photo_{}.jpg", get_preview_folder(), id)
    }

    pub fn get_thumbnail_path(&self) -> String {
        PhotoRecord::thumbnail_path_by_id(self.id)
    }

    pub fn thumbnail_path_by_id(id: i64) -> String {
        format!("{}/PHOTO{}_thumbnail.jpg", get_thumbnail_folder(), id)
    }

    pub async fn render(&self, app_handle: tauri::AppHandle) -> Result<(), String> {
        self.raw
            .render(
                &self.get_preview_path(),
                &(self.pipeline_params),
                &app_handle,
            )
            .await
    }

    pub async fn create_thumbnail(&self) -> Result<(), String> {
        println!(
            "Creating thumbnail for photo {}, input: {}, output: {}",
            self.id,
            self.get_preview_path(),
            self.get_thumbnail_path()
        );
        create_thumbnail_from_jpg(&self.get_preview_path(), &self.get_thumbnail_path()).await?;
        Ok(())
    }

    pub fn get_all(state: &State<'_, SharedDbState>) -> Result<Vec<PhotoRecord>, String> {
        let query = &format!(
            "SELECT {} {} ORDER BY raws.date_taken;",
            PHOTOS_COLUMNS, PHOTOS_FROM
        );

        let result = db_get_rows(query, &[], state)?;

        Ok(result
            .iter()
            .map(|row| PhotoRecord::from_row(row, state).unwrap())
            .collect())
    }

    pub fn get_by_date(
        date: &str,
        state: &State<'_, SharedDbState>,
    ) -> Result<Vec<PhotoRecord>, String> {
        let query = &format!(
            "SELECT {} {} WHERE raws.date_taken LIKE :date ORDER BY raws.date_taken;",
            PHOTOS_COLUMNS, PHOTOS_FROM
        );

        let result = db_get_rows(
            query,
            named_params! { ":date": format!("{}%", date) },
            state,
        )?;

        Ok(result
            .iter()
            .map(|row| PhotoRecord::from_row(row, state).unwrap())
            .collect())
    }
}
