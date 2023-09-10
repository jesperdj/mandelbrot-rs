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

use image::Rgb;

/// Palette; converts a f32 value into an RGB color.
pub trait Palette {
    fn evaluate(&self, t: f32) -> Rgb<u8>;
}

/// Entry in a TablePalette.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PaletteEntry {
    stop: f32,
    color: Rgb<u8>,
}

/// Palette consisting of a table of entries. Interpolates between the entry colors.
pub struct TablePalette {
    entries: Vec<PaletteEntry>,
}

/// Grayscale palette. Converts values in the range 0..1 into a grayscale color.
pub struct Grayscale;

/// Rainbow palette. Converts values in the range 0..1 into a color that varies from blue to cyan, green, yellow, red, magenta.
pub struct Rainbow;

// ===== PaletteEntry ==========================================================================================================================================

impl PaletteEntry {
    pub fn new(stop: f32, color: Rgb<u8>) -> PaletteEntry {
        PaletteEntry { stop, color }
    }
}

// ===== TablePalette ==========================================================================================================================================

impl TablePalette {
    pub fn new(mut entries: Vec<PaletteEntry>) -> TablePalette {
        debug_assert!(!entries.is_empty(), "entries must not be empty");
        entries.sort_by(|a, b| a.stop.partial_cmp(&b.stop).unwrap());
        TablePalette { entries }
    }
}

impl Palette for TablePalette {
    fn evaluate(&self, t: f32) -> Rgb<u8> {
        if t < self.entries[0].stop || t > self.entries[self.entries.len() - 1].stop {
            Rgb([0, 0, 0])
        } else if t == self.entries[self.entries.len() - 1].stop {
            self.entries[self.entries.len() - 1].color
        } else {
            let mut index = 1;
            while t > self.entries[index].stop {
                index += 1;
            }

            let left_stop = self.entries[index - 1].stop;
            let right_stop = self.entries[index].stop;

            // Translate and scale to interval 0..1
            let t = (t - left_stop) / (right_stop - left_stop);

            let left_color = self.entries[index - 1].color;
            let right_color = self.entries[index].color;

            // Interpolate and clamp color components
            let clamp = |t: f32, min: f32, max: f32| if t < min { min } else if t > max { max } else { t };
            let interpolate = |t: f32, left: u8, right: u8| clamp(left as f32 * (1.0 - t) + right as f32 * t, 0.0, 255.0) as u8;

            let r = interpolate(t, left_color[0], right_color[0]);
            let g = interpolate(t, left_color[1], right_color[1]);
            let b = interpolate(t, left_color[2], right_color[2]);

            Rgb([r, g, b])
        }
    }
}

// ===== Grayscale =============================================================================================================================================

impl Palette for Grayscale {
    fn evaluate(&self, t: f32) -> Rgb<u8> {
        if t >= 0.0 && t <= 1.0 {
            let v = (t * 255.0) as u8;
            Rgb([v, v, v])
        } else {
            Rgb([0, 0, 0])
        }
    }
}

// ===== Rainbow ===============================================================================================================================================

impl Palette for Rainbow {
    fn evaluate(&self, t: f32) -> Rgb<u8> {
        if t < 0.0 {
            Rgb([0, 0, 0])
        } else if t < 0.2 {
            // [0.0, 0.2): blue-cyan
            Rgb([0x00, (t * 1275.0) as u8, 0xFF])
        } else if t < 0.4 {
            // [0.2, 0.4): cyan-green
            Rgb([0x00, 0xFF, ((0.4 - t) * 1275.0) as u8])
        } else if t < 0.6 {
            // [0.4, 0.6): green-yellow
            Rgb([((t - 0.4) * 1275.0) as u8, 0xFF, 0x00])
        } else if t < 0.8 {
            // [0.6, 0.8): yellow-red
            Rgb([0xFF, ((0.8 - t) * 1275.0) as u8, 0x00])
        } else if t <= 1.0 {
            // [0.8, 1.0): red-magenta
            Rgb([0xFF, 0x00, ((t - 0.8) * 1275.0) as u8])
        } else {
            Rgb([0, 0, 0])
        }
    }
}
