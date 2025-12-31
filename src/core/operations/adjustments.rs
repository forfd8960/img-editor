use image::{DynamicImage, GenericImageView, ImageBuffer};
use rayon::prelude::*;

/// Adjust brightness of an image
/// 
/// # Parameters
/// - `img`: Input image
/// - `factor`: Brightness multiplier (0.0 = black, 1.0 = unchanged, 2.0 = double brightness)
/// 
/// # Returns
/// New image with adjusted brightness
pub fn brightness(img: &DynamicImage, factor: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let rgb_img = img.to_rgb8();
    let pixels: Vec<u8> = rgb_img.into_raw();
    
    let adjusted: Vec<u8> = pixels
        .par_chunks(3)
        .flat_map(|pixel| {
            [
                (pixel[0] as f32 * factor).clamp(0.0, 255.0) as u8,
                (pixel[1] as f32 * factor).clamp(0.0, 255.0) as u8,
                (pixel[2] as f32 * factor).clamp(0.0, 255.0) as u8,
            ]
        })
        .collect();
    
    DynamicImage::ImageRgb8(
        ImageBuffer::from_raw(width, height, adjusted)
            .expect("Failed to create image buffer")
    )
}

/// Adjust contrast of an image
/// 
/// # Parameters
/// - `img`: Input image
/// - `factor`: Contrast multiplier (0.0 = gray, 1.0 = unchanged, 2.0 = double contrast)
/// 
/// # Returns
/// New image with adjusted contrast
pub fn contrast(img: &DynamicImage, factor: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let rgb_img = img.to_rgb8();
    let pixels: Vec<u8> = rgb_img.into_raw();
    
    // Contrast adjustment around middle gray (128)
    let adjusted: Vec<u8> = pixels
        .par_chunks(3)
        .flat_map(|pixel| {
            [
                ((pixel[0] as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8,
                ((pixel[1] as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8,
                ((pixel[2] as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8,
            ]
        })
        .collect();
    
    DynamicImage::ImageRgb8(
        ImageBuffer::from_raw(width, height, adjusted)
            .expect("Failed to create image buffer")
    )
}

/// Convert RGB to HSL color space
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    
    // Lightness
    let l = (max + min) / 2.0;
    
    // Saturation
    let s = if delta == 0.0 {
        0.0
    } else if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };
    
    // Hue
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    
    (h, s, l)
}

/// Convert HSL to RGB color space
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

/// Adjust saturation of an image
/// 
/// # Parameters
/// - `img`: Input image
/// - `factor`: Saturation multiplier (0.0 = grayscale, 1.0 = unchanged, 2.0 = double saturation)
/// 
/// # Returns
/// New image with adjusted saturation
pub fn saturation(img: &DynamicImage, factor: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let rgb_img = img.to_rgb8();
    let pixels: Vec<u8> = rgb_img.into_raw();
    
    let adjusted: Vec<u8> = pixels
        .par_chunks(3)
        .flat_map(|pixel| {
            let (h, s, l) = rgb_to_hsl(pixel[0], pixel[1], pixel[2]);
            let new_s = (s * factor).clamp(0.0, 1.0);
            let (r, g, b) = hsl_to_rgb(h, new_s, l);
            [r, g, b]
        })
        .collect();
    
    DynamicImage::ImageRgb8(
        ImageBuffer::from_raw(width, height, adjusted)
            .expect("Failed to create image buffer")
    )
}

/// Adjust hue of an image
/// 
/// # Parameters
/// - `img`: Input image
/// - `shift`: Hue shift in degrees (-180 to 180)
/// 
/// # Returns
/// New image with adjusted hue
pub fn hue(img: &DynamicImage, shift: i32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let rgb_img = img.to_rgb8();
    let pixels: Vec<u8> = rgb_img.into_raw();
    
    let adjusted: Vec<u8> = pixels
        .par_chunks(3)
        .flat_map(|pixel| {
            let (h, s, l) = rgb_to_hsl(pixel[0], pixel[1], pixel[2]);
            let new_h = (h + shift as f32 + 360.0) % 360.0;
            let (r, g, b) = hsl_to_rgb(new_h, s, l);
            [r, g, b]
        })
        .collect();
    
    DynamicImage::ImageRgb8(
        ImageBuffer::from_raw(width, height, adjusted)
            .expect("Failed to create image buffer")
    )
}

/// Apply gamma correction to an image
/// 
/// # Parameters
/// - `img`: Input image
/// - `gamma`: Gamma value (0.1-3.0, where 1.0 = unchanged, <1.0 = darker, >1.0 = brighter)
/// 
/// # Returns
/// New image with gamma correction applied
pub fn gamma(img: &DynamicImage, gamma: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let rgb_img = img.to_rgb8();
    let pixels: Vec<u8> = rgb_img.into_raw();
    
    // Precompute gamma lookup table for performance
    let gamma_lut: Vec<u8> = (0..256)
        .map(|i| {
            let normalized = i as f32 / 255.0;
            let corrected = normalized.powf(1.0 / gamma);
            (corrected * 255.0).clamp(0.0, 255.0) as u8
        })
        .collect();
    
    let adjusted: Vec<u8> = pixels
        .par_iter()
        .map(|&pixel| gamma_lut[pixel as usize])
        .collect();
    
    DynamicImage::ImageRgb8(
        ImageBuffer::from_raw(width, height, adjusted)
            .expect("Failed to create image buffer")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rgb_to_hsl_conversion() {
        // Test pure red
        let (h, s, l) = rgb_to_hsl(255, 0, 0);
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((l - 0.5).abs() < 0.01);
        
        // Test gray
        let (_h, s, l) = rgb_to_hsl(128, 128, 128);
        assert!((s - 0.0).abs() < 0.01);
        assert!((l - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_hsl_to_rgb_conversion() {
        // Test pure red
        let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        
        // Test gray
        let (r, g, b) = hsl_to_rgb(0.0, 0.0, 0.5);
        assert!((r as i32 - 128).abs() <= 1);
        assert!((g as i32 - 128).abs() <= 1);
        assert!((b as i32 - 128).abs() <= 1);
    }
}
