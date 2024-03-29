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

#![allow(dead_code)]

use image::{Rgb, RgbImage};
use num_complex::Complex64;
use renderbase::filter::MitchellFilter;
use renderbase::rectangle::Rectangle;
use renderbase::renderer::{MultiThreadedRenderer, Renderer};
use renderbase::sampler::StratifiedSampler;

// use crate::julia::JuliaRenderFunction;
use crate::mandelbrot::MandelbrotRenderFunction;
use crate::palette::{Palette, PaletteEntry, TablePalette};

mod mandelbrot;
mod julia;
mod palette;

fn main() {
    // Initialize logging
    env_logger::builder().format_timestamp_micros().init();

    let (width, height) = (3840, 2160);
    let samples_per_pixel = 16;
    let jitter = true;
    log::info!("Size:                    {} x {}", width, height);
    log::info!("Samples per pixel:       {}", samples_per_pixel);
    log::info!("Total number of samples: {}", width as usize * height as usize * samples_per_pixel as usize);

    // Mandelbrot parameters
    let center = Complex64::new(-0.743643, 0.131825);
    let scale = 0.00006;
    let max_iterations = 10_000;

    // Create renderer
    let renderer = MultiThreadedRenderer::with_defaults();
    // let renderer = SimpleRenderer::new();

    // Setup sampler, render function and filter
    let sampler = StratifiedSampler::new(Rectangle::new(0, 0, width, height), f64::sqrt(samples_per_pixel as f64) as u32, jitter);
    // let sampler = IndependentSampler::new(Rectangle::new(0, 0, width, height), samples_per_pixel, jitter);
    let render_fn = MandelbrotRenderFunction::new(center, scale, max_iterations, width, height);
    // let render_fn = JuliaRenderFunction::new(Complex64::new(-0.4, -0.59), Complex64::new(0.0, 0.0), 2.0, 10_000, width, height);
    let filter = MitchellFilter::with_defaults();

    // Render a raster using the sampler, render function and filter
    let raster = renderer.render(&sampler, &render_fn, &filter);

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
        image.put_pixel(x, y, palette.evaluate(raster.get(x, y)));
    }

    image.save("mandelbrot.png").unwrap();
}
