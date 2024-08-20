#![deny(warnings)]

use std::io;
use std::path::PathBuf;

use clap::{ArgGroup, Parser};

mod create_parent_directory;
mod create_thumbnail;
mod get_thumbnail_dimensions;
mod is_animated_gif;

use crate::create_parent_directory::create_parent_directory;
use crate::get_thumbnail_dimensions::get_thumbnail_dimensions;
use crate::is_animated_gif::is_animated_gif;

/// Create a thumbnail for the image, and return the relative path of
/// the thumbnail within the collection folder.
///
/// TODO: Having two Option<u32> arguments is confusing because they could
/// easily be swapped.  Replace this with some sort of struct!
pub fn create_thumbnail(
    path: &PathBuf,
    out_dir: &PathBuf,
    width: Option<u32>,
    height: Option<u32>,
) -> io::Result<PathBuf> {
    let thumbnail_path = out_dir.join(path.file_name().unwrap());
    create_parent_directory(&thumbnail_path)?;

    // TODO: Does this check do what I think?
    assert!(*path != thumbnail_path);

    println!("w = {:?}", width);
    println!("h = {:?}", height);

    let (new_width, new_height) = get_thumbnail_dimensions(&path, width, height)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    println!("w = {:?}, h = {:?}", new_width, new_height);

    //
    if is_animated_gif(path)? {
        create_thumbnail::create_animated_gif_thumbnail(path, out_dir, new_width, new_height)
    } else {
        create_thumbnail::create_static_thumbnail(path, out_dir, new_width, new_height)
    }
}

#[cfg(test)]
mod test_create_thumbnail {
    use std::path::PathBuf;

    use super::create_thumbnail;
    use crate::test_utils::{get_dimensions, test_dir};

    #[test]
    fn creates_an_animated_gif_thumbnail() {
        let gif_path = PathBuf::from("src/tests/animated_squares.gif");
        let out_dir = test_dir();
        let target_width = Some(16);
        let target_height = None;

        let thumbnail_path =
            create_thumbnail(&gif_path, &out_dir, target_width, target_height).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("animated_squares.mp4"));
        assert!(thumbnail_path.exists());
    }

    #[test]
    fn creates_a_static_gif_thumbnail() {
        let gif_path = PathBuf::from("src/tests/yellow.gif");
        let out_dir = test_dir();
        let target_width = Some(16);
        let target_height = None;

        let thumbnail_path =
            create_thumbnail(&gif_path, &out_dir, target_width, target_height).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("yellow.gif"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 8));
    }

    #[test]
    fn creates_a_png_thumbnail() {
        let gif_path = PathBuf::from("src/tests/red.png");
        let out_dir = test_dir();
        let target_width = Some(16);
        let target_height = None;

        let thumbnail_path =
            create_thumbnail(&gif_path, &out_dir, target_width, target_height).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("red.png"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 32));
    }

    #[test]
    fn creates_a_jpeg_thumbnail() {
        let gif_path = PathBuf::from("src/tests/noise.jpg");
        let out_dir = test_dir();
        let target_width = Some(16);
        let target_height = None;

        let thumbnail_path =
            create_thumbnail(&gif_path, &out_dir, target_width, target_height).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("noise.jpg"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 32));
    }

    #[test]
    fn creates_a_tif_thumbnail() {
        let gif_path = PathBuf::from("src/tests/green.tiff");
        let out_dir = test_dir();
        let target_width = Some(16);
        let target_height = None;

        let thumbnail_path =
            create_thumbnail(&gif_path, &out_dir, target_width, target_height).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("green.tiff"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 16));
    }

    #[test]
    fn creates_a_webp_thumbnail() {
        let gif_path = PathBuf::from("src/tests/purple.webp");
        let out_dir = test_dir();
        let target_width = Some(16);
        let target_height = None;

        let thumbnail_path =
            create_thumbnail(&gif_path, &out_dir, target_width, target_height).unwrap();

        assert_eq!(thumbnail_path, out_dir.join("purple.webp"));
        assert!(thumbnail_path.exists());
        assert_eq!(get_dimensions(&thumbnail_path), (16, 16));
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

    create_thumbnail(&cli.path, &cli.out_dir, cli.width, cli.height).unwrap();
}

#[cfg(test)]
mod test_cli {
    use regex::Regex;

    use crate::test_utils::{get_failure, get_success};

    #[test]
    fn it_errors_if_you_pass_width_and_height() {
        let output = get_failure(&[
            "src/tests/red.png",
            "--width=100",
            "--height=100",
            "--out-dir=/tmp",
        ]);

        let re =
            Regex::new(r"the argument '--width <WIDTH>' cannot be used with '--height <HEIGHT>'")
                .unwrap();
        assert!(re.is_match(&output.stderr));

        assert_eq!(output.exit_code, 2);
        assert_eq!(output.stdout, "");
    }

    #[test]
    fn it_errors_if_you_pass_neither_width_nor_height() {
        let output = get_failure(&["src/tests/red.png", "--out-dir=/tmp"]);

        let re = Regex::new(r"the following required arguments were not provided:").unwrap();
        assert!(re.is_match(&output.stderr));

        assert_eq!(output.exit_code, 2);
        assert_eq!(output.stdout, "");
    }

    #[test]
    fn it_prints_the_version() {
        let output = get_success(&["--version"]);

        let re = Regex::new(r"^create_thumbnail [0-9]+\.[0-9]+\.[0-9]+\n$").unwrap();

        assert!(re.is_match(&output.stdout));

        assert_eq!(output.exit_code, 0);
        assert_eq!(output.stderr, "");
    }

    #[test]
    fn it_prints_the_help() {
        let output = get_success(&["--help"]);

        let re = Regex::new(r"create_thumbnail --out-dir").unwrap();

        assert!(re.is_match(&output.stdout));

        assert_eq!(output.exit_code, 0);
        assert_eq!(output.stderr, "");
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::path::PathBuf;
    use std::str;

    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::Command;
    use image::GenericImageView;

    /// Return a path to a temporary directory to use for testing.
    ///
    /// This function does *not* create the directory, just the path.
    pub fn test_dir() -> PathBuf {
        let tmp_dir = tempdir::TempDir::new("testing").unwrap();

        tmp_dir.path().to_owned()
    }

    /// Return the dimensions for an image.
    pub fn get_dimensions(path: &PathBuf) -> (u32, u32) {
        let img = image::open(path).unwrap();

        img.dimensions()
    }

    pub struct DcOutput {
        pub exit_code: i32,
        pub stdout: String,
        pub stderr: String,
    }

    pub fn get_success(args: &[&str]) -> DcOutput {
        let mut cmd = Command::cargo_bin("create_thumbnail").unwrap();
        let output = cmd
            .args(args)
            .unwrap()
            .assert()
            .success()
            .get_output()
            .to_owned();

        DcOutput {
            exit_code: output.status.code().unwrap(),
            stdout: str::from_utf8(&output.stdout).unwrap().to_owned(),
            stderr: str::from_utf8(&output.stderr).unwrap().to_owned(),
        }
    }

    pub fn get_failure(args: &[&str]) -> DcOutput {
        let mut cmd = Command::cargo_bin("create_thumbnail").unwrap();
        let output = cmd.args(args).unwrap_err().as_output().unwrap().to_owned();

        DcOutput {
            exit_code: output.status.code().unwrap(),
            stdout: str::from_utf8(&output.stdout).unwrap().to_owned(),
            stderr: str::from_utf8(&output.stderr).unwrap().to_owned(),
        }
    }
}
