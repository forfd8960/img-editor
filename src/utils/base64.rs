use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

use crate::types::errors::AppError;

/// Encode image to Base64 string (PNG format)
pub fn encode_image(img: &DynamicImage) -> Result<String, AppError> {
    let mut buffer = Vec::new();
    
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(|e| AppError::ProcessingError {
            details: format!("Failed to encode image: {}", e),
        })?;

    Ok(format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(&buffer)
    ))
}

/// Encode image to Base64 JPEG with quality
#[allow(dead_code)]
pub fn encode_image_as_jpeg(img: &DynamicImage, quality: u8) -> Result<String, AppError> {
    let mut buffer = Vec::new();
    
    let rgb_image = img.to_rgb8();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, quality);
    
    encoder
        .encode(
            rgb_image.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| AppError::ProcessingError {
            details: format!("Failed to encode JPEG: {}", e),
        })?;

    Ok(format!(
        "data:image/jpeg;base64,{}",
        general_purpose::STANDARD.encode(&buffer)
    ))
}

/// Decode Base64 string to image
#[allow(dead_code)]
pub fn decode_image(base64_str: &str) -> Result<DynamicImage, AppError> {
    // Strip data URL prefix if present
    let base64_data = if let Some(comma_pos) = base64_str.find(',') {
        &base64_str[comma_pos + 1..]
    } else {
        base64_str
    };

    let bytes = general_purpose::STANDARD
        .decode(base64_data)
        .map_err(|e| AppError::ProcessingError {
            details: format!("Failed to decode base64: {}", e),
        })?;

    image::load_from_memory(&bytes).map_err(|e| AppError::ProcessingError {
        details: format!("Failed to load image from bytes: {}", e),
    })
}
