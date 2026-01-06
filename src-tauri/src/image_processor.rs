use anyhow::{Context, Result};
use image::{ImageBuffer, Luma, Rgb, RgbImage, ImageEncoder};
use ndarray::Array4;

pub struct ImageProcessor;

impl ImageProcessor {
    /// Load image and convert to model input format (RGB for color models)
    pub fn load_and_preprocess(image_path: &str) -> Result<Array4<f32>> {
        log::debug!("Loading image: {}", image_path);
        
        // Load image and convert to RGB
        let img = image::open(image_path)
            .with_context(|| format!("Failed to load image: {}", image_path))?
            .to_rgb8();
        
        let (width, height) = img.dimensions();
        log::debug!("Original dimensions: {}x{}", width, height);
        
        // Convert to normalized float array [1, 3, H, W] for RGB
        let mut tensor = Array4::zeros((1, 3, height as usize, width as usize));
        
        for (x, y, pixel) in img.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            tensor[[0, 0, y as usize, x as usize]] = r;
            tensor[[0, 1, y as usize, x as usize]] = g;
            tensor[[0, 2, y as usize, x as usize]] = b;
        }
        
        Ok(tensor)
    }
    
    /// Convert model output back to image (RGB)
    pub fn postprocess_to_image(output: &Array4<f32>) -> Result<RgbImage> {
        log::debug!("Output shape: {:?}", output.shape());
        
        let (_, channels, height, width) = (
            output.shape()[0],
            output.shape()[1],
            output.shape()[2],
            output.shape()[3],
        );
        
        // Create new RGB image
        let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
        
        // Handle both grayscale (1 channel) and RGB (3 channels) outputs
        if channels == 1 {
            // Grayscale output - convert to RGB
            for y in 0..height {
                for x in 0..width {
                    let gray = (output[[0, 0, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
                    img.put_pixel(x as u32, y as u32, Rgb([gray, gray, gray]));
                }
            }
        } else {
            // RGB output
            for y in 0..height {
                for x in 0..width {
                    let r = (output[[0, 0, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
                    let g = (output[[0, 1, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
                    let b = (output[[0, 2, y, x]].clamp(0.0, 1.0) * 255.0) as u8;
                    
                    img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                }
            }
        }
        
        Ok(img)
    }
    
    /// Save image to file
    pub fn save_image(img: &ImageBuffer<Luma<u8>, Vec<u8>>, output_path: &str) -> Result<()> {
        img.save(output_path)
            .with_context(|| format!("Failed to save image: {}", output_path))?;
        
        let (width, height) = img.dimensions();
        log::info!("Saved upscaled image: {} ({}x{})", output_path, width, height);
        Ok(())
    }
    
    /// Save image with specified format and quality
    pub fn save_image_with_quality(
        img: &RgbImage,
        output_path: &str,
        format: &str,
        quality: u8,
    ) -> Result<()> {
        match format.to_lowercase().as_str() {
            "jpg" | "jpeg" => {
                let file = std::fs::File::create(output_path)?;
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(file, quality);
                encoder.write_image(
                    img.as_raw(),
                    img.width(),
                    img.height(),
                    image::ExtendedColorType::Rgb8,
                )?;
            }
            "webp" => {
                img.save_with_format(output_path, image::ImageFormat::WebP)?;
            }
            "png" | _ => {
                img.save(output_path)?;
            }
        }
        
        let (width, height) = img.dimensions();
        log::debug!("Saved {}x{} image to: {} (format: {}, quality: {})", 
            width, height, output_path, format, quality);
        Ok(())
    }
}
