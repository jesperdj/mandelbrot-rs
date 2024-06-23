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

use std::iter::FusedIterator;

use rand::random;

use crate::sampling::{Sample, Sampler};

pub struct StratifiedSampler {
    x: u32,
    y: u32,
    samples_per_pixel_x: u32,
    samples_per_pixel_y: u32,
    jitter: bool,
    index_x: u32,
    index_y: u32,
}

// ===== StratifiedSampler =====================================================================================================================================

impl StratifiedSampler {
    pub fn new(x: u32, y: u32, samples_per_pixel: u32, jitter: bool) -> StratifiedSampler {
        let samples_per_pixel_x = f32::sqrt(samples_per_pixel as f32).round() as u32;
        let samples_per_pixel_y = samples_per_pixel / samples_per_pixel_x;

        StratifiedSampler { x, y, samples_per_pixel_x, samples_per_pixel_y, jitter, index_x: 0, index_y: 0 }
    }
}

impl Sampler for StratifiedSampler {}

impl Iterator for StratifiedSampler {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_y < self.samples_per_pixel_y {
            let (jitter_x, jitter_y) = if self.jitter { random() } else { (0.5, 0.5) };
            let offset_x = (self.index_x as f64 + jitter_x) / self.samples_per_pixel_x as f64;
            let offset_y = (self.index_y as f64 + jitter_y) / self.samples_per_pixel_y as f64;
            let sample = Sample::new(self.x, self.y, offset_x, offset_y);

            self.index_x += 1;
            if self.index_x >= self.samples_per_pixel_x {
                self.index_x = 0;
                self.index_y += 1;
            }

            Some(sample)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.index_y < self.samples_per_pixel_y {
            let remaining_y = (self.samples_per_pixel_y - self.index_y - 1) as usize;
            let remaining_x = (self.samples_per_pixel_x - self.index_x) as usize;
            let remaining = remaining_y * self.samples_per_pixel_y as usize + remaining_x;

            (remaining, Some(remaining))
        } else {
            (0, Some(0))
        }
    }
}

impl ExactSizeIterator for StratifiedSampler {}

impl FusedIterator for StratifiedSampler {}
