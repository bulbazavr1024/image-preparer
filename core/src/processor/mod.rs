pub mod png;
pub mod mp3;
pub mod webp;
pub mod mp4;

use crate::config::ProcessingConfig;
use crate::error::ProcessingError;
use crate::format::ImageFormat;

pub trait ImageProcessor: Send + Sync {
    fn supported_formats(&self) -> &[ImageFormat];
    fn process(&self, input: &[u8], config: &ProcessingConfig) -> Result<Vec<u8>, ProcessingError>;
}
