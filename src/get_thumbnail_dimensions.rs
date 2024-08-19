use std::path::PathBuf;

use image::GenericImageView;

/// Given the path to the original image and the target width/height,
/// calculate the dimensions of the new image.
///
/// This function expects that exactly one of width/height will be
/// specified, and then the image will be resized to be no larger
/// than this dimension.
///
/// If the image is smaller than the target dimensions, it will be
/// left as-is.
///
/// TODO: Are there any scenarios in which this division could round
/// one dimension of an image to zero, if it was very tall or very long?
///
pub fn get_thumbnail_dimensions(
    path: &PathBuf,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> Result<(u32, u32), image::error::ImageError> {
    // Assert that exactly one of `target_width` and `target_height` are defined
    assert!(target_width.is_some() || target_height.is_some());
    assert!(target_width.is_none() || target_height.is_none());

    // Open the image, and compare its dimensions to the target
    let img = image::open(path)?;

    // Calculate the new width/height of the image
    let (new_width, new_height) = match (target_width, target_height) {
        (Some(w), None) if w >= img.width() => img.dimensions(),
        (None, Some(h)) if h >= img.height() => img.dimensions(),

        (Some(w), None) => (w, w * img.height() / img.width()),
        (None, Some(h)) => (h * img.width() / img.height(), h),

        _ => unreachable!(),
    };

    Ok((new_width, new_height))
}

#[cfg(test)]
mod test_get_thumbnail_dimensions {
    use std::path::PathBuf;

    use super::*;

    // The `red.png` file used in this test has dimensions 100×200

    #[test]
    fn with_target_width() {
        let p = PathBuf::from("src/tests/red.png");

        let target_width: Option<u32> = Some(50);
        let target_height: Option<u32> = None;

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert_eq!(dimensions.unwrap(), (50, 100));
    }

    #[test]
    fn with_target_height() {
        let p = PathBuf::from("src/tests/red.png");

        let target_width: Option<u32> = None;
        let target_height: Option<u32> = Some(100);

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert_eq!(dimensions.unwrap(), (50, 100));
    }

    #[test]
    fn leaves_image_as_is_if_target_width_greater_than_width() {
        let p = PathBuf::from("src/tests/red.png");

        let target_width: Option<u32> = Some(500);
        let target_height: Option<u32> = None;

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn leaves_image_as_is_if_target_width_equal_to_width() {
        let p = PathBuf::from("src/tests/red.png");

        let target_width: Option<u32> = Some(500);
        let target_height: Option<u32> = None;

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn leaves_image_as_is_if_target_height_greater_than_height() {
        let p = PathBuf::from("src/tests/red.png");

        let target_width: Option<u32> = None;
        let target_height: Option<u32> = Some(500);

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn leaves_image_as_is_if_target_height_equal_to_height() {
        let p = PathBuf::from("src/tests/red.png");

        let target_width: Option<u32> = None;
        let target_height: Option<u32> = Some(500);

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn errors_if_image_does_not_exist() {
        let p = PathBuf::from("src/tests/doesnotexist.png");

        let target_width: Option<u32> = Some(50);
        let target_height: Option<u32> = None;

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert!(dimensions.is_err());
    }

    #[test]
    fn errors_if_cannot_read_image() {
        let p = PathBuf::from("README.md");

        let target_width: Option<u32> = Some(50);
        let target_height: Option<u32> = None;

        let dimensions = get_thumbnail_dimensions(&p, target_width, target_height);
        assert!(dimensions.is_err());
    }
}
