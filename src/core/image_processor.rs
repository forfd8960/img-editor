use image::DynamicImage;

use crate::types::errors::AppError;
use crate::types::operations::{AdjustmentParams, CropRect, FilterType, OperationType, TransformType};
use crate::core::operations::{adjustments, crop, filters, transform};

/// Image processor for applying operations
pub struct ImageProcessor;

impl ImageProcessor {
    /// Apply a filter to an image
    pub fn apply_filter(img: &DynamicImage, filter: &FilterType) -> Result<DynamicImage, AppError> {
        let result = match filter {
            FilterType::Grayscale => filters::grayscale(img),
            FilterType::Sepia => filters::sepia(img),
            FilterType::Invert => filters::invert(img),
            FilterType::Blur { radius } => {
                if *radius <= 0.0 || *radius > 100.0 {
                    return Err(AppError::InvalidOperation {
                        details: format!("Blur radius must be between 0 and 100, got {}", radius),
                    });
                }
                filters::blur(img, *radius)
            }
            FilterType::Sharpen => filters::sharpen(img),
        };
        
        Ok(result)
    }
    
    /// Apply an adjustment to an image
    pub fn apply_adjustment(img: &DynamicImage, params: &AdjustmentParams) -> Result<DynamicImage, AppError> {
        let mut result = img.clone();
        
        // Apply adjustments in sequence if multiple are specified
        if let Some(brightness_val) = params.brightness {
            if brightness_val < 0.0 || brightness_val > 2.0 {
                return Err(AppError::InvalidOperation {
                    details: format!("Brightness must be between 0.0 and 2.0, got {}", brightness_val),
                });
            }
            result = adjustments::brightness(&result, brightness_val);
        }
        
        if let Some(contrast_val) = params.contrast {
            if contrast_val < 0.0 || contrast_val > 2.0 {
                return Err(AppError::InvalidOperation {
                    details: format!("Contrast must be between 0.0 and 2.0, got {}", contrast_val),
                });
            }
            result = adjustments::contrast(&result, contrast_val);
        }
        
        if let Some(saturation_val) = params.saturation {
            if saturation_val < 0.0 || saturation_val > 2.0 {
                return Err(AppError::InvalidOperation {
                    details: format!("Saturation must be between 0.0 and 2.0, got {}", saturation_val),
                });
            }
            result = adjustments::saturation(&result, saturation_val);
        }
        
        if let Some(hue_val) = params.hue {
            if hue_val < -180 || hue_val > 180 {
                return Err(AppError::InvalidOperation {
                    details: format!("Hue must be between -180 and 180, got {}", hue_val),
                });
            }
            result = adjustments::hue(&result, hue_val);
        }
        
        if let Some(gamma_val) = params.gamma {
            if gamma_val < 0.1 || gamma_val > 3.0 {
                return Err(AppError::InvalidOperation {
                    details: format!("Gamma must be between 0.1 and 3.0, got {}", gamma_val),
                });
            }
            result = adjustments::gamma(&result, gamma_val);
        }
        
        Ok(result)
    }
    
    /// Apply a transform to an image
    pub fn apply_transform(img: &DynamicImage, transform: &TransformType) -> Result<DynamicImage, AppError> {
        let result = match transform {
            TransformType::Rotate90 => transform::rotate90(img),
            TransformType::Rotate180 => transform::rotate180(img),
            TransformType::Rotate270 => transform::rotate270(img),
            TransformType::FlipHorizontal => transform::flip_horizontal(img),
            TransformType::FlipVertical => transform::flip_vertical(img),
        };
        
        Ok(result)
    }
    
    /// Apply a crop to an image
    pub fn apply_crop(img: &DynamicImage, rect: &CropRect) -> Result<DynamicImage, AppError> {
        crop::crop_image(img, rect)
    }
    
    /// Apply an operation to an image
    pub fn apply_operation(img: &DynamicImage, operation_type: &OperationType) -> Result<DynamicImage, AppError> {
        match operation_type {
            OperationType::Filter(filter) => Self::apply_filter(img, filter),
            OperationType::Adjustment(params) => Self::apply_adjustment(img, params),
            OperationType::Transform(transform) => Self::apply_transform(img, transform),
            OperationType::Crop(rect) => Self::apply_crop(img, rect),
        }
    }
    
    /// Apply multiple operations in sequence
    #[allow(dead_code)]
    pub fn apply_operations(
        img: &DynamicImage,
        operations: &[OperationType],
    ) -> Result<DynamicImage, AppError> {
        let mut current = img.clone();
        
        for operation in operations {
            current = Self::apply_operation(&current, operation)?;
        }
        
        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_apply_grayscale() {
        let img = DynamicImage::new_rgb8(10, 10);
        let result = ImageProcessor::apply_filter(&img, &FilterType::Grayscale);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_blur_validation() {
        let img = DynamicImage::new_rgb8(10, 10);
        let result = ImageProcessor::apply_filter(&img, &FilterType::Blur { radius: -1.0 });
        assert!(result.is_err());
    }
}
