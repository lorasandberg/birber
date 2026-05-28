// Manage file saving, removing, reading, metadata functionality.

use std::{fs::File, io::BufReader};

use exif::{In, Reader, Tag, Value};

#[derive(Debug)]
pub struct PhotoMetadata {
    pub date_taken: Option<String>,
}

pub fn read_photo_metadata(path: &str) -> Result<PhotoMetadata, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let mut buf_reader = BufReader::new(file);

    let exif = Reader::new()
        .read_from_container(&mut buf_reader)
        .map_err(|e| e.to_string())?;

    let mut metadata = PhotoMetadata { date_taken: None };

    if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        if let Value::Ascii(ref vec) = field.value {
            if let Some(first_val) = vec.first() {
                if let Ok(date_str) = std::str::from_utf8(first_val) {
                    metadata.date_taken = Some(date_str.to_string());
                }
            }
        }
    }

    Ok(metadata)
}
