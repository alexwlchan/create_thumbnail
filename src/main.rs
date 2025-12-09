#![deny(warnings)]

use std::path::PathBuf;

use clap::Parser;

mod create_parent_directory;
mod create_thumbnail;
mod errors;
mod get_thumbnail_dimensions;
mod is_animated_gif;

use crate::create_thumbnail::create_thumbnail;
use crate::get_thumbnail_dimensions::TargetDimension;

#[derive(Debug, Parser)]
#[clap(version, about)]
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

    let target = match (cli.width, cli.height) {
        (Some(w), Some(h)) => TargetDimension::BoundingBox(w, h),
        (Some(w), None) => TargetDimension::MaxWidth(w),
        (None, Some(h)) => TargetDimension::MaxHeight(h),
        _ => {
            eprintln!(
                "Failed to create thumbnail: you must pass at least one of --width or --height"
            );
            std::process::exit(1);
        }
    };

    match create_thumbnail(&cli.path, &cli.out_dir, target) {
        Ok(thumbnail_path) => print!("{}", thumbnail_path.display()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
}

#[expect(
    deprecated,
    reason = "cargo_bin is deprecated, cargo_bin! is not, `use` does not differentiate them. See https://github.com/assert-rs/assert_cmd/issues/258"
)]
#[cfg(test)]
mod test_cli {
    use std::path::PathBuf;

    use assert_cmd::Command;
    use predicates::prelude::*;

    use crate::test_utils::get_dimensions;

    #[test]
    fn it_creates_a_thumbnail_with_max_width() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&["src/tests/red.png", "--width=50", "--out-dir=/tmp"])
            .assert()
            .success()
            .stdout("/tmp/red.png")
            .stderr("");

        assert_eq!(get_dimensions(&PathBuf::from("/tmp/red.png")), (50, 100));
    }

    #[test]
    fn it_creates_a_thumbnail_with_max_height() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&["src/tests/noise.jpg", "--height=128", "--out-dir=/tmp"])
            .assert()
            .success()
            .stdout("/tmp/noise.jpg")
            .stderr("");

        assert_eq!(get_dimensions(&PathBuf::from("/tmp/noise.jpg")), (64, 128));
    }

    #[test]
    fn it_creates_a_thumbnail_with_a_bounding_box() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&[
                "src/tests/noise.jpg",
                "--width=64",
                "--height=64",
                "--out-dir=/tmp",
            ])
            .assert()
            .success()
            .stdout("/tmp/noise.jpg")
            .stderr("");

        assert_eq!(get_dimensions(&PathBuf::from("/tmp/noise.jpg")), (32, 64));
    }

    #[test]
    fn it_fails_if_you_pass_neither_width_nor_height() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&["src/tests/red.png", "--out-dir=/tmp"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr(
                "Failed to create thumbnail: you must pass at least one of --width or --height\n",
            );
    }

    #[test]
    fn it_fails_if_you_pass_a_non_existent_file() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&["doesnotexist.txt", "--width=50", "--out-dir=/tmp"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("Failed to open image: No such file or directory (os error 2)\n");
    }

    #[test]
    fn it_fails_if_you_pass_a_non_image() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&["Cargo.toml", "--width=50", "--out-dir=/tmp"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("Failed to open image: The file extension `.\"toml\"` was not recognized as an image format\n");
    }

    // TODO: Improve this error message.
    //
    // It's good to know the tool won't completely break when this happens, but ideally
    // we'd return a more meaningful error message in this case.
    #[test]
    fn it_fails_if_out_dir_is_a_file() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&["src/images/noise.jpg", "--width=50", "--out-dir=README.md"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("I/O error: File exists (os error 17)\n");
    }

    #[test]
    fn it_fails_if_you_try_to_overwrite_the_original_file() {
        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .args(&["src/images/noise.jpg", "--width=50", "--out-dir=src/images"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("Cannot write thumbnail to the same path as the original image\n");
    }

    #[test]
    fn it_prints_the_version() {
        // Match strings like `create_thumbnail 1.2.3`
        let is_version_string =
            predicate::str::is_match(r"^create_thumbnail [0-9]+\.[0-9]+\.[0-9]+\n$").unwrap();

        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .arg("--version")
            .assert()
            .success()
            .stdout(is_version_string)
            .stderr("");
    }

    #[test]
    fn it_prints_the_help() {
        // Match strings like `create_thumbnail 1.2.3`
        let is_help_text =
            predicate::str::is_match(r"create_thumbnail \[OPTIONS\] --out-dir").unwrap();

        Command::cargo_bin("create_thumbnail")
            .unwrap()
            .arg("--help")
            .assert()
            .success()
            .stdout(is_help_text)
            .stderr("");
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::path::PathBuf;

    use image::GenericImageView;

    /// Return a path to a temporary directory to use for testing.
    ///
    /// This function does *not* create the directory, just the path.
    pub fn test_dir() -> PathBuf {
        let tmp_dir = tempfile::tempdir().unwrap();

        tmp_dir.path().to_owned()
    }

    /// Return the dimensions for an image.
    pub fn get_dimensions(path: &PathBuf) -> (u32, u32) {
        let img = image::open(path).unwrap();

        img.dimensions()
    }
}
