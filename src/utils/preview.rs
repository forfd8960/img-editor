use image::{DynamicImage, imageops::FilterType, GenericImageView};
use std::path::Path;

use crate::types::errors::AppError;

/// Resize image to fit within max dimensions
pub fn resize_to_fit(img: &DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    // Calculate if resize is needed
    if width <= max_width && height <= max_height {
        return img.clone();
    }

    // Calculate new dimensions maintaining aspect ratio
    let width_ratio = max_width as f64 / width as f64;
    let height_ratio = max_height as f64 / height as f64;
    let ratio = width_ratio.min(height_ratio);

    let new_width = (width as f64 * ratio) as u32;
    let new_height = (height as f64 * ratio) as u32;

    img.resize(new_width, new_height, FilterType::Lanczos3)
}

/// Validate image dimensions
#[allow(dead_code)]
pub fn validate_dimensions(width: u32, height: u32) -> Result<(), String> {
    const MAX_DIMENSION: u32 = 16384;

    if width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(format!(
            "Image dimensions {}x{} exceed maximum {}x{}",
            width, height, MAX_DIMENSION, MAX_DIMENSION
        ));
    }

    if width == 0 || height == 0 {
        return Err("Image dimensions must be greater than 0".to_string());
    }

    Ok(())
}

/// Validate file size (max 100MB)
pub fn validate_file_size<P: AsRef<Path>>(path: P) -> Result<(), AppError> {
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB

    let metadata = std::fs::metadata(path.as_ref())
        .map_err(|e| AppError::ImageLoadError(format!("Cannot read file metadata: {}", e)))?;

    let size = metadata.len();
    if size > MAX_FILE_SIZE {
        return Err(AppError::ImageLoadError(format!(
            "File size {} MB exceeds maximum {} MB",
            size / 1024 / 1024,
            MAX_FILE_SIZE / 1024 / 1024
        )));
    }

    Ok(())
}

/// Validate supported image format
pub fn validate_format<P: AsRef<Path>>(path: P) -> Result<(), AppError> {
    let extension = path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("bmp") | Some("webp") | Some("tiff") | Some("tif") => Ok(()),
        Some(ext) => Err(AppError::UnsupportedFormat { format: ext.to_string() }),
        None => Err(AppError::UnsupportedFormat { format: "unknown".to_string() }),
    }
}
