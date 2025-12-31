use image::{DynamicImage, GenericImageView, Rgba, imageops};
use rayon::prelude::*;

/// Apply grayscale filter
pub fn grayscale(img: &DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(img.to_luma8())
}

/// Apply sepia filter
pub fn sepia(img: &DynamicImage) -> DynamicImage {
    let mut output = img.to_rgba8();
    
    output.par_chunks_mut(4).for_each(|pixel| {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        
        // Sepia tone matrix
        let tr = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0);
        let tg = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0);
        let tb = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0);
        
        pixel[0] = tr as u8;
        pixel[1] = tg as u8;
        pixel[2] = tb as u8;
    });
    
    DynamicImage::ImageRgba8(output)
}

/// Apply invert filter
pub fn invert(img: &DynamicImage) -> DynamicImage {
    let mut output = img.to_rgba8();
    
    output.par_chunks_mut(4).for_each(|pixel| {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
        // Keep alpha unchanged
    });
    
    DynamicImage::ImageRgba8(output)
}

/// Apply Gaussian blur filter
pub fn blur(img: &DynamicImage, radius: f32) -> DynamicImage {
    DynamicImage::ImageRgba8(imageops::blur(&img.to_rgba8(), radius))
}

/// Apply sharpen filter
pub fn sharpen(img: &DynamicImage) -> DynamicImage {
    // Use unsharp mask for sharpening
    let blurred = imageops::blur(&img.to_rgba8(), 1.0);
    let (width, height) = img.dimensions();
    let original = img.to_rgba8();
    let mut output = image::RgbaImage::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let orig = original.get_pixel(x, y);
            let blur_px = blurred.get_pixel(x, y);
            
            let r = ((orig[0] as i32 * 2 - blur_px[0] as i32).clamp(0, 255)) as u8;
            let g = ((orig[1] as i32 * 2 - blur_px[1] as i32).clamp(0, 255)) as u8;
            let b = ((orig[2] as i32 * 2 - blur_px[2] as i32).clamp(0, 255)) as u8;
            
            output.put_pixel(x, y, Rgba([r, g, b, orig[3]]));
        }
    }
    
    DynamicImage::ImageRgba8(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grayscale() {
        let img = DynamicImage::new_rgb8(10, 10);
        let result = grayscale(&img);
        assert_eq!(result.dimensions(), (10, 10));
    }
    
    #[test]
    fn test_invert() {
        let img = DynamicImage::new_rgb8(10, 10);
        let result = invert(&img);
        assert_eq!(result.dimensions(), (10, 10));
    }
}
