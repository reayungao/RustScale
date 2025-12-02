use crate::error::{AppError, AppResult};
// FIX: Import Image from the 'images' submodule for newer versions of fast_image_resize
use fast_image_resize::images::Image;
use fast_image_resize::Resizer;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use sysinfo::System;

pub fn load_image(path: &Path) -> AppResult<DynamicImage> {
    let file = File::open(path).map_err(|e| AppError::Unknown(e.to_string()))?;
    let mmap = unsafe { memmap2::Mmap::map(&file).map_err(|e| AppError::Unknown(e.to_string()))? };
    let mut image = image::load_from_memory(&mmap).map_err(AppError::from)?;

    if image.color() != image::ColorType::Rgb8 {
        image = DynamicImage::ImageRgb8(image.to_rgb8());
    }

    // EXIF orientation handling removed to prevent double-rotation.
    // Since we are now using "Raw Copy" for metadata, we should process the image
    // exactly as it is stored (raw pixels) and let the copied metadata handle the rotation.
    // This fixes the issue where images were rotated twice (once here, once by the viewer).
    Ok(image)
}

pub fn encode_image(
    image: &DynamicImage,
    format: image::ImageFormat,
    compression: &str,
) -> AppResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut w = std::io::Cursor::new(&mut buffer);

    if format == image::ImageFormat::Png {
        // OPTIMIZATION: Use Fast compression for speed.
        // Default is Best/Slow, which takes 20s+ for large images. Fast takes ~2-5s.
        let encoder = image::codecs::png::PngEncoder::new_with_quality(
            w,
            image::codecs::png::CompressionType::Fast,
            image::codecs::png::FilterType::Adaptive,
        );
        image.write_with_encoder(encoder).map_err(AppError::from)?;
    } else if format == image::ImageFormat::WebP {
        if compression == "lossy" {
            // Use webp crate for Lossy (supports quality configuration)
            let width = image.width();
            let height = image.height();

            let memory = match image.color() {
                image::ColorType::Rgb8 => {
                    let rgb = image.to_rgb8();
                    let encoder = webp::Encoder::from_rgb(rgb.as_raw(), width, height);
                    encoder.encode(75.0)
                }
                _ => {
                    let rgba = image.to_rgba8();
                    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), width, height);
                    encoder.encode(75.0)
                }
            };
            return Ok(memory.to_vec());
        } else {
            // Use image crate for Lossless (Robust & Standard)
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(w);
            image.write_with_encoder(encoder).map_err(AppError::from)?;
        }
    } else if format == image::ImageFormat::Jpeg {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(w, 90);
        image.write_with_encoder(encoder).map_err(AppError::from)?;
    } else {
        image.write_to(&mut w, format).map_err(AppError::from)?;
    }

    Ok(buffer)
}

pub fn save_image(image: &DynamicImage, path: &Path, compression: &str) -> AppResult<()> {
    let format = image::ImageFormat::from_path(path).unwrap_or(image::ImageFormat::Png);
    let data = encode_image(image, format, compression)?;
    std::fs::write(path, data).map_err(|e| AppError::Unknown(e.to_string()))?;
    Ok(())
}

pub fn resize_image(image: &DynamicImage, width: u32, height: u32) -> AppResult<DynamicImage> {
    let src_width = std::num::NonZeroU32::new(image.width()).unwrap();
    let src_height = std::num::NonZeroU32::new(image.height()).unwrap();
    let src_image = Image::from_vec_u8(
        src_width.into(),
        src_height.into(),
        image.to_rgb8().into_raw(),
        fast_image_resize::PixelType::U8x3,
    )
    .map_err(|e| AppError::ImageError(e.to_string()))?;

    let dst_width = std::num::NonZeroU32::new(width).unwrap();
    let dst_height = std::num::NonZeroU32::new(height).unwrap();
    let mut dst_image = Image::new(dst_width.into(), dst_height.into(), src_image.pixel_type());

    let mut resizer = Resizer::new();
    resizer
        .resize(
            &src_image,
            &mut dst_image,
            &fast_image_resize::ResizeOptions::new().resize_alg(
                fast_image_resize::ResizeAlg::Convolution(
                    fast_image_resize::FilterType::CatmullRom,
                ),
            ),
        )
        .map_err(|e| AppError::ImageError(e.to_string()))?;

    let buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, dst_image.into_vec())
        .ok_or_else(|| AppError::ImageError("Failed to create image buffer".to_string()))?;
    Ok(DynamicImage::ImageRgb8(buffer))
}

