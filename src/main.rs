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

    println!("args = {:?}", cli);

    let target = match (cli.width, cli.height) {
        (Some(w), None) => TargetDimension::MaxWidth(w),
        (None, Some(h)) => TargetDimension::MaxHeight(h),
        _ => unreachable!(),
    };

    create_thumbnail(&cli.path, &cli.out_dir, target).unwrap();
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
