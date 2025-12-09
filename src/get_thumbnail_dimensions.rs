use std::path::PathBuf;

use image::GenericImageView;

use crate::errors::ThumbnailError;

/// Represents the target dimensions of the thumbnail.
pub enum TargetDimension {
    BoundingBox(u32, u32),
    MaxWidth(u32),
    MaxHeight(u32),
}

/// Given the path to the original image and the target width/height,
/// calculate the dimensions of the new image.
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
) -> Result<(u32, u32), ThumbnailError> {
    let img = image::open(path)?;

    Ok(calculate_dimensions(img.dimensions(), target))
}

// Calculate the dimensions of the new image, given the original dimensions
// and target dimensions.
fn calculate_dimensions(dimensions: (u32, u32), target: TargetDimension) -> (u32, u32) {
    let (img_w, img_h) = dimensions;

    match target {
        TargetDimension::MaxWidth(max_w) if max_w >= img_w => dimensions,
        TargetDimension::MaxHeight(max_h) if max_h >= img_h => dimensions,

        TargetDimension::MaxWidth(max_w) => (
            max_w,
            ((max_w as f64) * (img_h as f64) / (img_w as f64)).round() as u32,
        ),
        TargetDimension::MaxHeight(max_h) => (
            ((max_h as f64) * (img_w as f64) / (img_h as f64)).round() as u32,
            max_h,
        ),

        // The bounding box has a wider aspect ratio than the original image,
        // so filter by height.
        TargetDimension::BoundingBox(max_w, max_h)
            if (max_w as f64) / (max_h as f64) >= (img_w as f64) / (img_h as f64) =>
        {
            calculate_dimensions(dimensions, TargetDimension::MaxHeight(max_h))
        }
        TargetDimension::BoundingBox(max_w, _) => {
            calculate_dimensions(dimensions, TargetDimension::MaxWidth(max_w))
        }
    }
}

#[cfg(test)]
mod test_get_thumbnail_dimensions {
    use std::path::PathBuf;

    use super::*;

    macro_rules! get_thumb_dimensions_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, target, expected) = $value;

                let dimensions = calculate_dimensions(input, target);
                assert_eq!(dimensions, expected);
            }
        )*
        }
    }

    get_thumb_dimensions_tests! {
        width_lt: ((100, 200), TargetDimension::MaxWidth(50),  ( 50, 100)),
        width_eq: ((100, 200), TargetDimension::MaxWidth(100), (100, 200)),
        width_gt: ((100, 200), TargetDimension::MaxWidth(200), (100, 200)),

        height_lt: ((100, 200), TargetDimension::MaxHeight(100), ( 50, 100)),
        height_eq: ((100, 200), TargetDimension::MaxHeight(200), (100, 200)),
        height_gt: ((100, 200), TargetDimension::MaxHeight(400), (100, 200)),

        // bounding box which is larger than the image in one or both
        // dimensions
        bbox_larger_w:  ((100, 200), TargetDimension::BoundingBox(500, 200), (100, 200)),
        bbox_larger_h:  ((100, 200), TargetDimension::BoundingBox(100, 500), (100, 200)),
        bbox_larger_wh: ((100, 200), TargetDimension::BoundingBox(500, 500), (100, 200)),

        // bounding box with an equal aspect ratio to the image
        bbox_equal_lt: ((100, 200), TargetDimension::BoundingBox( 50, 100), ( 50, 100)),
        bbox_equal_eq: ((100, 200), TargetDimension::BoundingBox(100, 200), (100, 200)),
        bbox_equal_gt: ((100, 200), TargetDimension::BoundingBox(200, 400), (100, 200)),

        // bounding box which is skinnier than the image
        bbox_skinnier_lt: ((100, 200), TargetDimension::BoundingBox(10, 100), (10, 20)),
        bbox_skinnier_eq: ((100, 200), TargetDimension::BoundingBox(20, 200), (20, 40)),
        bbox_skinnier_gt: ((100, 200), TargetDimension::BoundingBox(40, 400), (40, 80)),

        // bounding box which is wider than the image
        bbox_wider_lt: ((100, 200), TargetDimension::BoundingBox(100, 20), (10, 20)),
        bbox_wider_eq: ((100, 200), TargetDimension::BoundingBox(200, 40), (20, 40)),
        bbox_wider_gt: ((100, 200), TargetDimension::BoundingBox(400, 80), (40, 80)),

        // case to ensure we do floating point division correctly, and
        // aren't making rounding errors
        fp_width:  ((500, 333), TargetDimension::MaxWidth(300),  (300, 200)),
        fp_height: ((333, 500), TargetDimension::MaxHeight(300), (200, 300)),
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
