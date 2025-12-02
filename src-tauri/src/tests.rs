#[cfg(test)]
mod tests {
    use crate::image_processing;

    use image::{DynamicImage, ImageBuffer, Rgba};

    fn create_dummy_image(width: u32, height: u32) -> DynamicImage {
        let buffer = ImageBuffer::from_fn(width, height, |x, y| {
            Rgba([((x % 255) as u8), ((y % 255) as u8), 0, 255])
        });
        DynamicImage::ImageRgba8(buffer)
    }

    #[test]
    fn test_resize_image() {
        let src = create_dummy_image(100, 100);
        let resized = image_processing::resize_image(&src, 50, 50).expect("Failed to resize image");

        assert_eq!(resized.width(), 50);
        assert_eq!(resized.height(), 50);
    }
}
