create-thumbnail PATH [--width=WIDTH | --height=HEIGHT] --out-dir=OUT_DIR

focusing on a small piece of code makes it better

* pull out dimension choosing
    -> test that code
        image which is smaller in width/height
        rounding errors?

* CLI:
    -> version
    -> help
    -> width + height
    -> neither of width/height
    -> width only
    -> height only

* happy path:
    -> animated GIF
    -> static GIF
    -> PNG
    -> JPEG
    -> TIF
    -> small file

* errors:
    -> creates thumbnail directory
    -> /dev/null
    -> thumbnail dir is file?
    -> try to overwrite original file?
    -> non-image format
    -> unrecognised image format
    -> non-existent file

* print name of thumbnail