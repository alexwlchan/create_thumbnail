use std::fs::File;
use std::io::{BufReader, Result};
use std::path::PathBuf;

use image::codecs::gif::GifDecoder;
use image::codecs::webp::WebPDecoder;
use image::{AnimationDecoder, ImageFormat};

/// Returns True if a file is an animated image, and False otherwise.
pub fn is_animated(path: &PathBuf) -> Result<bool> {
    let format = match path.extension() {
        Some(ext) => ImageFormat::from_extension(ext),
        None => None,
    };

    match format {
        Some(ImageFormat::Gif) => {
            let f = File::open(path)?;
            let reader = BufReader::new(f);
            match GifDecoder::new(reader) {
              Ok(decoder) => Ok(decoder.into_frames().count() > 1),
              Err(_) => Ok(false),
            }
        },

        Some(ImageFormat::WebP) => {
            let f = File::open(path)?;
            let reader = BufReader::new(f);
            match WebPDecoder::new(reader) {
              Ok(decoder) => Ok(decoder.into_frames().count() > 1),
              Err(_) => Ok(false),
            }
        },

        _ => Ok(false),
    }
}

#[cfg(test)]
mod test_is_animated {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn a_png_is_not_animated() {
        let p = PathBuf::from("src/tests/blue.png");
        assert_eq!(is_animated(&p).unwrap(), false);
    }

    #[test]
    fn a_static_gif_is_not_an_animated() {
        let p = PathBuf::from("src/tests/static.gif");
        assert_eq!(is_animated(&p).unwrap(), false);
    }

    #[test]
    fn an_animated_gif_is_animated() {
        let p = PathBuf::from("src/tests/animated_squares.gif");
        assert_eq!(is_animated(&p).unwrap(), true);
    }

    #[test]
    fn an_animated_webp_is_animated() {
        let p = PathBuf::from("src/tests/animated_squares.webp");
        assert_eq!(is_animated(&p).unwrap(), true);
    }

    #[test]
    fn a_non_image_is_not_animated() {
        let p = PathBuf::from("Cargo.toml");
        assert_eq!(is_animated(&p).unwrap(), false);
    }

    #[test]
    fn a_malformed_gif_is_not_animated() {
        let p = PathBuf::from("src/tests/malformed.txt.gif");
        assert_eq!(is_animated(&p).unwrap(), false);
    }

    #[test]
    fn a_malformed_webp_is_not_animated() {
        let p = PathBuf::from("src/tests/malformed.txt.webp");
        assert_eq!(is_animated(&p).unwrap(), false);
    }

    #[test]
    fn a_file_which_doesnt_exist_is_an_error() {
        let p = PathBuf::from("does_not_exist.txt");
        assert!(is_animated(&p).is_err());
    }
}
