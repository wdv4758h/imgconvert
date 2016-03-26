========================================
imgconvert
========================================

**This project is still in development**


Dependencies
========================================

* `kbknapp/clap-rs <https://github.com/kbknapp/clap-rs>`_
* `PistonDevelopers/image <https://github.com/PistonDevelopers/image>`_


Support
========================================

* resize image


.. code-block:: sh

    $ imgconvert --help
    Image Converter
    Chiu-Hsiang Hsu <wdv4758h@gmail.com>
    Convert image into other form

    USAGE:
        imgconvert [FLAGS] [OPTIONS] <IMAGE> <OUT>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -s, --size --size <SIZE>    image size e.g. 800x600

    ARGS:
        <IMAGE>    input image
        <OUT>      output file


Todo
========================================

* convert to text with VT100
* image rotation
* detect output image format
* grayscale
* crop
* blur
* flip
* thumbnail
* ...


License
========================================

This project is licensed under the Apache 2.0 license.
