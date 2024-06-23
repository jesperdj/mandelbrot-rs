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
use crate::sampling::Sample;

pub mod filter;

pub trait RenderResult: Copy + Default + AddAssign + Mul<f64, Output=Self> + Div<f64, Output=Self> {}

pub struct Reconstructor<'a, R, F>
where
    R: RenderResult,
    F: Filter,
{
    accumulator: R,
    total_weight: f64,
    filter: &'a F,
}

// ===== RenderResult ==========================================================================================================================================

impl<T: Copy + Default + AddAssign + Mul<f64, Output=Self> + Div<f64, Output=Self>> RenderResult for T {}

// ===== Reconstructor =========================================================================================================================================

impl<'a, R, F> Reconstructor<'a, R, F>
where
    R: RenderResult,
    F: Filter,
{
    #[inline]
    pub fn new(filter: &'a F) -> Reconstructor<R, F> {
        Reconstructor { accumulator: R::default(), total_weight: 0.0, filter }
    }

    #[inline]
    pub fn accumulate(&mut self, sample: &Sample, result: R) {
        let (offset_x, offset_y) = sample.offset();
        let weight = self.filter.evaluate(offset_x - 0.5, offset_y - 0.5);
        self.accumulator += result * weight;
        self.total_weight += weight;
    }

    #[inline]
    pub fn value(self) -> R {
        if self.total_weight != 0.0 { self.accumulator / self.total_weight } else { R::default() }
    }
}
