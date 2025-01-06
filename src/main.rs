#![deny(warnings)]

use std::path::PathBuf;

use clap::{ArgGroup, Parser};

mod create_parent_directory;
mod create_thumbnail;
mod get_thumbnail_dimensions;
mod is_animated_gif;

use crate::create_thumbnail::create_thumbnail;
use crate::get_thumbnail_dimensions::TargetDimension;

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

    let target = match (cli.width, cli.height) {
        (Some(w), None) => TargetDimension::MaxWidth(w),
        (None, Some(h)) => TargetDimension::MaxHeight(h),
        _ => unreachable!(),
    };

    match create_thumbnail(&cli.path, &cli.out_dir, target) {
        Ok(thumbnail_path) => print!("{}", thumbnail_path.display()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
}

#[cfg(test)]
mod test_cli {
    use std::path::PathBuf;

    use predicates::prelude::*;

    use crate::run_command;
    use crate::test_utils::get_dimensions;

    #[test]
    fn it_creates_a_thumbnail_with_max_width() {
        let result = run_command!("src/tests/red.png", "--width=50", "--out-dir=/tmp");

        result.success().stdout("/tmp/red.png").stderr("");

        assert_eq!(get_dimensions(&PathBuf::from("/tmp/red.png")), (50, 100));
    }

    #[test]
    fn it_creates_a_thumbnail_with_max_height() {
        let result = run_command!("src/tests/noise.jpg", "--height=128", "--out-dir=/tmp");

        result.success().stdout("/tmp/noise.jpg").stderr("");

        assert_eq!(get_dimensions(&PathBuf::from("/tmp/noise.jpg")), (64, 128));
    }

    #[test]
    fn it_fails_if_you_pass_width_and_height() {
        let result = run_command!(
            "src/tests/red.png",
            "--width=100",
            "--height=100",
            "--out-dir=/tmp",
        );

        let is_invalid_args_err = predicate::str::is_match(
            r"the argument '--width <WIDTH>' cannot be used with '--height <HEIGHT>'",
        )
        .unwrap();

        result
            .failure()
            .code(2)
            .stdout("")
            .stderr(is_invalid_args_err);
    }

    #[test]
    fn it_fails_if_you_pass_neither_width_nor_height() {
        let result = run_command!("src/tests/red.png", "--out-dir=/tmp");

        let is_missing_args_err =
            predicate::str::is_match(r"the following required arguments were not provided:")
                .unwrap();

        result
            .failure()
            .code(2)
            .stdout("")
            .stderr(is_missing_args_err);
    }

    #[test]
    fn it_fails_if_you_pass_a_non_existent_file() {
        let result = run_command!("doesnotexist.txt", "--width=50", "--out-dir=/tmp");

        result
            .failure()
            .code(1)
            .stdout("")
            .stderr("No such file or directory (os error 2)\n");
    }

    #[test]
    fn it_fails_if_you_pass_a_non_image() {
        let result = run_command!("Cargo.toml", "--width=50", "--out-dir=/tmp");

        result
            .failure()
            .code(1)
            .stdout("")
            .stderr("The image format could not be determined\n");
    }

    // TODO: Improve this error message.
    //
    // It's good to know the tool won't completely break when this happens, but ideally
    // we'd return a more meaningful error message in this case.
    #[test]
    fn it_fails_if_out_dir_is_a_file() {
        let result = run_command!("src/images/noise.jpg", "--width=50", "--out-dir=README.md");

        result
            .failure()
            .code(1)
            .stdout("")
            .stderr("File exists (os error 17)\n");
    }

    #[test]
    fn it_fails_if_you_try_to_overwrite_the_original_file() {
        let result = run_command!("src/images/noise.jpg", "--width=50", "--out-dir=src/images");

        result
            .failure()
            .code(1)
            .stdout("")
            .stderr("Cannot write thumbnail to the same directory as the original image\n");
    }

    #[test]
    fn it_prints_the_version() {
        let result = run_command!("--version");

        // Match strings like `create_thumbnail 1.2.3`
        let is_version_string =
            predicate::str::is_match(r"^create_thumbnail [0-9]+\.[0-9]+\.[0-9]+\n$").unwrap();

        result.success().stdout(is_version_string).stderr("");
    }

    #[test]
    fn it_prints_the_help() {
        let result = run_command!("--help");

        // Match strings like `dominant_colours 1.2.3`
        let is_help_text = predicate::str::is_match(r"create_thumbnail --out-dir").unwrap();

        result.success().stdout(is_help_text).stderr("");
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

#[cfg(test)]
#[macro_use]
mod test_helpers {
    /// Run this command-line tool with zero or more arguments:
    ///
    ///     run_command!();
    ///     run_command!("shape.png");
    ///     run_command!("shape.png", "--sides=4", "--colour=red");
    ///
    /// This returns an `assert_cmd::assert::Assert` that will allow
    /// you to make assertions about the output.
    /// See https://docs.rs/assert_cmd/latest/assert_cmd/assert/struct.Assert.html
    #[macro_export]
    macro_rules! run_command {
        () => {
            assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
                       .unwrap()
                       .assert()
        };

        ($($arg:expr),+ $(,)?) => {{
            assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
                       .unwrap()
                       .args(&[$($arg),*])
                       .assert()
        }};
    }
}
