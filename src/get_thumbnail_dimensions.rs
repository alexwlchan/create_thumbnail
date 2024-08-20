use std::path::PathBuf;

use image::GenericImageView;

/// Represents the target dimensions of the thumbnail.
///
/// The user can choose a max width, or a max height, but not both.
pub enum TargetDimension {
    MaxWidth(u32),
    MaxHeight(u32),
}

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
    target: TargetDimension,
) -> Result<(u32, u32), image::error::ImageError> {
    let img = image::open(path)?;

    let (new_width, new_height) = match target {
        TargetDimension::MaxWidth(w) if w >= img.width() => img.dimensions(),
        TargetDimension::MaxHeight(h) if h >= img.height() => img.dimensions(),

        TargetDimension::MaxWidth(w) => (w, w * img.height() / img.width()),
        TargetDimension::MaxHeight(h) => (h * img.width() / img.height(), h),
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

        let target = TargetDimension::MaxWidth(50);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert_eq!(dimensions.unwrap(), (50, 100));
    }

    #[test]
    fn with_target_height() {
        let p = PathBuf::from("src/tests/red.png");

        let target = TargetDimension::MaxHeight(100);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert_eq!(dimensions.unwrap(), (50, 100));
    }

    #[test]
    fn leaves_image_as_is_if_target_width_greater_than_width() {
        let p = PathBuf::from("src/tests/red.png");

        let target = TargetDimension::MaxWidth(500);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn leaves_image_as_is_if_target_width_equal_to_width() {
        let p = PathBuf::from("src/tests/red.png");

        let target = TargetDimension::MaxWidth(500);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn leaves_image_as_is_if_target_height_greater_than_height() {
        let p = PathBuf::from("src/tests/red.png");

        let target = TargetDimension::MaxHeight(500);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn leaves_image_as_is_if_target_height_equal_to_height() {
        let p = PathBuf::from("src/tests/red.png");

        let target = TargetDimension::MaxHeight(500);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert_eq!(dimensions.unwrap(), (100, 200));
    }

    #[test]
    fn errors_if_image_does_not_exist() {
        let p = PathBuf::from("src/tests/doesnotexist.png");

        let target = TargetDimension::MaxWidth(50);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert!(dimensions.is_err());
    }

    #[test]
    fn errors_if_cannot_read_image() {
        let p = PathBuf::from("README.md");

        let target = TargetDimension::MaxWidth(50);

        let dimensions = get_thumbnail_dimensions(&p, target);
        assert!(dimensions.is_err());
    }
}
