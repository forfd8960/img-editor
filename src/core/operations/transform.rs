use image::DynamicImage;

/// Rotate image 90 degrees clockwise
pub fn rotate90(img: &DynamicImage) -> DynamicImage {
    img.rotate90()
}

/// Rotate image 180 degrees
pub fn rotate180(img: &DynamicImage) -> DynamicImage {
    img.rotate180()
}

/// Rotate image 270 degrees clockwise (90 degrees counter-clockwise)
pub fn rotate270(img: &DynamicImage) -> DynamicImage {
    img.rotate270()
}

/// Flip image horizontally (left to right)
pub fn flip_horizontal(img: &DynamicImage) -> DynamicImage {
    img.fliph()
}

/// Flip image vertically (top to bottom)
pub fn flip_vertical(img: &DynamicImage) -> DynamicImage {
    img.flipv()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    #[test]
    fn test_rotate90() {
        // Create a 2x3 test image
        let img = DynamicImage::ImageRgb8(
            ImageBuffer::from_fn(2, 3, |x, y| {
                Rgb([(x * 10) as u8, (y * 10) as u8, 0])
            })
        );
        
        let rotated = rotate90(&img);
        assert_eq!(rotated.width(), 3);
        assert_eq!(rotated.height(), 2);
    }
    
    #[test]
    fn test_rotate180() {
        let img = DynamicImage::ImageRgb8(
            ImageBuffer::from_fn(2, 2, |x, y| {
                Rgb([x as u8, y as u8, 0])
            })
        );
        
        let rotated = rotate180(&img);
        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 2);
    }
    
    #[test]
    fn test_flip_horizontal() {
        let img = DynamicImage::ImageRgb8(
            ImageBuffer::from_fn(3, 2, |_, _| Rgb([255, 0, 0]))
        );
        
        let flipped = flip_horizontal(&img);
        assert_eq!(flipped.width(), 3);
        assert_eq!(flipped.height(), 2);
    }
    
    #[test]
    fn test_flip_vertical() {
        let img = DynamicImage::ImageRgb8(
            ImageBuffer::from_fn(3, 2, |_, _| Rgb([0, 255, 0]))
        );
        
        let flipped = flip_vertical(&img);
        assert_eq!(flipped.width(), 3);
        assert_eq!(flipped.height(), 2);
    }
}
