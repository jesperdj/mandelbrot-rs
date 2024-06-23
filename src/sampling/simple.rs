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

use crate::sampling::Sample;

pub struct SimpleSampler {
    pixel_x: u32,
    pixel_y: u32,
    fresh: bool,
}

// ===== SimpleSampler =========================================================================================================================================

impl SimpleSampler {
    #[inline]
    pub fn new(pixel_x: u32, pixel_y: u32) -> SimpleSampler {
        SimpleSampler { pixel_x, pixel_y, fresh: true }
    }
}

impl Iterator for SimpleSampler {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.fresh {
            self.fresh = false;
            Some(Sample::new(self.pixel_x, self.pixel_y, 0.5, 0.5))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.fresh { (1, Some(1)) } else { (0, Some(0)) }
    }
}

impl ExactSizeIterator for SimpleSampler {}

impl FusedIterator for SimpleSampler {}
