use image::{
    imageops::FilterType,
    DynamicImage, GenericImageView, ImageBuffer,
    ImageFormat::{self},
    ImageReader, Rgb,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufWriter, Cursor},
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

use entities::raw_record::RawRecord;

use crate::entities::{
    self,
    pipeline_parameters::{PipelineOutFile, PipelineParameters},
};

impl RawRecord {
    // Load raw, convert to TIFF, load TIFF, run render pipeline, return image buffer.
    pub async fn render(
        &self,
        output_path: &str,
        params: &PipelineParameters,
        app_handle: &tauri::AppHandle,
    ) -> Result<(), String> {
        let raw_path = self.get_raw_path()?;
        let tiff_bytes = convert_raw_to_tiff(raw_path, app_handle, &params).await?;
        let render = run_render_pipeline(tiff_bytes, &params)?;
        save_render_results_to_file(render, &params, output_path)?;
        Ok(())
    }
}

// Pushes raw into dcraw_emu, reads the output .tiff file, removes the file and returns the data.
async fn convert_raw_to_tiff(
    input_path: PathBuf,
    app_handle: &tauri::AppHandle,
    params: &PipelineParameters,
) -> Result<Vec<u8>, String> {
    let input_path = input_path.into_os_string().into_string().unwrap();

    let resource_path = app_handle
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?;
    let binaries_path = resource_path.join("binaries");
    let mut command = app_handle
        .shell()
        .sidecar("dcraw_emu")
        .map_err(|e| e.to_string())?;

    // To make sure dcraw_emu has access to libraw.dll, edit command PATH env variable before running it.
    command = command.env(
        "PATH",
        format!(
            "{};{}",
            binaries_path.to_string_lossy(),
            std::env::var("PATH").unwrap_or_default()
        ),
    );

    // Add -h for half-size testing image.
    let mut args = vec!["-4", "-T", "-o0", &input_path];
    if params.to_be_resized() {
        args.insert(2, "-h");
    }

    let output = command
        .args(args)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "dcraw_emu exited with error code {:?}. System message: {}",
            output.status.code(),
            stderr_msg
        ));
    }

    let tiff_path = format!("{input_path}.tiff");
    let tiff_bytes = fs::read(&tiff_path).map_err(|e| e.to_string())?;

    fs::remove_file(&tiff_path).map_err(|e| e.to_string())?;

    Ok(tiff_bytes)
}

struct RenderPipelineResult {
    pixels: Vec<u16>,
    width: u32,
    height: u32,
}

fn run_render_pipeline(
    tiff_bytes: Vec<u8>,
    parameters: &PipelineParameters,
) -> Result<RenderPipelineResult, String> {
    let cursor = Cursor::new(tiff_bytes);
    let mut reader = ImageReader::new(cursor);
    reader.set_format(ImageFormat::Tiff);

    let mut dynamic_image = reader.decode().map_err(|e| e.to_string())?;

    let (full_width, full_height) = dynamic_image.dimensions();
    let (full_width, full_height) = (full_width as f64, full_height as f64);

    // Cropping happens here.
    if parameters.to_be_cropped() {
        dynamic_image = dynamic_image.crop_imm(
            (parameters.crop.x * full_width) as u32,
            (parameters.crop.y * full_height) as u32,
            (parameters.crop.width * full_width) as u32,
            (parameters.crop.height * full_height) as u32,
        );
    }

    let (width, height) = dynamic_image.dimensions();

    let rgb16_image = dynamic_image.to_rgb16();
    let flat_samples = rgb16_image.as_flat_samples();

    let mut result_pixels: Vec<u16> = Vec::with_capacity(flat_samples.samples.len());

    let inv_gamma = 1.0 / parameters.target_gamma;

    // Loop through every pixel of the image data.
    for &sample in flat_samples.samples {
        let mut value = sample as f32 / 65535.0;

        // Here:
        // Add exposure
        // Add contrast

        // Gamma correction
        // Do always last.

        value = value.powf(inv_gamma);

        let scaled = (value * 65535.0).clamp(0.0, 65535.0) as u16;
        result_pixels.push(scaled);
    }

    Ok(RenderPipelineResult {
        pixels: result_pixels,
        width,
        height,
    })
}

fn save_render_results_to_file(
    result: RenderPipelineResult,
    params: &PipelineParameters,
    output_path: &str,
) -> Result<(), String> {
    let buffer = ImageBuffer::<Rgb<u16>, _>::from_raw(result.width, result.height, result.pixels)
        .ok_or_else(|| "Render result to imagebuffer error")?;

    let dynamic_16bit_image = DynamicImage::ImageRgb16(buffer);

    let resized_image = dynamic_16bit_image.resize(
        params.target_width,
        params.target_height,
        FilterType::Lanczos3,
    );

    let final_8bit_rgb = resized_image.to_rgb8();

    let file = File::create(Path::new(&output_path)).map_err(|e| e.to_string())?;
    let ref mut buffered_writer = BufWriter::new(file);

    match params.out_file_type {
        PipelineOutFile::WEBP => {
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(buffered_writer);

            final_8bit_rgb
                .write_with_encoder(encoder)
                .map_err(|e| e.to_string())?;
        }
        PipelineOutFile::JPEG => {
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(buffered_writer, 95);

            final_8bit_rgb
                .write_with_encoder(encoder)
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
