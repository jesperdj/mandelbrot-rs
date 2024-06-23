// Copyright 2024 Jesper de Jong
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

use std::time::Instant;

use image::{Rgb, RgbImage};
use num_complex::Complex64;

use crate::palette::{Entry, Palette, TablePalette};
use crate::reconstruction::{Reconstructor, RenderResult};
use crate::reconstruction::filter::{Filter, MitchellFilter};
use crate::rendering::mandelbrot::MandelbrotRenderer;
use crate::rendering::Renderer;
use crate::sampling::Sampler;
use crate::sampling::stratified::StratifiedSampler;

mod palette;
mod math;
mod sampling;
mod reconstruction;
mod rendering;

fn main() {
    let width = 3840;
    let height = 2160;
    let samples_per_pixel = 16;

    let palette = TablePalette::new(vec![
        Entry::new(0.000, Rgb([0x00, 0x00, 0x66])),
        Entry::new(0.010, Rgb([0x19, 0x19, 0x19])),
        Entry::new(0.018, Rgb([0xFF, 0xFF, 0x4C])),
        Entry::new(0.022, Rgb([0x00, 0x66, 0x00])),
        Entry::new(0.040, Rgb([0xFF, 0xFF, 0xFF])),
        Entry::new(0.200, Rgb([0x00, 0x00, 0x99])),
        Entry::new(0.500, Rgb([0x00, 0x00, 0x00])),
        Entry::new(1.000, Rgb([0xFF, 0xFF, 0xFF])),
    ]);

    // let center = Complex64::new(-0.75f64, 0f64);
    // let scale = 2.5f64;
    // let max_iterations = 100usize;

    let center = Complex64::new(-0.743643, 0.131825);
    let scale = 0.00006;
    let max_iterations = 10_000;

    let sampler_factory = |x, y| StratifiedSampler::new(x, y, samples_per_pixel, true);
    // let sampler_factory = |x, y| SimpleSampler::new(x, y);
    let renderer = MandelbrotRenderer::new(center, scale, max_iterations, width, height);
    let filter = MitchellFilter::with_defaults();
    // let filter = BoxFilter::with_defaults();

    let result_to_color = |value| palette.evaluate(value);

    let image = render_image(&sampler_factory, &renderer, &filter, &result_to_color, width, height);
    image.save("mandelbrot.png").unwrap();
}

fn render_image<SF, S, R, RR, F, M>(sampler_factory: &SF, renderer: &R, filter: &F, result_to_color: &M, width: u32, height: u32) -> RgbImage
where
    SF: Fn(u32, u32) -> S + Sync,
    S: Sampler,
    R: Renderer<Result=RR> + Sync,
    RR: RenderResult,
    F: Filter + Sync,
    M: Fn(RR) -> Rgb<u8> + Sync,
{
    let start_time = Instant::now();
    let image = RgbImage::from_par_fn(width, height, |x, y| {
        let sampler = sampler_factory(x, y);
        let mut reconstructor = Reconstructor::new(filter);

        for sample in sampler {
            let result = renderer.render(&sample);
            reconstructor.accumulate(&sample, result);
        }

        result_to_color(reconstructor.value())
    });
    let duration = Instant::now().duration_since(start_time).as_millis();
    println!("Rendering time: {} ms", duration);

    image
}
