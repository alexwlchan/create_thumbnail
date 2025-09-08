use std::path::PathBuf;
use std::process::Command;
use std::str;

use image::imageops::FilterType;
use image::{DynamicImage, ImageDecoder, ImageReader};

use crate::create_parent_directory::create_parent_directory;
use crate::errors::ThumbnailError;
use crate::get_thumbnail_dimensions::{get_thumbnail_dimensions, TargetDimension};
use crate::is_animated_gif::is_animated_gif;

/// Create a thumbnail for the image, and return the relative path of
/// the thumbnail within the collection folder.
pub fn create_thumbnail(
    path: &PathBuf,
    out_dir: &PathBuf,
    target: TargetDimension,
) -> Result<PathBuf, ThumbnailError> {
    let file_name = path.file_name().ok_or(ThumbnailError::MissingFileName)?;
    let thumbnail_path = out_dir.join(file_name);
    create_parent_directory(&thumbnail_path)?;

    // Make sure we don't overwrite the original image with a thumbnail
    if *path == thumbnail_path {
        return Err(ThumbnailError::SameInputOutputPath);
    }

    let (new_width, new_height) = get_thumbnail_dimensions(&path, target)?;

    if is_animated_gif(path)? {
        create_animated_gif_thumbnail(path, out_dir, new_width, new_height)
    } else {
        create_static_thumbnail(path, out_dir, new_width, new_height)
    }
}

#[cfg(test)]
mod test_create_thumbnail {
    use std::path::PathBuf;

    use super::create_thumbnail;
    use crate::get_thumbnail_dimensions::TargetDimension;
    use crate::test_utils::{get_dimensions, test_dir};

    #[test]
    fn creates_an_animated_gif_thumbnail() {
        let gif_path = PathBuf::from("src/tests/animated_squares.gif");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(16);

        let thumbnail_path = create_thumbnail(&gif_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("animated_squares.mp4"));
        assert!(thumbnail_path.exists());
    }

    #[test]
    fn creates_an_animated_gif_thumbnail_with_odd_width() {
        let gif_path = PathBuf::from("src/tests/animated_squares.gif");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(15);

        let thumbnail_path = create_thumbnail(&gif_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("animated_squares.mp4"));
        assert!(thumbnail_path.exists());
    }

    #[test]
    fn creates_a_static_gif_thumbnail() {
        let img_path = PathBuf::from("src/tests/yellow.gif");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(16);

        let thumbnail_path = create_thumbnail(&img_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("yellow.gif"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 8));
    }

    #[test]
    fn creates_a_png_thumbnail() {
        let img_path = PathBuf::from("src/tests/red.png");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(16);

        let thumbnail_path = create_thumbnail(&img_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("red.png"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 32));
    }

    #[test]
    fn creates_a_jpeg_thumbnail() {
        let img_path = PathBuf::from("src/tests/noise.jpg");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(16);

        let thumbnail_path = create_thumbnail(&img_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("noise.jpg"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 32));
    }

    #[test]
    fn creates_a_tif_thumbnail() {
        let img_path = PathBuf::from("src/tests/green.tiff");
        let out_dir = test_dir();
        let target = TargetDimension::MaxHeight(16);

        let thumbnail_path = create_thumbnail(&img_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("green.tiff"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 16));
    }

    #[test]
    fn creates_a_webp_thumbnail() {
        let img_path = PathBuf::from("src/tests/purple.webp");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(16);

        let thumbnail_path = create_thumbnail(&img_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("purple.webp"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 16));
    }

    #[test]
    fn it_creates_an_equal_size_thumbnail_if_dimension_larger_than_original() {
        let img_path = PathBuf::from("src/tests/noise.jpg");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(500);

        let thumbnail_path = create_thumbnail(&img_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("noise.jpg"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (128, 256));
    }

    #[test]
    fn it_applies_exif_orientation() {
        // This source image comes from Dave Perrett's exif-orientation-examples
        // repo, and is used under MIT.
        // See https://github.com/recurser/exif-orientation-examples
        let img_path = PathBuf::from("src/tests/Landscape_5.jpg");
        let out_dir = test_dir();
        let target = TargetDimension::MaxWidth(180);

        let thumbnail_path = create_thumbnail(&img_path, &out_dir, target).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("Landscape_5.jpg"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (180, 120));
    }
}

/// Return this value if it's even, or the closest value which is even.
fn ensure_even(x: u32) -> u32 {
    if x % 2 == 0 {
        x
    } else {
        x + 1
    }
}

/// Create a thumbnail for an animated GIF.
///
/// This will use `ffmpeg` to create an MP4 file of the desired dimensions
/// which plays the GIF on a loop.  This is typically much smaller and more
/// space-efficient than creating a resized GIF.
///
/// This function assumes that the original GIF file definitely exists.
///
/// TODO: It would be nice to have a test for the case where `ffmpeg` isn't
/// installed, but I'm not sure how to simulate that.
///
pub fn create_animated_gif_thumbnail(
    gif_path: &PathBuf,
    out_dir: &PathBuf,
    width: u32,
    height: u32,
) -> Result<PathBuf, ThumbnailError> {
    let file_name = gif_path
        .file_name()
        .ok_or(ThumbnailError::MissingFileName)?;

    let thumbnail_path = out_dir.join(file_name).with_extension("mp4");

    let gif_path_str = gif_path
        .to_str()
        .ok_or(ThumbnailError::PathConversionError)?;
    let thumbnail_path_str = thumbnail_path
        .to_str()
        .ok_or(ThumbnailError::PathConversionError)?;

    // There's a subtlety here with ffmpeg I don't understand fully -- if
    // the width/height aren't even, it doesn't create the MP4, instead
    // failing with the error:
    //
    //     width not divisible by 2
    //
    // I don't usually need these files to be pixel-perfect width, so
    // fudging by a single pixel or two is fine.
    let dimension_str = format!("scale={}:{}", ensure_even(width), ensure_even(height));

    let output = Command::new("ffmpeg")
        .args([
            "-i",
            gif_path_str,
            "-movflags",
            "faststart",
            "-pix_fmt",
            "yuv420p",
            "-vf",
            &dimension_str,
            thumbnail_path_str,
        ])
        .output()
        .map_err(|e| ThumbnailError::CommandFailed(format!("Failed to run ffmpeg: {}", e)))?;

    if output.status.success() {
        Ok(thumbnail_path)
    } else {
        let stderr = str::from_utf8(&output.stderr)?;
        Err(ThumbnailError::CommandFailed(stderr.to_string()))
    }
}

/// Create a thumbnail for a static (non-animated) image.
///
/// This function assumes that the original image file definitely exists.
///
pub fn create_static_thumbnail(
    image_path: &PathBuf,
    out_dir: &PathBuf,
    width: u32,
    height: u32,
) -> Result<PathBuf, ThumbnailError> {
    let file_name = image_path
        .file_name()
        .ok_or(ThumbnailError::MissingFileName)?;

    let thumbnail_path = out_dir.join(file_name);

    let mut decoder = ImageReader::open(image_path)?.into_decoder()?;
    let orientation = decoder.orientation()?;
    let mut img = DynamicImage::from_decoder(decoder)?;
    img.apply_orientation(orientation);

    img.resize(width, height, FilterType::Lanczos3)
        .save(&thumbnail_path)
        .map_err(ThumbnailError::ImageSaveError)?;

    Ok(thumbnail_path)
}
