use image::{DynamicImage, GenericImageView};
use crate::types::errors::AppError;
use crate::types::operations::CropRect;

/// Crop an image to a rectangular region
/// 
/// # Parameters
/// - `img`: Input image
/// - `rect`: Crop rectangle (x, y, width, height)
/// 
/// # Returns
/// Cropped image or error if rectangle is invalid
pub fn crop_image(img: &DynamicImage, rect: &CropRect) -> Result<DynamicImage, AppError> {
    let (img_width, img_height) = img.dimensions();
    
    // Validate crop rectangle is within image bounds
    if rect.x >= img_width || rect.y >= img_height {
        return Err(AppError::InvalidOperation {
            details: format!(
                "Crop position ({}, {}) is outside image bounds ({}x{})",
                rect.x, rect.y, img_width, img_height
            ),
        });
    }
    
    // Validate minimum dimensions
    if rect.width == 0 || rect.height == 0 {
        return Err(AppError::InvalidOperation {
            details: "Crop dimensions must be at least 1x1".to_string(),
        });
    }
    
    // Calculate actual crop dimensions (clamp to image bounds)
    let actual_width = rect.width.min(img_width - rect.x);
    let actual_height = rect.height.min(img_height - rect.y);
    
    if actual_width == 0 || actual_height == 0 {
        return Err(AppError::InvalidOperation {
            details: "Calculated crop area is empty".to_string(),
        });
    }
    
    // Perform crop
    let cropped = img.crop_imm(rect.x, rect.y, actual_width, actual_height);
    
    Ok(cropped)
}

/// Calculate crop rectangle maintaining aspect ratio
/// 
/// # Parameters
/// - `img_width`: Original image width
/// - `img_height`: Original image height
/// - `desired_aspect`: Desired aspect ratio (width/height)
/// - `from_center`: If true, crop from center; otherwise from top-left
/// 
/// # Returns
/// CropRect that maintains the desired aspect ratio
#[allow(dead_code)]
pub fn crop_with_aspect_ratio(
    img_width: u32,
    img_height: u32,
    desired_aspect: f32,
    from_center: bool,
) -> CropRect {
    let img_aspect = img_width as f32 / img_height as f32;
    
    let (crop_width, crop_height) = if img_aspect > desired_aspect {
        // Image is wider than desired, crop width
        let new_width = (img_height as f32 * desired_aspect) as u32;
        (new_width, img_height)
    } else {
        // Image is taller than desired, crop height
        let new_height = (img_width as f32 / desired_aspect) as u32;
        (img_width, new_height)
    };
    
    let (x, y) = if from_center {
        // Center the crop
        let x = (img_width - crop_width) / 2;
        let y = (img_height - crop_height) / 2;
        (x, y)
    } else {
        // Crop from top-left
        (0, 0)
    };
    
    CropRect {
        x,
        y,
        width: crop_width,
        height: crop_height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crop_image() {
        let img = DynamicImage::new_rgb8(100, 100);
        let rect = CropRect {
            x: 10,
            y: 10,
            width: 50,
            height: 50,
        };
        
        let result = crop_image(&img, &rect);
        assert!(result.is_ok());
        
        let cropped = result.unwrap();
        assert_eq!(cropped.width(), 50);
        assert_eq!(cropped.height(), 50);
    }
    
    #[test]
    fn test_crop_out_of_bounds() {
        let img = DynamicImage::new_rgb8(100, 100);
        let rect = CropRect {
            x: 150,
            y: 10,
            width: 50,
            height: 50,
        };
        
        let result = crop_image(&img, &rect);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_crop_zero_dimensions() {
        let img = DynamicImage::new_rgb8(100, 100);
        let rect = CropRect {
            x: 10,
            y: 10,
            width: 0,
            height: 50,
        };
        
        let result = crop_image(&img, &rect);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_crop_with_aspect_ratio_wider() {
        // 16:9 aspect ratio on a square image
        let rect = crop_with_aspect_ratio(100, 100, 16.0 / 9.0, true);
        
        // Should crop height to maintain 16:9
        assert_eq!(rect.width, 100);
        assert!(rect.height < 100);
        
        // Should be centered
        assert!(rect.y > 0);
    }
    
    #[test]
    fn test_crop_with_aspect_ratio_taller() {
        // 1:2 aspect ratio on a square image
        let rect = crop_with_aspect_ratio(100, 100, 0.5, true);
        
        // Should crop width to maintain 1:2
        assert!(rect.width < 100);
        assert_eq!(rect.height, 100);
        
        // Should be centered
        assert!(rect.x > 0);
    }
}