#[derive(Clone, Copy, Debug)]
pub struct TilingConfig {
    pub tile_size: u32,
    pub padding: u32,
    pub batch_size: usize,
}

// Tile metadata to track actual dimensions
#[derive(Clone, Copy, Debug)]
struct TileMetadata {
    x_index: u32,
    y_index: u32,
    content_width: u32,  // Actual content width (valid part of the tile)
    content_height: u32, // Actual content height (valid part of the tile)
}

pub fn process_tiled<F, I>(
    image: &DynamicImage,
    config: TilingConfig,
    scale: u32,
    cancel_flag: &Arc<AtomicBool>,
    mut progress_callback: F,
    inference_callback: I,
) -> AppResult<DynamicImage>
where
    F: FnMut(f32),
    I: Fn(Vec<DynamicImage>) -> AppResult<Vec<DynamicImage>>,
{
    let (width, height) = image.dimensions();
    let tile_size = config.tile_size;
    let batch_size = config.batch_size;
    let out_width = width * scale;
    let out_height = height * scale;

    // Memory check
    let required_memory = (out_width as u64 * out_height as u64 * 3) + (1024 * 1024 * 100);
    let mut sys = System::new_all();
    sys.refresh_memory();
    if sys.available_memory() < required_memory / 1024 {
        return Err(AppError::Unknown(format!(
            "Insufficient memory. Need ~{} MB, Available {} MB",
            required_memory / 1024 / 1024,
            sys.available_memory() / 1024
        )));
    }

    let mut output_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(out_width, out_height);
    let tiles_x = (width as f32 / tile_size as f32).ceil() as u32;
    let tiles_y = (height as f32 / tile_size as f32).ceil() as u32;
    let total_tiles = tiles_x * tiles_y;
    let mut processed_tiles = 0;

    // Force strict padding to ensure overlap blending works
    let padding = if total_tiles == 1 {
        0
    } else {
        config.padding.max(32)
    };

    std::thread::scope(|s| {
        // PERFORMANCE FIX: Increased channel size from 2 to 4.
        // This allows the CPU to prepare up to 4 batches ahead of the GPU.
        // For a batch size of 4, this means 16 tiles are "in flight", ensuring the GPU never waits.
        let (tx, rx) = std::sync::mpsc::sync_channel(4);

        // Producer: Extract tiles with metadata
        s.spawn(move || {
            let mut tile_batch: Vec<DynamicImage> = Vec::with_capacity(batch_size);
            let mut meta_batch: Vec<TileMetadata> = Vec::with_capacity(batch_size);

            for y in 0..tiles_y {
                for x in 0..tiles_x {
                    if cancel_flag.load(Ordering::Relaxed) {
                        return;
                    }

                    // Calculate actual content dimensions for this tile location
                    // This is only used for STITCHING later.
                    let tile_x_start = x * tile_size;
                    let tile_y_start = y * tile_size;
                    let valid_content_w = tile_size.min(width - tile_x_start);
                    let valid_content_h = tile_size.min(height - tile_y_start);

                    let metadata = TileMetadata {
                        x_index: x,
                        y_index: y,
                        content_width: valid_content_w,
                        content_height: valid_content_h,
                    };

                    // IMPORTANT: We request a tile of the FULL 'tile_size', even if we are at the edge.
                    // This ensures every single image sent to the model is exactly the same resolution (tile_size + 2*padding).
                    // This prevents the inference engine from stretching small edge tiles.
                    let tile = extract_tile_with_mirroring(
                        image,
                        tile_x_start,
                        tile_y_start,
                        tile_size, // Request full size
                        tile_size, // Request full size
                        padding,
                    );

                    tile_batch.push(tile);
                    meta_batch.push(metadata);

                    if tile_batch.len() >= batch_size {
                        if tx.send((tile_batch.clone(), meta_batch.clone())).is_err() {
                            return;
                        }
                        tile_batch.clear();
                        meta_batch.clear();
                    }
                }
            }

            if !tile_batch.is_empty() {
                let _ = tx.send((tile_batch, meta_batch));
            }
        });

        // Consumer: Run inference and stitch
        while let Ok((batch, metadata_batch)) = rx.recv() {
            if cancel_flag.load(Ordering::Relaxed) {
                return Err(AppError::Unknown("Operation cancelled".to_string()));
            }

            let upscaled_tiles = inference_callback(batch)?;

            if upscaled_tiles.len() != metadata_batch.len() {
                return Err(AppError::Unknown("Batch size mismatch".to_string()));
            }

            for (tile, meta) in upscaled_tiles.iter().zip(metadata_batch.iter()) {
                stitch_tile(&mut output_image, tile, meta, scale, padding, tile_size)?;
            }

            processed_tiles += metadata_batch.len();
            progress_callback(processed_tiles as f32 / total_tiles as f32);
        }
        Ok(())
    })?;

    Ok(DynamicImage::ImageRgb8(output_image))
}

