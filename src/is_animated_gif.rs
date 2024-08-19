use std::fs::File;
use std::io::{BufReader, Result};
use std::path::PathBuf;

use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;

/// Returns True if a file is an animated GIF, and False otherwise.
pub fn is_animated_gif(path: &PathBuf) -> Result<bool> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader);

    match decoder {
        Ok(dc) => Ok(dc.into_frames().count() > 1),
        _ => Ok(false),
    }
}

#[cfg(test)]
mod test_is_animated_gif {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn a_png_is_not_an_animated_gif() {
        let p = PathBuf::from("src/tests/blue.png");
        assert_eq!(is_animated_gif(&p).unwrap(), false);
    }

    #[test]
    fn a_static_gif_is_not_an_animated_gif() {
        let p = PathBuf::from("src/tests/static.gif");
        assert_eq!(is_animated_gif(&p).unwrap(), false);
    }

    #[test]
    fn an_animated_gif_is_animated() {
        let p = PathBuf::from("src/tests/animated_squares.gif");
        assert_eq!(is_animated_gif(&p).unwrap(), true);
    }

    #[test]
    fn a_non_image_is_not_animated_gif() {
        let p = PathBuf::from("Cargo.toml");
        assert_eq!(is_animated_gif(&p).unwrap(), false);
    }

    #[test]
    fn a_file_which_doesnt_exist_is_an_error() {
        let p = PathBuf::from("does_not_exist.txt");
        assert!(is_animated_gif(&p).is_err());
    }
}
