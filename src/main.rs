// Copyright 2020 Jesper de Jong
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use image::{Rgb, RgbImage};
use num_complex::Complex64;
use renderbase::filter::MitchellFilter;
use renderbase::renderer::render;
use renderbase::sampler::StratifiedSampler;

use crate::mandelbrot::MandelbrotRenderFunction;
use crate::palette::{Palette, PaletteEntry, TablePalette};

mod mandelbrot;
mod palette;

fn main() {
    // Initialize logging
    env_logger::builder().format_timestamp_micros().init();

    let (width, height) = (3840, 2160);
    let oversampling = 4;
    log::info!("Size:                    {} x {}", width, height);
    log::info!("Samples per pixel:       {}", oversampling * oversampling);
    log::info!("Total number of samples: {}", width as usize * oversampling as usize * height as usize * oversampling as usize);

    // Setup sampler, render function and filter
    let sampler = StratifiedSampler::new(width * oversampling, height * oversampling, true);
    let render_fn = MandelbrotRenderFunction::new(Complex64::new(-0.743643, 0.131825), 0.00006, 10_000, width as f64 / height as f64);
    let filter = MitchellFilter::with_defaults();

    // Render a raster using the sampler, render function and filter
    let raster = render(&sampler, &render_fn, &filter, width, height);

    let palette = TablePalette::new(vec![
        PaletteEntry::new(0.000, Rgb([0x00, 0x00, 0x66])),
        PaletteEntry::new(0.010, Rgb([0x19, 0x19, 0x19])),
        PaletteEntry::new(0.018, Rgb([0xFF, 0xFF, 0x4C])),
        PaletteEntry::new(0.022, Rgb([0x00, 0x66, 0x00])),
        PaletteEntry::new(0.040, Rgb([0xFF, 0xFF, 0xFF])),
        PaletteEntry::new(0.200, Rgb([0x00, 0x00, 0x99])),
        PaletteEntry::new(0.500, Rgb([0x00, 0x00, 0x00])),
        PaletteEntry::new(1.000, Rgb([0xFF, 0xFF, 0xFF])),
    ]);

    // Convert the raster into an image
    let mut image = RgbImage::new(width, height);
    for (x, y) in raster.rectangle().index_iter() {
        let value = raster.get(x, y);
        image.put_pixel(x as u32, y as u32, palette.evaluate(value));
    }

    image.save("mandelbrot.png").unwrap();
}
