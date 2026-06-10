// Manage file saving, removing, reading, metadata functionality.

use std::{
    clone,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use crate::entities::{
    pipeline_parameters::{PipelineOutFile, PipelineParameters},
    raw_record::RawRecord,
};
use exif::{In, Reader, Tag, Value};
use image::{codecs::jpeg, ImageReader};
use tauri::State;
use tokio::runtime;
use trpl::join_all;

use crate::{
    photo_queries::{get_all_raws, get_raw_by_cam_id},
    SharedDbState,
};

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

#[tauri::command]
pub async fn trigger_create_thumbnail(
    cam_id: &str,
    state: State<'_, SharedDbState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let record: RawRecord = get_raw_by_cam_id(cam_id, state)?;

    create_thumbnail(&record, &app_handle).await
}

// Create thumbnails for all raws that don't have one.
// Can take long.
#[tauri::command]
pub async fn create_all_missing_thumbnails(
    state: State<'_, SharedDbState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let raws = get_all_raws(state)?;
    let mut tasks = Vec::new();

    for raw in raws {
        if !raw.has_thumbnail_file()? {
            let app_handle_clone = app_handle.clone();

            // Run mass operations in async threads.
            let task = tokio::spawn(async move { create_thumbnail(&raw, &app_handle_clone).await });
            tasks.push(task);
        }
    }

    let results = join_all(tasks).await;

    for res in results {
        if let Err(e) = res {
            eprintln!("Thumbnail thread worker panicked: {:?}", e);
        }
    }

    Ok(())
}

pub async fn create_thumbnail(
    raw: &RawRecord,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    let output_path = raw.get_thumbnail_path();

    if raw.has_nef_file() {
        let mut params = PipelineParameters::default();
        params.target_width = 300;
        params.target_height = 200;
        params.out_file_type = PipelineOutFile::WEBP;

        raw.render(&output_path, &params, app_handle).await?;
    }
    // Older files don't have .NEF at all. Let's create a thumbnail from the JPG.
    else if raw.jpg_path.is_some() {
        create_thumbnail_from_jpg(&(raw.jpg_path.as_ref().unwrap()), &output_path).await?;
    }
    Ok(())
}

pub async fn create_thumbnail_from_jpg(input_path: &str, output_path: &str) -> Result<(), String> {
    let file = File::open(input_path).map_err(|e| e.to_string())?;
    let reader = std::io::BufReader::new(file);

    // sdf
    let mut decoder = jpeg_decoder::Decoder::new(reader);
    decoder.scale(300, 200).map_err(|e| e.to_string())?;

    let raw_pixels = decoder.decode().map_err(|e| e.to_string())?;
    let info = decoder.info().ok_or("Decoder error")?;

    let img_buffer = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(
        info.width as u32,
        info.height as u32,
        raw_pixels,
    )
    .ok_or("Failed to create image buffer")?;

    let img = image::DynamicImage::ImageRgb8(img_buffer);

    let thumbnail = img.resize(300, 200, image::imageops::FilterType::Triangle);

    let out_file = File::create(output_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(out_file);

    thumbnail
        .write_to(&mut writer, image::ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;

    Ok(())
}
