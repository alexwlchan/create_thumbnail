create-thumbnail PATH [--width=WIDTH | --height=HEIGHT] --out-dir=OUT_DIR

focusing on a small piece of code makes it better

* CLI:
    -> width only
    -> height only

* happy path:
    -> small file
    -> test dimensions

* errors:
    -> creates thumbnail directory
    -> /dev/null
    -> thumbnail dir is file?
    -> try to overwrite original file?
    -> non-image format
    -> unrecognised image format
    -> non-existent file

* print name of thumbnail