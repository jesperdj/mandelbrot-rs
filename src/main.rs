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
use rayon::prelude::*;

use crate::palette::{Entry, Palette, TablePalette};
use crate::reconstruction::{Reconstructor, RendererOutput};
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

    let value_to_color = |value| match value {
        Some(value) => palette.evaluate(value),
        None => Rgb([0, 0, 0]),
    };

    let image = render_image(&sampler_factory, &renderer, &filter, &value_to_color, width, height);
    image.save("mandelbrot.png").unwrap();
}

/// A rendered sample kept for the reconstruction pass. The offset is the sub-pixel position within
/// the pixel it was generated in; combined with that pixel's coordinate it gives the sample's
/// location. Offsets are stored as `f32` to halve the memory footprint of the sample buffer.
struct StoredSample<R> {
    offset_x: f32,
    offset_y: f32,
    value: R,
}

fn render_image<SF, S, R, RR, F, M>(sampler_factory: &SF, renderer: &R, filter: &F, value_to_color: &M, width: u32, height: u32) -> RgbImage
where
    SF: Fn(u32, u32) -> S + Sync,
    S: Sampler,
    R: Renderer<Output=RR> + Sync,
    RR: RendererOutput,
    F: Filter + Sync,
    M: Fn(Option<RR>) -> Rgb<u8> + Sync,
{
    let start_time = Instant::now();
    let width = width as usize;
    let height = height as usize;

    // Pass 1: generate and render every sample, grouped per pixel. Only samples that produced a
    // value are kept (samples inside the set produce None), so interior regions store nothing.
    let samples: Vec<Vec<StoredSample<RR>>> = (0..width * height)
        .into_par_iter()
        .map(|index| {
            let x = (index % width) as u32;
            let y = (index / width) as u32;

            let mut pixel_samples = Vec::new();
            for sample in sampler_factory(x, y) {
                if let Some(value) = renderer.render(&sample) {
                    let (offset_x, offset_y) = sample.offset();
                    pixel_samples.push(StoredSample { offset_x: offset_x as f32, offset_y: offset_y as f32, value });
                }
            }
            pixel_samples
        })
        .collect();

    // Pass 2: reconstruct each pixel by gathering every sample within the filter's radius. Because
    // the filter can reach beyond the pixel, samples generated in neighboring pixels contribute
    // too. Each output pixel is written by exactly one task, so no synchronization is needed.
    let (radius_x, radius_y) = filter.radius();
    let mut raw = vec![0u8; width * height * 3];
    raw.par_chunks_exact_mut(3).enumerate().for_each(|(index, pixel)| {
        let x = index % width;
        let y = index / width;
        let center_x = x as f64 + 0.5;
        let center_y = y as f64 + 0.5;

        let x_lo = (center_x - radius_x).floor().max(0.0) as usize;
        let x_hi = ((center_x + radius_x).floor().max(0.0) as usize).min(width - 1);
        let y_lo = (center_y - radius_y).floor().max(0.0) as usize;
        let y_hi = ((center_y + radius_y).floor().max(0.0) as usize).min(height - 1);

        let mut reconstructor = Reconstructor::new(filter);
        for sy in y_lo..=y_hi {
            for sx in x_lo..=x_hi {
                for stored in &samples[sy * width + sx] {
                    let dx = sx as f64 + stored.offset_x as f64 - center_x;
                    let dy = sy as f64 + stored.offset_y as f64 - center_y;
                    reconstructor.accumulate(stored.value, dx, dy);
                }
            }
        }

        pixel.copy_from_slice(&value_to_color(reconstructor.value()).0);
    });
    let image = RgbImage::from_raw(width as u32, height as u32, raw).expect("raw buffer has the right size");

    let duration = Instant::now().duration_since(start_time).as_millis();
    println!("Rendering time: {} ms", duration);

    image
}
