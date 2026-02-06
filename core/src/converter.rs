use image::{GenericImageView, ImageFormat as ImgFormat, DynamicImage};
use std::io::Cursor;

use crate::config::ProcessingConfig;
use crate::error::ProcessingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertFormat {
    Png,
    Jpg,
    Webp,
}

impl ConvertFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "png" => Some(ConvertFormat::Png),
            "jpg" | "jpeg" => Some(ConvertFormat::Jpg),
            "webp" => Some(ConvertFormat::Webp),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            ConvertFormat::Png => "png",
            ConvertFormat::Jpg => "jpg",
            ConvertFormat::Webp => "webp",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ConvertFormat::Png => "PNG",
            ConvertFormat::Jpg => "JPEG",
            ConvertFormat::Webp => "WebP",
        }
    }
}

/// Convert image from one format to another
pub fn convert_image(
    input: &[u8],
    target_format: ConvertFormat,
    config: &ProcessingConfig,
) -> Result<Vec<u8>, ProcessingError> {
    // Load image (supports PNG, JPG, WebP automatically)
    let img = image::load_from_memory(input)
        .map_err(|e| ProcessingError::Decode(format!("Failed to load image: {}", e)))?;

    log::debug!(
        "Converting image: {}x{} pixels to {}",
        img.width(),
        img.height(),
        target_format.as_str()
    );

    // Convert based on target format
    let output = match target_format {
        ConvertFormat::Png => convert_to_png(&img, config)?,
        ConvertFormat::Jpg => convert_to_jpg(&img, config)?,
        ConvertFormat::Webp => convert_to_webp(&img, config)?,
    };

    log::debug!(
        "Conversion complete: {} bytes ({})",
        output.len(),
        target_format.as_str()
    );

    Ok(output)
}

/// Convert to PNG format
fn convert_to_png(img: &DynamicImage, config: &ProcessingConfig) -> Result<Vec<u8>, ProcessingError> {
    let mut output = Vec::new();
    let mut cursor = Cursor::new(&mut output);

    if config.no_lossy {
        // Lossless PNG
        img.write_to(&mut cursor, ImgFormat::Png)
            .map_err(|e| ProcessingError::Encode(format!("Failed to encode PNG: {}", e)))?;
    } else {
        // For lossy PNG, we could use imagequant here
        // For now, just save as regular PNG and let PNG processor optimize it later
        img.write_to(&mut cursor, ImgFormat::Png)
            .map_err(|e| ProcessingError::Encode(format!("Failed to encode PNG: {}", e)))?;
    }

    Ok(output)
}

/// Convert to JPEG format
fn convert_to_jpg(img: &DynamicImage, config: &ProcessingConfig) -> Result<Vec<u8>, ProcessingError> {
    let mut output = Vec::new();
    let mut cursor = Cursor::new(&mut output);

    // Convert to RGB (JPEG doesn't support alpha)
    let rgb_img = img.to_rgb8();

    // Create JPEG encoder with quality
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
        &mut cursor,
        config.quality,
    );

    encoder
        .encode(
            rgb_img.as_raw(),
            rgb_img.width(),
            rgb_img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| ProcessingError::Encode(format!("Failed to encode JPEG: {}", e)))?;

    Ok(output)
}

/// Convert to WebP format
fn convert_to_webp(img: &DynamicImage, config: &ProcessingConfig) -> Result<Vec<u8>, ProcessingError> {
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();

    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), width, height);

    let encoded = if config.no_lossy {
        encoder.encode_lossless()
    } else {
        encoder.encode(config.quality as f32)
    };

    Ok(encoded.to_vec())
}
