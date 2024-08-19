use std::io;
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgGroup, Parser};
use image::GenericImageView;
use image::imageops::FilterType;

mod is_animated_gif;

use crate::is_animated_gif::is_animated_gif;

/// Create a thumbnail for the image, and return the relative path of
/// the thumbnail within the collection folder.
pub fn create_thumbnail(
    path: &PathBuf,
    out_dir: &PathBuf,
    height: Option<u32>,
    width: Option<u32>,
) -> io::Result<PathBuf> {
    let thumbnail_path = out_dir.join(path.file_name().unwrap());

    // TODO: Does this check do what I think?
    assert!(*path != thumbnail_path);

    let img = image::open(path)
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    // Assert that exactly one of `width` and `height` are defined
    assert!(width.is_some() || height.is_some());
    assert!(width.is_none() || height.is_none());

    // Calculate the new width/height of the image
    let (new_width, new_height) = match (width, height) {
      (Some(w), None) if w >= img.width() => img.dimensions(),
      (None, Some(h)) if h >= img.height() => img.dimensions(),

      (Some(w), None) => (w, w * img.height() / img.width()),
      (None, Some(h)) => (h * img.width() / img.height(), h),

      _ =>  unreachable!(),
    };

    println!("w = {:?}, h = {:?}", new_width, new_height);

    //
    if is_animated_gif(path)? {
      let mp4_path = thumbnail_path.with_extension("mp4");

      Command::new("ffmpeg")
          .args([
              "-i",
              path.to_str().unwrap(),
              "-movflags",
              "faststart",
              "-pix_fmt",
              "yuv420p",
              "-vf",
              &format!("scale={}:{}", new_width, new_height),
              mp4_path.to_str().unwrap(),
          ])
          .output()
          .expect("failed to create thumbnail");

      Ok(mp4_path)
    } else {
      println!("thumbnail_path = {:?}", thumbnail_path);
      img.resize(new_width, new_height, FilterType::Lanczos3)
          .save(&thumbnail_path)
          .unwrap();

      Ok(thumbnail_path)
    }
}

#[derive(Debug, Parser)]
#[clap(version, about)]
#[clap(group(
  ArgGroup::new("dimensions")
      .required(true)
      .args(&["height", "width"]),
))]
struct Cli {
    /// Path to the image to be thumbnailed
    path: PathBuf,

    /// Path to the directory to save the thumbnail in
    #[arg(long)]
    out_dir: PathBuf,

    /// Height of the thumbnail to create
    #[arg(long)]
    height: Option<u32>,

    /// Width of the thumbnail to create
    #[arg(long)]
    width: Option<u32>,
}

fn main() {
    let cli = Cli::parse();

    println!("args = {:?}", cli);

    create_thumbnail(&cli.path, &cli.out_dir, cli.height, cli.width).unwrap();
}
