use image::{DynamicImage, ImageEncoder};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{PngEncoder, CompressionType};
use image::codecs::webp::WebPEncoder;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use crate::types::errors::AppError;

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Jpeg,
    Png,
    WebP,
}

impl ExportFormat {
    /// Parse format from file extension
    pub fn from_extension(ext: &str) -> Result<Self, AppError> {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(ExportFormat::Jpeg),
            "png" => Ok(ExportFormat::Png),
            "webp" => Ok(ExportFormat::WebP),
            _ => Err(AppError::UnsupportedFormat {
                format: format!("{}. Supported: jpeg, png, webp", ext),
            }),
        }
    }
}

/// Export an image to a file with the specified format and quality
///
/// # Arguments
/// * `image` - The image to export
/// * `path` - Output file path
/// * `format` - Export format (JPEG, PNG, WebP)
/// * `quality` - Quality parameter (1-100, used for JPEG and WebP)
///
/// # Returns
/// File size in bytes on success
pub fn export_image(
    image: &DynamicImage,
    path: &Path,
    format: ExportFormat,
    quality: u8,
) -> Result<u64, AppError> {
    // Validate quality
    if quality == 0 || quality > 100 {
        return Err(AppError::InvalidOperation {
            details: format!("Quality must be 1-100, got {}", quality),
        });
    }

    // Validate path is writable
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(AppError::InvalidOperation {
                details: format!("Parent directory does not exist: {:?}", parent),
            });
        }
    }

    // Export based on format
    match format {
        ExportFormat::Jpeg => export_jpeg(image, path, quality)?,
        ExportFormat::Png => export_png(image, path)?,
        ExportFormat::WebP => export_webp(image, path, quality)?,
    }

    // Get file size
    let metadata = std::fs::metadata(path)
        .map_err(|e| AppError::ImageSaveError(e.to_string()))?;

    Ok(metadata.len())
}

/// Export image as JPEG with quality setting
fn export_jpeg(image: &DynamicImage, path: &Path, quality: u8) -> Result<(), AppError> {
    let file = File::create(path)
        .map_err(|e| AppError::ImageSaveError(e.to_string()))?;
    let writer = BufWriter::new(file);

    let rgb_image = image.to_rgb8();
    let encoder = JpegEncoder::new_with_quality(writer, quality);
    
    encoder
        .write_image(
            rgb_image.as_raw(),
            rgb_image.width(),
            rgb_image.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e: image::ImageError| AppError::ProcessingError { details: e.to_string() })?;

    Ok(())
}

/// Export image as PNG with compression
fn export_png(image: &DynamicImage, path: &Path) -> Result<(), AppError> {
    let file = File::create(path)
        .map_err(|e| AppError::ImageSaveError(e.to_string()))?;
    let writer = BufWriter::new(file);

    let rgba_image = image.to_rgba8();
    let encoder = PngEncoder::new_with_quality(
        writer,
        CompressionType::Best,
        image::codecs::png::FilterType::Adaptive,
    );
    
    encoder
        .write_image(
            rgba_image.as_raw(),
            rgba_image.width(),
            rgba_image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| AppError::ProcessingError { details: e.to_string() })?;

    Ok(())
}

/// Export image as WebP (lossless - quality parameter ignored for now)
fn export_webp(image: &DynamicImage, path: &Path, _quality: u8) -> Result<(), AppError> {
    let file = File::create(path)
        .map_err(|e| AppError::ImageSaveError(e.to_string()))?;
    let writer = BufWriter::new(file);

    let rgba_image = image.to_rgba8();
    // Note: image crate 0.25 only supports lossless WebP encoding
    let encoder = WebPEncoder::new_lossless(writer);
    
    encoder
        .write_image(
            rgba_image.as_raw(),
            rgba_image.width(),
            rgba_image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e: image::ImageError| AppError::ProcessingError { details: e.to_string() })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_export_format_from_extension() {
        assert_eq!(ExportFormat::from_extension("jpg").unwrap(), ExportFormat::Jpeg);
        assert_eq!(ExportFormat::from_extension("jpeg").unwrap(), ExportFormat::Jpeg);
        assert_eq!(ExportFormat::from_extension("JPG").unwrap(), ExportFormat::Jpeg);
        assert_eq!(ExportFormat::from_extension("png").unwrap(), ExportFormat::Png);
        assert_eq!(ExportFormat::from_extension("webp").unwrap(), ExportFormat::WebP);
        assert!(ExportFormat::from_extension("gif").is_err());
    }

    #[test]
    fn test_export_jpeg() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.jpg");
        
        let image = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
        let size = export_image(&image, &path, ExportFormat::Jpeg, 95).unwrap();
        
        assert!(path.exists());
        assert!(size > 0);
    }

    #[test]
    fn test_export_png() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.png");
        
        let image = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
        let size = export_image(&image, &path, ExportFormat::Png, 100).unwrap();
        
        assert!(path.exists());
        assert!(size > 0);
    }

    #[test]
    fn test_export_quality_validation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.jpg");
        let image = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
        
        // Quality 0 should fail
        assert!(export_image(&image, &path, ExportFormat::Jpeg, 0).is_err());
        
        // Quality 101 should fail
        assert!(export_image(&image, &path, ExportFormat::Jpeg, 101).is_err());
        
        // Quality 1 should work
        assert!(export_image(&image, &path, ExportFormat::Jpeg, 1).is_ok());
        
        // Quality 100 should work
        let _ = fs::remove_file(&path);
        assert!(export_image(&image, &path, ExportFormat::Jpeg, 100).is_ok());
    }
}