// Helper to mirror coordinates (Reflect mode)
// Handles coordinates outside the image bounds by reflecting them back in.
fn mirror_coordinate(coord: i64, max: i64) -> u32 {
    let mut c = coord;
    // Reflect lower bound (negative coordinates)
    while c < 0 {
        c = -c;
    }
    // Reflect upper bound (past image width/height)
    // Uses ping-pong style reflection: 0 1 2 1 0
    while c >= max {
        c = 2 * (max - 1) - c;
    }
    // Final clamp just to be safe against edge cases like max=1
    c.clamp(0, max - 1) as u32
}

// Extract tile with proper padding using Mirroring
// Now accepts target_w/h to ensure output is always standard size
fn extract_tile_with_mirroring(
    image: &DynamicImage,
    x_start: u32,
    y_start: u32,
    target_w: u32,
    target_h: u32,
    padding: u32,
) -> DynamicImage {
    let (img_w, img_h) = image.dimensions();

    // The physical size of the tile image to be created
    let total_w = target_w + 2 * padding;
    let total_h = target_h + 2 * padding;

    let mut tile_buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(total_w, total_h);
    let rgb_image = image.to_rgb8();

    for tile_y in 0..total_h {
        for tile_x in 0..total_w {
            // Calculate the coordinate relative to the image source.
            // (x_start - padding) moves us to the top-left of the padded area.
            // + tile_x moves us through the pixels of the new tile.
            let src_x_ideal = (x_start as i64) + (tile_x as i64) - (padding as i64);
            let src_y_ideal = (y_start as i64) + (tile_y as i64) - (padding as i64);

            // Use mirror_coordinate to handle ANY out of bounds access.
            // This handles:
            // 1. Left/Top padding (negative coords)
            // 2. Right/Bottom padding (coords > img_w)
            // 3. "Filler" content if the tile is larger than the remaining image.
            let src_x = mirror_coordinate(src_x_ideal, img_w as i64);
            let src_y = mirror_coordinate(src_y_ideal, img_h as i64);

            // Safe get_pixel because mirror_coordinate guarantees bounds
            let pixel = rgb_image.get_pixel(src_x, src_y);
            tile_buffer.put_pixel(tile_x, tile_y, *pixel);
        }
    }

    DynamicImage::ImageRgb8(tile_buffer)
}

// Stitch upscaled tile into output
fn stitch_tile(
    output: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    upscaled_tile: &DynamicImage,
    meta: &TileMetadata,
    scale: u32,
    padding: u32,
    tile_size: u32,
) -> AppResult<()> {
    // We only want to paste the VALID part of the tile.
    // The tile might contain mirrored junk at the right/bottom edges if it was an edge tile.

    // 1. Skip the padding area
    let crop_x = padding * scale;
    let crop_y = padding * scale;

    // 2. Only take the valid content width/height
    let crop_w = meta.content_width * scale;
    let crop_h = meta.content_height * scale;

    let cropped = upscaled_tile
        .crop_imm(crop_x, crop_y, crop_w, crop_h)
        .to_rgb8();

    // Stitch into output at correct position
    let out_x = meta.x_index * tile_size * scale;
    let out_y = meta.y_index * tile_size * scale;

    // Fast row-based copy
    let dst_width = output.width();

    // Safety check: ensure we don't write past output bounds
    // (though logic should prevent this, robust code checks)
    let rows_to_copy = crop_h.min(output.height().saturating_sub(out_y));
    let cols_to_copy = crop_w.min(dst_width.saturating_sub(out_x));

    for y in 0..rows_to_copy {
        let src_row = &cropped.as_raw()
            [(y * crop_w * 3) as usize..((y * crop_w + cols_to_copy) * 3) as usize];
        let dst_idx = ((out_y + y) * dst_width * 3 + out_x * 3) as usize;
        let dst_row = &mut output.as_mut()[dst_idx..dst_idx + (cols_to_copy * 3) as usize];
        dst_row.copy_from_slice(src_row);
    }

    Ok(())
}
