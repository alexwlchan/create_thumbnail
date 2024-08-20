# create_thumbnail

This is a simple command-line tool for creating image thumbnails.

You tell it your original image, the directory where you're storing thumbnails, and the max width/height of the thumbnail you want to create.
It prints the path to the newly-created thumbnail.

```console
$ create_thumbnail cat.jpg --out-dir=thumbnails --width=150
./thumbnails/cat.jpg

$ create_thumbnail dog.png --out-dir=thumbnails --height=200
./thumbnails/dog.png

$ create_thumbnail --help
Usage: create_thumbnail --out-dir <OUT_DIR> <--height <HEIGHT>|--width <WIDTH>> <PATH>
```

It supports JPEG, PNG, TIFF, WEBP, and both static and animated GIFs.
Thumbnails match the format of the original image, except for animated GIFs, which become MP4 movies.



## Installation

You can download compiled binaries from the [GitHub releases](https://github.com/alexwlchan/create_thumbnail/releases).

Alternatively, you can install from source.
You need Rust installed; I recommend using [Rustup].
Then clone this repository and compile the code:

```console
$ git clone "https://github.com/alexwlchan/create_thumbnail.git"
$ cd dominant_colours
$ cargo install --path .
```

For animated GIF support, you additionally need to install `ffmpeg`.

[Rustup]: https://rustup.rs/



## License

MIT.
