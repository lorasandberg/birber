use serde::{Deserialize, Serialize};

// Setup structure and defaults for render parameters.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineParameters {
    pub out_file_type: PipelineOutFile,
    pub target_gamma: f32,
    pub target_width: u32,
    pub target_height: u32,
    pub crop: PipelineCrop,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PipelineOutFile {
    JPEG,
    WEBP,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PipelineCrop {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for PipelineParameters {
    fn default() -> PipelineParameters {
        PipelineParameters {
            out_file_type: PipelineOutFile::JPEG,
            target_gamma: 2.2,
            target_height: 0,
            target_width: 0,
            crop: PipelineCrop {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        }
    }
}

impl PipelineParameters {
    pub fn to_be_resized(&self) -> bool {
        self.target_width > 0 && self.target_height > 0
    }

    pub fn to_be_cropped(&self) -> bool {
        self.crop.width < 0.99 || self.crop.height < 0.99
    }

    pub fn from_json(data: &str) -> Result<PipelineParameters, String> {
        Ok(serde_json::from_str::<PipelineParameters>(data).map_err(|e| e.to_string())?)
    }

    pub fn to_json(&self) -> Result<String, String> {
        Ok(serde_json::to_string(self).map_err(|e| e.to_string())?)
    }
}

//     fn get_value<T>(row: &Row, field: &str) -> Result<T, String>
//     where
//         T: rusqlite::types::FromSql,
//     {
//         row.get(field).map_err(|e| e.to_string())
//     }

//     Ok(PhotoRecord {
//         id: get_value::<i64>(row, "id")?,
//         raw: RawRecord::from_cam_id(&get_value::<String>(row, "raw")?, state)?,
//         file_path: get_value::<Option<String>>(row, "file_path")?,
//         pipeline_params: serde_json::from_str::<PipelineParameters>(&get_value::<String>(
//             row,
//             "pipeline_params",
//         )?)
//         .map_err(|e| e.to_string())?,
//         rating: get_value::<Option<String>>(row, "rating")?,
//     })
// }
