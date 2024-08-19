use std::path::PathBuf;

use clap::{ArgGroup, Parser};

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
    height: Option<usize>,

    /// Width of the thumbnail to create
    #[arg(long)]
    width: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    println!("args = {:?}", cli);
}
