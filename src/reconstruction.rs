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

use std::ops::{AddAssign, Div, Mul};

use crate::reconstruction::filter::Filter;

pub mod filter;

pub trait RendererOutput: Copy + Default + AddAssign + Mul<f64, Output=Self> + Div<f64, Output=Self> + Send + Sync {}

pub struct Reconstructor<'a, R, F>
where
    R: RendererOutput,
    F: Filter,
{
    accumulator: R,
    total_weight: f64,
    filter: &'a F,
}

// ===== RendererOutput ========================================================================================================================================

impl<T: Copy + Default + AddAssign + Mul<f64, Output=Self> + Div<f64, Output=Self> + Send + Sync> RendererOutput for T {}

// ===== Reconstructor =========================================================================================================================================

impl<'a, R, F> Reconstructor<'a, R, F>
where
    R: RendererOutput,
    F: Filter,
{
    #[inline]
    pub fn new(filter: &'a F) -> Reconstructor<'a, R, F> {
        Reconstructor { accumulator: R::default(), total_weight: 0.0, filter }
    }

    /// Accumulates a sample value with a weight determined by the reconstruction filter. `dx` and
    /// `dy` are the sample's offset from the center of the pixel being reconstructed, measured in
    /// pixels; the sample may originate from a neighboring pixel, so they are not restricted to
    /// `[-0.5, 0.5]`.
    #[inline]
    pub fn accumulate(&mut self, value: R, dx: f64, dy: f64) {
        let weight = self.filter.evaluate(dx, dy);
        self.accumulator += value * weight;
        self.total_weight += weight;
    }

    /// Returns the filtered value, or `None` when no sample contributed any weight (for the
    /// Mandelbrot renderer this means every nearby sample was inside the set).
    #[inline]
    pub fn value(self) -> Option<R> {
        if self.total_weight != 0.0 { Some(self.accumulator / self.total_weight) } else { None }
    }
}
