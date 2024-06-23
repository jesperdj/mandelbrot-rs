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

use std::ops::Range;

use image::Rgb;

use crate::math::clamp;
use crate::math::interpolate;

pub trait Palette {
    fn evaluate(&self, value: f64) -> Rgb<u8>;
}

pub struct Grayscale {
    range: Range<f64>,
    scale: f64,
}

pub struct Rainbow {
    range: Range<f64>,
    scale: f64,
}

pub struct Entry {
    value: f64,
    color: Rgb<u8>,
}

pub struct TablePalette {
    range: Range<f64>,
    entries: Vec<Entry>,
}

// ===== Grayscale =============================================================================================================================================

impl Grayscale {
    pub fn new(range: Range<f64>) -> Grayscale {
        let scale = (range.end - range.start) * 255.0;

        Grayscale { range, scale }
    }
}

impl Palette for Grayscale {
    fn evaluate(&self, value: f64) -> Rgb<u8> {
        if self.range.contains(&value) {
            let v = ((value - self.range.start) * self.scale).round() as u8;
            Rgb([v, v, v])
        } else {
            Rgb([0, 0, 0])
        }
    }
}

// ===== Rainbow ===============================================================================================================================================

impl Rainbow {
    pub fn new(range: Range<f64>) -> Rainbow {
        let scale = range.end - range.start;

        Rainbow { range, scale }
    }
}

impl Palette for Rainbow {
    fn evaluate(&self, value: f64) -> Rgb<u8> {
        let v = (value - self.range.start) * self.scale;
        if v < 0.0 {
            Rgb([0, 0, 0])
        } else if v < 0.2 {
            Rgb([0, (v * 1275.0).round() as u8, 255]) // 0.0..0.2: blue-cyan
        } else if v < 0.4 {
            Rgb([0, 255, ((0.4 - v) * 1275.0).round() as u8]) // 0.2..0.4: cyan-green
        } else if v < 0.6 {
            Rgb([((v - 0.4) * 1275.0).round() as u8, 255, 0]) // 0.4..0.6: green-yellow
        } else if v < 0.8 {
            Rgb([255, ((0.8 - v) * 1275.0).round() as u8, 0]) // 0.6..0.8: yellow-red
        } else if v < 1.0 {
            Rgb([255, 0, ((v - 0.8) * 1275.0).round() as u8]) // 0.8..1.0: red-magenta
        } else {
            Rgb([0, 0, 0])
        }
    }
}

// ===== Entry =================================================================================================================================================

impl Entry {
    pub fn new(value: f64, color: Rgb<u8>) -> Entry {
        Entry { value, color }
    }
}

// ===== TablePalette ==========================================================================================================================================

impl TablePalette {
    pub fn new(mut entries: Vec<Entry>) -> TablePalette {
        debug_assert!(!entries.is_empty(), "entries must not be empty");

        entries.sort_by(|first, second| first.value.partial_cmp(&second.value).unwrap());
        let range = entries.first().unwrap().value..entries.last().unwrap().value;

        TablePalette { range, entries }
    }
}

impl Palette for TablePalette {
    fn evaluate(&self, value: f64) -> Rgb<u8> {
        if self.range.contains(&value) {
            let mut index = 1;
            while value > self.entries[index].value {
                index += 1;
            }

            let left = &self.entries[index - 1];
            let right = &self.entries[index];
            let value = (value - left.value) / (right.value - left.value);

            let r = clamp(interpolate(value, left.color[0] as f64, right.color[0] as f64), 0.0, 255.0).round() as u8;
            let g = clamp(interpolate(value, left.color[1] as f64, right.color[1] as f64), 0.0, 255.0).round() as u8;
            let b = clamp(interpolate(value, left.color[2] as f64, right.color[2] as f64), 0.0, 255.0).round() as u8;

            Rgb([r, g, b])
        } else {
            Rgb([0, 0, 0])
        }
    }
}
