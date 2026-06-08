use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rusqlite::{named_params, Row};
use serde::Serialize;
use tauri::State;

use crate::{
    db_schema::{db_get_row, with_db},
    get_preview_folder, get_thumbnail_folder, SharedDbState,
};

#[derive(Serialize)]
pub struct RawRecord {
    pub id: i64,
    pub cam_id: String,
    pub raw_path: Option<String>,
    pub jpg_path: Option<String>,
    pub preview: Option<String>,
    pub thumbnail: Option<String>,
    pub date_taken: Option<String>,
}

const RAWS_COLUMNS: &str = "
    raw.id, 
    raw.cam_id, 
    raw.raw_path, 
    raw.jpg_path, 
    raw.preview, 
    raw.thumbnail, 
    raw.date_taken, 
    raw.author,
    EXISTS(SELECT 1 FROM trash_bin WHERE type = 'raw' AND item_id = raw.cam_id) AS in_trash
";

const RAWS_FROM: &str = "
    FROM raws raw
";

impl<'a> TryFrom<&HashMap<String, String>> for RawRecord {
    type Error = String;
    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(RawRecord {
            id: map["raw.id"].parse::<i64>().unwrap(),
            cam_id: map["raw.cam_id"].clone(),
            raw_path: map.get("raw.raw_path").filter(|s| !s.is_empty()).cloned(),
            jpg_path: map.get("raw.jpg_path").filter(|s| !s.is_empty()).cloned(),
            preview: map.get("raw.preview").filter(|s| !s.is_empty()).cloned(),
            thumbnail: map.get("raw.thumbnail").filter(|s| !s.is_empty()).cloned(),
            date_taken: map.get("raw.date_taken").filter(|s| !s.is_empty()).cloned(),
        })
    }
}

impl RawRecord {
    pub fn from_cam_id(
        cam_id: &str,
        state: &State<'_, SharedDbState>,
    ) -> Result<RawRecord, String> {
        let result = db_get_row(
            &format!(
                "SELECT {} {} WHERE raw.cam_id = :cam_id",
                RAWS_COLUMNS, RAWS_FROM
            ),
            named_params! { ":cam_id": cam_id },
            state,
        )?;

        RawRecord::try_from(&result)
    }

    // Obsolete now
    pub fn from_row(row: &Row) -> Result<RawRecord, rusqlite::Error> {
        Ok(RawRecord {
            id: row.get("raw.id")?,
            cam_id: row.get("raw.cam_id")?,
            raw_path: row.get("raw.raw_path")?,
            jpg_path: row.get("raw.jpg_path")?,
            preview: row.get("raw.preview")?,
            thumbnail: row.get("raw.thumbnail")?,
            date_taken: row.get("raw.date_taken")?,
        })
    }

    pub fn get_raw_path(&self) -> Result<PathBuf, String> {
        let path = self
            .raw_path
            .as_ref()
            .ok_or("Fetching raw path when raw path is empty.")?;
        Ok(PathBuf::from(path))
    }

    pub fn get_jpg_path(&self) -> Result<PathBuf, String> {
        let path = self
            .jpg_path
            .as_ref()
            .ok_or("Fetching jpg path when jpg path is empty.")?;
        Ok(PathBuf::from(path))
    }

    pub fn get_thumbnail_path(&self) -> Result<String, String> {
        let base_folder = get_thumbnail_folder();
        let filename = format!("{}_thumbnail.jpg", self.cam_id);
        Ok(format!("{base_folder}/{filename}"))
        // Ok(Path::new(get_thumbnail_folder()?).join(format!("{}_thumbnail.jpg", { &self.cam_id? })))
    }

    pub fn get_preview_path(&self) -> Result<PathBuf, String> {
        let base_folder = get_preview_folder();
        let filename = format!("{}get_preview_folder.jpg", self.cam_id);
        Ok(PathBuf::from(base_folder).join(filename))
    }

    pub fn has_nef_file(&self) -> bool {
        self.raw_path.as_deref().is_some_and(|s| !s.is_empty())
    }

    pub fn has_thumbnail_file(&self) -> Result<bool, String> {
        let p = self.get_thumbnail_path()?;
        let path = Path::new(&p);

        Ok(path.exists())
    }
}
