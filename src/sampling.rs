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

pub mod simple;
pub mod stratified;

pub struct Sample {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub offset_x: f64,
    pub offset_y: f64,
}

pub trait Sampler: Iterator<Item=Sample> {}

// ===== Sample ================================================================================================================================================

impl Sample {
    #[inline]
    pub fn new(pixel_x: u32, pixel_y: u32, offset_x: f64, offset_y: f64) -> Sample {
        Sample { pixel_x, pixel_y, offset_x, offset_y }
    }

    #[inline]
    pub fn location(&self) -> (f64, f64) {
        (self.pixel_x as f64 + self.offset_x, self.pixel_y as f64 + self.offset_y)
    }
}

// ===== Sampler ===============================================================================================================================================

impl<T: Iterator<Item=Sample>> Sampler for T {}
