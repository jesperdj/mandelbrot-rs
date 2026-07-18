# Mandelbrot fractal generator in Rust

A Mandelbrot generator in Rust. This uses sampling and reconstruction techniques for generating high-quality images.

The part of the Mandelbrot set to render, the image size, and the sampling, filtering and coloring
options are all controlled from the command line. Run `mandelbrot --help` for the full list.

## Compiling

Compile this with:

    cargo build --release

Note: Build a release build. Without the `--release` option, Rust will build a debug build, which will run about 10 times as slow as a release build.

Included in this project is a `.cargo/config.toml` file which sets the option `-C target-cpu=native`, so that the build will be specifically optimized for the CPU in the computer you're compiling on.

## Running

After compiling, run this with:

    ./target/release/mandelbrot

By default this does a quick render (one sample per pixel, box filter) to `mandelbrot.png`. Use the
options to change the view, quality and colors. For a high-quality render of the default view:

    ./target/release/mandelbrot --sampler stratified --samples 16 --filter mitchell

Some other examples:

    # A different location and zoom level
    ./target/release/mandelbrot --center-re -0.75 --center-im 0.0 --scale 2.5 --max-iterations 100

    # A smaller image with a rainbow palette
    ./target/release/mandelbrot --width 1920 --height 1080 --palette rainbow -o rainbow.png

Run `./target/release/mandelbrot --help` to see every option and its default.

A high-quality render of the default view looks like this:

![Mandelbrot detail](doc/mandelbrot.png)

### Palettes

Choose the palette with `--palette` (`table`, `grayscale` or `rainbow`). The `table` palette
interpolates between color stops; by default it uses a built-in set of stops. You can supply your
own stops in a TOML file with `--palette-file`:

    ./target/release/mandelbrot --palette table --palette-file palette.toml

See [`palette.toml`](palette.toml) for the file format: a list of stops, each mapping a normalized
iteration value (`0.0 ..= 1.0`) to an `#RRGGBB` color, interpolated linearly in between.
