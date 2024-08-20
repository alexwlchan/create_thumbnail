use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use image::imageops::FilterType;

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
