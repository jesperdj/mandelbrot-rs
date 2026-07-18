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

use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::{Parser, ValueEnum};
use image::{Rgb, RgbImage};
use num_complex::Complex64;
use rayon::prelude::*;
use serde::Deserialize;

use crate::palette::{Entry, Grayscale, Palette, Rainbow, TablePalette};
use crate::reconstruction::{Reconstructor, RendererOutput};
use crate::reconstruction::filter::{BoxFilter, Filter, MitchellFilter};
use crate::rendering::mandelbrot::MandelbrotRenderer;
use crate::rendering::Renderer;
use crate::sampling::Sampler;
use crate::sampling::simple::SimpleSampler;
use crate::sampling::stratified::StratifiedSampler;

mod palette;
mod math;
mod sampling;
mod reconstruction;
mod rendering;

/// Mandelbrot fractal generator using sampling and reconstruction.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Image width in pixels.
    #[arg(long, default_value_t = 1920)]
    width: u32,

    /// Image height in pixels.
    #[arg(long, default_value_t = 1080)]
    height: u32,

    /// Real part of the complex number at the center of the view.
    #[arg(long, allow_hyphen_values = true, default_value_t = -0.75)]
    center_re: f64,

    /// Imaginary part of the complex number at the center of the view.
    #[arg(long, allow_hyphen_values = true, default_value_t = 0.0)]
    center_im: f64,

    /// Half of the smaller extent of the view around the center, in the complex plane.
    #[arg(long, default_value_t = 2.5)]
    scale: f64,

    /// Maximum number of iterations before a point is considered inside the set.
    #[arg(long, default_value_t = 100)]
    max_iterations: u64,

    /// Sampler that places samples within each pixel.
    #[arg(long, value_enum, default_value = "simple")]
    sampler: SamplerKind,

    /// Number of samples per pixel. Only used by the stratified sampler; must be a perfect square.
    #[arg(long, default_value_t = 16)]
    samples: u32,

    /// Reconstruction filter that weights samples.
    #[arg(long, value_enum, default_value = "box")]
    filter: FilterKind,

    /// Palette that maps iteration values to colors.
    #[arg(long, value_enum, default_value = "rainbow")]
    palette: PaletteKind,

    /// TOML file with the color stops for the table palette. If omitted, a built-in default is used.
    #[arg(long)]
    palette_file: Option<PathBuf>,

    /// Path of the output PNG image.
    #[arg(short, long, default_value = "mandelbrot.png")]
    output: PathBuf,
}

#[derive(Clone, Copy, ValueEnum)]
enum SamplerKind {
    /// A single sample at the center of each pixel (fast, no anti-aliasing).
    Simple,
    /// A jittered grid of samples per pixel (see --samples).
    Stratified,
}

#[derive(Clone, Copy, ValueEnum)]
enum FilterKind {
    /// Box filter: samples within the pixel are weighted equally (fast).
    Box,
    /// Mitchell-Netravali filter: higher quality; also gathers samples from neighboring pixels.
    Mitchell,
}

#[derive(Clone, Copy, ValueEnum)]
enum PaletteKind {
    /// Colors interpolated between configurable stops (see --palette-file).
    Table,
    /// Shades of gray.
    Grayscale,
    /// A rainbow of colors.
    Rainbow,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Validate up front for a clean error, rather than letting StratifiedSampler's assertion panic
    // inside a worker thread once rendering has started.
    if matches!(args.sampler, SamplerKind::Stratified) {
        let side = (args.samples as f64).sqrt().round() as u32;
        if side * side != args.samples {
            return Err(format!("--samples must be a perfect square for the stratified sampler, got {}", args.samples).into());
        }
    }

    let palette = build_palette(args.palette, args.palette_file.as_deref())?;
    let value_to_color = |value| match value {
        Some(value) => palette.evaluate(value),
        None => Rgb([0, 0, 0]),
    };

    let center = Complex64::new(args.center_re, args.center_im);
    let renderer = MandelbrotRenderer::new(center, args.scale, args.max_iterations, args.width, args.height);

