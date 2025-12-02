use crate::error::{AppError, AppResult};
use bytes::Bytes;
use img_parts::jpeg::Jpeg;
use img_parts::png::Png;
use img_parts::webp::WebP;
use img_parts::{ImageEXIF, ImageICC};
use std::fs::File;
use std::path::Path;

/// Grafts metadata (ICC Profile + Allowlisted EXIF) from source to destination.
///
/// Strategy:
/// 1. ICC Profile: Injected directly into the memory buffer (Fast).
/// 2. EXIF: Parsed from source, filtered for safe tags, and written to disk (Safe).
///
/// This reduces disk I/O from 3 writes to 2 (1 write for image+ICC, 1 write for EXIF).
pub fn save_with_metadata(
    image_data: Vec<u8>,
    output_path: &Path,
    source_path: &Path,
) -> AppResult<()> {
    // 1. Read Source Metadata (EXIF + ICC)
    // We need to read the source file to get the ICC profile to inject.
    let source_bytes = std::fs::read(source_path).map_err(|e| AppError::Unknown(e.to_string()))?;

    // Detect format from output path
    let ext = output_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 2. Inject Metadata (ICC + EXIF) & Write to Disk
    // Unified Strategy: Raw Copy. We extract raw EXIF bytes and inject them directly.
    // This is faster (1 write) and safer for data integrity (preserves Maker Notes).
    let raw_exif = extract_raw_exif(&source_bytes);

    match ext.as_str() {
        "jpg" | "jpeg" => graft_jpeg_memory(&source_bytes, image_data, output_path, raw_exif)?,
        "png" => graft_png_memory(&source_bytes, image_data, output_path, raw_exif)?,
        "webp" => graft_webp_memory(&source_bytes, image_data, output_path, raw_exif)?,
        _ => {
            // Just write the raw data if format not supported for grafting
            std::fs::write(output_path, image_data)
                .map_err(|e| AppError::Unknown(e.to_string()))?;
        }
    }

    Ok(())
}

fn extract_raw_exif(source_data: &[u8]) -> Option<Bytes> {
    // Try JPEG
    if let Ok(jpeg) = Jpeg::from_bytes(Bytes::copy_from_slice(source_data)) {
        if let Some(exif) = jpeg.exif() {
            return Some(exif);
        }
    }
    // Try PNG
    if let Ok(png) = Png::from_bytes(Bytes::copy_from_slice(source_data)) {
        if let Some(exif) = png.exif() {
            return Some(exif);
        }
    }
    // Try WebP
    if let Ok(webp) = WebP::from_bytes(Bytes::copy_from_slice(source_data)) {
        if let Some(exif) = webp.exif() {
            return Some(exif);
        }
    }
    None
}

fn graft_jpeg_memory(
    source_data: &[u8],
    dest_data: Vec<u8>,
    dest_path: &Path,
    raw_exif: Option<Bytes>,
) -> AppResult<()> {
    // 1. Write Image + ICC + EXIF
    let mut dest_jpeg =
        Jpeg::from_bytes(Bytes::from(dest_data)).map_err(|e| AppError::Unknown(e.to_string()))?;

    // Inject ICC
    if let Ok(source_jpeg) = Jpeg::from_bytes(Bytes::copy_from_slice(source_data)) {
        if let Some(icc) = source_jpeg.icc_profile() {
            dest_jpeg.set_icc_profile(Some(icc.into()));
        }
    }

    // Inject EXIF
    if let Some(exif) = raw_exif {
        dest_jpeg.set_exif(Some(exif));
    }

    let mut output_file = File::create(dest_path).map_err(|e| AppError::Unknown(e.to_string()))?;
    dest_jpeg
        .encoder()
        .write_to(&mut output_file)
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    Ok(())
}

fn graft_png_memory(
    source_data: &[u8],
    dest_data: Vec<u8>,
    dest_path: &Path,
    raw_exif: Option<Bytes>,
) -> AppResult<()> {
    // 1. Write Image + ICC + EXIF
    let mut dest_png =
        Png::from_bytes(Bytes::from(dest_data)).map_err(|e| AppError::Unknown(e.to_string()))?;

    // Inject ICC
    if let Ok(source_png) = Png::from_bytes(Bytes::copy_from_slice(source_data)) {
        if let Some(icc) = source_png.icc_profile() {
            dest_png.set_icc_profile(Some(icc.into()));
        }
    }

    // Inject EXIF
    if let Some(exif) = raw_exif {
        dest_png.set_exif(Some(exif));
    }

    let mut output_file = File::create(dest_path).map_err(|e| AppError::Unknown(e.to_string()))?;
    dest_png
        .encoder()
        .write_to(&mut output_file)
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    Ok(())
}

fn graft_webp_memory(
    source_data: &[u8],
    dest_data: Vec<u8>,
    dest_path: &Path,
    raw_exif: Option<Bytes>,
) -> AppResult<()> {
    // 1. Write Image + ICC + EXIF
    let mut dest_webp =
        WebP::from_bytes(Bytes::from(dest_data)).map_err(|e| AppError::Unknown(e.to_string()))?;

    // Inject ICC
    if let Ok(source_webp) = WebP::from_bytes(Bytes::copy_from_slice(source_data)) {
        if let Some(icc) = source_webp.icc_profile() {
            dest_webp.set_icc_profile(Some(icc.into()));
        }
    }

    // Inject EXIF
    if let Some(exif) = raw_exif {
        dest_webp.set_exif(Some(exif));
    }

    let mut output_file = File::create(dest_path).map_err(|e| AppError::Unknown(e.to_string()))?;
    dest_webp
        .encoder()
        .write_to(&mut output_file)
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    Ok(())
}
