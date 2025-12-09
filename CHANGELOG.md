# Changelog

## v1.1.1 - 2025-12-09

Pin the version of the image crate to 0.25.8; there's an issue with [artefacts in JPEG images](https://github.com/image-rs/image/issues/2688) in 0.25.9

## v1.1.0 - 2025-12-09

You can now pass `--width` and `--height` together, and the thumbnail will be the smallest image that fits inside that bounding box.

## v1.0.2 - 2025-09-08

Pay attention to EXIF orientation in input images, so thumbnails have the same rotation/reflection as the original.

## v1.0.1 - 2024-08-20

Fix a bug where the tool couldn't create thumbnails of animated GIFs if the thumbnail would have an odd width/height dimension.

## v1.0.0 - 2024-08-20

Initial release.