    // Select the filter and sampler at runtime, but keep them statically dispatched: each match arm
    // instantiates render_image with concrete types, so there are no virtual calls in the hot loop.
    let image = match args.filter {
        FilterKind::Box => {
            let filter = BoxFilter::with_defaults();
            render_with_sampler(args.sampler, args.samples, &renderer, &filter, &value_to_color, args.width, args.height)
        }
        FilterKind::Mitchell => {
            let filter = MitchellFilter::with_defaults();
            render_with_sampler(args.sampler, args.samples, &renderer, &filter, &value_to_color, args.width, args.height)
        }
    };

    image.save(&args.output)?;
    Ok(())
}

// ===== Palette construction ==================================================================================================================================

fn build_palette(kind: PaletteKind, palette_file: Option<&Path>) -> Result<Box<dyn Palette + Sync>, Box<dyn Error>> {
    let palette: Box<dyn Palette + Sync> = match kind {
        PaletteKind::Table => {
            let entries = match palette_file {
                Some(path) => load_table_entries(path)?,
                None => default_table_entries(),
            };
            Box::new(TablePalette::new(entries))
        }
        PaletteKind::Grayscale => Box::new(Grayscale::new(0.0..1.0)),
        PaletteKind::Rainbow => Box::new(Rainbow::new(0.0..1.0)),
    };
    Ok(palette)
}

fn default_table_entries() -> Vec<Entry> {
    vec![
        Entry::new(0.000, Rgb([0x00, 0x00, 0x66])),
        Entry::new(0.010, Rgb([0x19, 0x19, 0x19])),
        Entry::new(0.018, Rgb([0xFF, 0xFF, 0x4C])),
        Entry::new(0.022, Rgb([0x00, 0x66, 0x00])),
        Entry::new(0.040, Rgb([0xFF, 0xFF, 0xFF])),
        Entry::new(0.200, Rgb([0x00, 0x00, 0x99])),
        Entry::new(0.500, Rgb([0x00, 0x00, 0x00])),
        Entry::new(1.000, Rgb([0xFF, 0xFF, 0xFF])),
    ]
}

/// Deserialized form of a palette file: a list of `[[stops]]` tables, each with a `value` and a
/// `#RRGGBB` `color`.
#[derive(Deserialize)]
struct PaletteFile {
    stops: Vec<StopEntry>,
}

#[derive(Deserialize)]
struct StopEntry {
    value: f64,
    color: String,
}

fn load_table_entries(path: &Path) -> Result<Vec<Entry>, Box<dyn Error>> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read palette file {}: {e}", path.display()))?;
    let file: PaletteFile = toml::from_str(&text).map_err(|e| format!("cannot parse palette file {}: {e}", path.display()))?;
    file.stops.into_iter().map(|stop| Ok(Entry::new(stop.value, parse_hex_color(&stop.color)?))).collect()
}

fn parse_hex_color(color: &str) -> Result<Rgb<u8>, Box<dyn Error>> {
    let hex = color.strip_prefix('#').unwrap_or(color);
    let invalid = || format!("invalid color '{color}', expected #RRGGBB");
    if hex.len() != 6 {
        return Err(invalid().into());
    }
    let component = |range: std::ops::Range<usize>| u8::from_str_radix(&hex[range], 16).map_err(|_| invalid());
    Ok(Rgb([component(0..2)?, component(2..4)?, component(4..6)?]))
}

// ===== Rendering =============================================================================================================================================

fn render_with_sampler<R, RR, F, M>(sampler: SamplerKind, samples: u32, renderer: &R, filter: &F, value_to_color: &M, width: u32, height: u32) -> RgbImage
where
    R: Renderer<Output=RR> + Sync,
    RR: RendererOutput,
    F: Filter + Sync,
    M: Fn(Option<RR>) -> Rgb<u8> + Sync,
{
    match sampler {
        SamplerKind::Simple => render_image(&|x, y| SimpleSampler::new(x, y), renderer, filter, value_to_color, width, height),
        SamplerKind::Stratified => render_image(&|x, y| StratifiedSampler::new(x, y, samples, true), renderer, filter, value_to_color, width, height),
    }
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
    let image = RgbImage::from_par_fn(width as u32, height as u32, |x, y| {
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

        value_to_color(reconstructor.value())
    });
    let duration = Instant::now().duration_since(start_time).as_millis();
    println!("Rendering time: {} ms", duration);

    image
}
