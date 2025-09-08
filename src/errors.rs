use std::fmt;
use std::io;

use image::ImageError;

#[derive(Debug)]
pub enum ThumbnailError {
    MissingFileName,
    ImageOpenError(ImageError),
    ImageSaveError(ImageError),
    CommandFailed(String),
    Utf8Error(std::str::Utf8Error),
    PathConversionError,
    SameInputOutputPath,
    IoError(std::io::Error),
}

impl fmt::Display for ThumbnailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThumbnailError::MissingFileName => write!(f, "Image path is missing a file name"),
            ThumbnailError::ImageOpenError(e) => write!(f, "Failed to open image: {}", e),
            ThumbnailError::ImageSaveError(e) => write!(f, "Failed to save thumbnail: {}", e),
            ThumbnailError::CommandFailed(msg) => write!(f, "ffmpeg command failed: {}", msg),
            ThumbnailError::Utf8Error(e) => write!(f, "Failed to decode ffmpeg output: {}", e),
            ThumbnailError::PathConversionError => write!(f, "Failed to convert path to string"),
            ThumbnailError::SameInputOutputPath => write!(
                f,
                "Cannot write thumbnail to the same path as the original image"
            ),
            ThumbnailError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for ThumbnailError {}

impl From<ImageError> for ThumbnailError {
    fn from(err: ImageError) -> Self {
        ThumbnailError::ImageOpenError(err)
    }
}

impl From<std::str::Utf8Error> for ThumbnailError {
    fn from(err: std::str::Utf8Error) -> Self {
        ThumbnailError::Utf8Error(err)
    }
}

impl From<io::Error> for ThumbnailError {
    fn from(err: io::Error) -> Self {
        ThumbnailError::IoError(err)
    }
}
