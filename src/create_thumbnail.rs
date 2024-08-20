use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use image::imageops::FilterType;

use crate::create_parent_directory::create_parent_directory;
use crate::get_thumbnail_dimensions::{get_thumbnail_dimensions, TargetDimension};
use crate::is_animated_gif::is_animated_gif;

/// Create a thumbnail for the image, and return the relative path of
/// the thumbnail within the collection folder.
pub fn create_thumbnail(
    path: &PathBuf,
    out_dir: &PathBuf,
    target: TargetDimension,
) -> io::Result<PathBuf> {
    let thumbnail_path = out_dir.join(path.file_name().unwrap());
    create_parent_directory(&thumbnail_path)?;

    // TODO: Does this check do what I think?
    assert!(*path != thumbnail_path);

    let (new_width, new_height) = get_thumbnail_dimensions(&path, target)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

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
) -> io::Result<PathBuf> {
    let file_name = gif_path.file_name().unwrap();
    let thumbnail_path = out_dir.join(file_name).with_extension("mp4");

    let dimension_str = format!("scale={}:{}", width, height);

    let output = Command::new("ffmpeg")
        .args([
            "-i",
            gif_path.to_str().unwrap(),
            "-movflags",
            "faststart",
            "-pix_fmt",
            "yuv420p",
            "-vf",
            &dimension_str,
            thumbnail_path.to_str().unwrap(),
        ])
        .output()
        .expect("failed to create thumbnail");

    if output.status.success() {
        Ok(thumbnail_path)
    } else {
        let error_message = format!(
            "Unable to invoke ffmpeg!\nstderr from ffmpeg:\n{}",
            str::from_utf8(&output.stderr).unwrap()
        );
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }
}

/// Create a thumbnail for a static (non-animated) image.
///
/// This function assumes that the original image file definitely exists.
///
/// TODO: Get rid of the use of `unwrap()` in this code.
///
pub fn create_static_thumbnail(
    image_path: &PathBuf,
    out_dir: &PathBuf,
    width: u32,
    height: u32,
) -> io::Result<PathBuf> {
    let file_name = image_path.file_name().unwrap();
    let thumbnail_path = out_dir.join(file_name);

    let img = image::open(image_path).unwrap();

    img.resize(width, height, FilterType::Lanczos3)
        .save(&thumbnail_path)
        .unwrap();

    Ok(thumbnail_path)
}
