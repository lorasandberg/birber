use std::collections::HashMap;

use rusqlite::{named_params, Connection, Row};
use serde::Serialize;
use tauri::State;

use crate::{
    db_schema::{db_execute, db_get_row, db_insert_row, with_db},
    entities::{
        pipeline_parameters::{PipelineOutFile, PipelineParameters},
        raw_record::RawRecord,
    },
    get_preview_folder, SharedDbState,
};

#[derive(Serialize)]
pub struct PhotoRecord {
    pub id: i64,
    pub raw: RawRecord,
    pub file_path: Option<String>,
    pub pipeline_params: PipelineParameters,
    pub rating: Option<String>,
}

impl PhotoRecord {
    pub fn from_id(id: i64, state: &State<'_, SharedDbState>) -> Result<PhotoRecord, String> {
        println!("Id is: {:?}", id);
        let data = db_get_row(
            "SELECT * FROM photos WHERE photos.id = :id",
            named_params! { ":id": id },
            state,
        )?;

        println!("{:?}", data);
        PhotoRecord::from_row(data, state)
    }

    pub fn from_row(
        row: HashMap<String, String>,
        state: &State<'_, SharedDbState>,
    ) -> Result<PhotoRecord, String> {
        Ok(PhotoRecord {
            id: row["id"].parse::<i64>().unwrap(),
            raw: RawRecord::from_cam_id(&row["raw"], state)?,
            file_path: (!row["file_path"].is_empty()).then_some(row["file_path"].clone()),
            pipeline_params: PipelineParameters::from_json(&row["pipeline_params"])?,
            rating: (!row["rating"].is_empty()).then_some(row["rating"].clone()),
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
        get_preview_folder()
            + &format!(
                "photo_{id}{suffix}",
                id = self.id,
                suffix = match self.pipeline_params.out_file_type {
                    PipelineOutFile::JPEG => ".jpg",
                    PipelineOutFile::WEBP => ".webp",
                }
            )
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
}
