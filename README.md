# create_thumbnail

This is a simple command-line tool for creating image thumbnails.

It takes three arguments:

*   Your original image;
*   The directory where you're storing thumbnails;
*   The max allowed height or width of the thumbnail you want.
    You constrain in one dimension, and it resizes the image to fit, preserving the aspect ratio of the original image.

The tool prints the path to the newly-created thumbnail.
Here are two examples:

```console
$ create_thumbnail clever_cat.jpg --out-dir=thumbnails --width=100
./thumbnails/clever_cat.jpg

$ create_thumbnail dappy_dog.png --out-dir=thumbnails --height=250
./thumbnails/dappy_dog.png

$ create_thumbnail --help
Usage: create_thumbnail --out-dir <OUT_DIR> <--height <HEIGHT>|--width <WIDTH>> <PATH>
```

It supports JPEG, PNG, TIFF, WEBP, and both static and animated GIFs.
Thumbnails match the format of the original image, except for animated GIFs, which become MP4 movies.

This tool only does one thing: it creates thumbnails that I like.
I need image thumbnails in a lot of projects, and I wanted a single tool I could use in all of them rather than having multiple copies of the same code.

You might prefer to look at flexible tools like ImageMagick or ffmpeg, which have more customisation to fit a wider variety of use cases.




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
