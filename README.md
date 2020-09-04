# Mandelbrot and Julia fractal renderer using RenderBase

A small demo application that shows how to use [RenderBase](https://github.com/jesperdj/renderbase-rs) for high-quality, multi-threaded rendering.

## Compiling

Compile this with:

    cargo build --release

Note: Build a release build. Without the `--release` option, Rust will build a debug build, which will run more than 10 times as slow as a release build.

Included in this project is a `.cargo/config.toml` file which sets the option `-C target-cpu=native`, so that the build will be specifically optimized for the
CPU in the computer you're compiling on. This is important to enable the use of SIMD instruction sets such as AVX on Intel x86-64 CPU's.

## Running

Run this with, for example:

    RUST_LOG=debug /usr/bin/time -f '%MK %E' ./target/release/mandelbrot

The `RUST_LOG` environment variable configures logging. If this is not set to a logging level, you will not see any output.

The above command will produce output that looks like the following. The number of worker threads depends on the number of cores your CPU has.

```
[2020-09-04T21:44:43.046932Z INFO  mandelbrot] Size:                    3840 x 2160
[2020-09-04T21:44:43.046976Z INFO  mandelbrot] Samples per pixel:       16
[2020-09-04T21:44:43.046986Z INFO  mandelbrot] Total number of samples: 132710400
[2020-09-04T21:44:43.048052Z INFO  renderbase::renderer] [01] Worker thread started
[2020-09-04T21:44:43.048229Z INFO  renderbase::renderer] Sample generator thread started
[2020-09-04T21:44:43.048369Z INFO  renderbase::renderer] [04] Worker thread started
[2020-09-04T21:44:43.048560Z INFO  renderbase::renderer] [03] Worker thread started
[2020-09-04T21:44:43.048663Z INFO  renderbase::renderer] [02] Worker thread started
[2020-09-04T21:44:43.048839Z INFO  renderbase::renderer] [05] Worker thread started
[2020-09-04T21:44:43.048974Z INFO  renderbase::renderer] [06] Worker thread started
[2020-09-04T21:44:43.050259Z INFO  renderbase::renderer] [07] Worker thread started
[2020-09-04T21:44:43.050901Z INFO  renderbase::renderer] [08] Worker thread started
[2020-09-04T21:44:43.051690Z INFO  renderbase::renderer] [10] Worker thread started
[2020-09-04T21:44:43.051992Z INFO  renderbase::renderer] [11] Worker thread started
[2020-09-04T21:44:43.052094Z INFO  renderbase::renderer] [09] Worker thread started
[2020-09-04T21:44:43.052137Z INFO  renderbase::renderer] Sample generator thread finished, generated 299 tiles, run time: 3 ms
[2020-09-04T21:44:43.052250Z INFO  renderbase::renderer] Aggregating results
[2020-09-04T21:44:43.052477Z INFO  renderbase::renderer] [12] Worker thread started
[2020-09-04T21:44:52.280461Z INFO  renderbase::renderer] [03] Worker thread finished, processed 26 tiles; 11535740 samples, run time: 9231 ms
[2020-09-04T21:44:52.322544Z INFO  renderbase::renderer] [11] Worker thread finished, processed 24 tiles; 10649280 samples, run time: 9270 ms
[2020-09-04T21:44:52.365342Z INFO  renderbase::renderer] [02] Worker thread finished, processed 26 tiles; 11537720 samples, run time: 9316 ms
[2020-09-04T21:44:52.411457Z INFO  renderbase::renderer] [06] Worker thread finished, processed 26 tiles; 11543040 samples, run time: 9362 ms
[2020-09-04T21:44:52.416966Z INFO  renderbase::renderer] [04] Worker thread finished, processed 26 tiles; 11543040 samples, run time: 9368 ms
[2020-09-04T21:44:52.437293Z INFO  renderbase::renderer] [10] Worker thread finished, processed 28 tiles; 12428820 samples, run time: 9385 ms
[2020-09-04T21:44:52.449063Z INFO  renderbase::renderer] [05] Worker thread finished, processed 27 tiles; 11987260 samples, run time: 9400 ms
[2020-09-04T21:44:52.466694Z INFO  renderbase::renderer] [07] Worker thread finished, processed 24 tiles; 10651940 samples, run time: 9416 ms
[2020-09-04T21:44:52.482216Z INFO  renderbase::renderer] [12] Worker thread finished, processed 27 tiles; 11987260 samples, run time: 9429 ms
[2020-09-04T21:44:52.495345Z INFO  renderbase::renderer] [08] Worker thread finished, processed 25 tiles; 11093500 samples, run time: 9444 ms
[2020-09-04T21:44:52.523396Z INFO  renderbase::renderer] [01] Worker thread finished, processed 15 tiles; 6653980 samples, run time: 9475 ms
[2020-09-04T21:44:52.525313Z INFO  renderbase::renderer] [09] Worker thread finished, processed 25 tiles; 11098820 samples, run time: 9473 ms
[2020-09-04T21:44:52.525524Z INFO  renderbase::renderer] Converting raster
[2020-09-04T21:44:52.546122Z INFO  renderbase::renderer] Rendering finished, run time: 9498 ms
105072K 0:09.84
```
