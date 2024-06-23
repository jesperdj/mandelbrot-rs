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

pub trait Filter {
    fn radius(&self) -> (f64, f64);

    fn evaluate(&self, x: f64, y: f64) -> f64;
}

pub struct BoxFilter {
    radius_x: f64,
    radius_y: f64,
}

pub struct MitchellFilter {
    radius_x: f64,
    radius_y: f64,
    p1: [f64; 4],
    p2: [f64; 4],
}

// ===== BoxFilter =============================================================================================================================================

impl BoxFilter {
    #[inline]
    pub fn new(radius_x: f64, radius_y: f64) -> BoxFilter {
        BoxFilter { radius_x, radius_y }
    }

    #[inline]
    pub fn with_defaults() -> BoxFilter {
        BoxFilter::new(0.5, 0.5)
    }
}

impl Filter for BoxFilter {
    #[inline]
    fn radius(&self) -> (f64, f64) {
        (self.radius_x, self.radius_y)
    }

    #[inline]
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        if x.abs() <= self.radius_x && y <= self.radius_y { 1.0 } else { 0.0 }
    }
}

// ===== MitchellFilter ========================================================================================================================================

impl MitchellFilter {
    pub fn new(radius_x: f64, radius_y: f64, b: f64, c: f64) -> MitchellFilter {
        let p1 = [1.0 - b / 3.0, 0.0, -3.0 + 2.0 * b + c, 2.0 - 1.5 * b - c];
        let p2 = [4.0 / 3.0 * b + 4.0 * c, -2.0 * b - 8.0 * c, b + 5.0 * c, -b / 6.0 - c];

        MitchellFilter { radius_x, radius_y, p1, p2 }
    }

    pub fn with_defaults() -> MitchellFilter {
        MitchellFilter::new(2.0, 2.0, 1.0 / 3.0, 1.0 / 3.0)
    }

    #[inline]
    fn mitchell(&self, v: f64) -> f64 {
        let x = 2.0 * v.abs();
        if x <= 1.0 {
            self.p1[0] + self.p1[1] * x + self.p1[2] * x * x + self.p1[3] * x * x * x
        } else if x <= 2.0 {
            self.p2[0] + self.p2[1] * x + self.p2[2] * x * x + self.p2[3] * x * x * x
        } else {
            0.0
        }
    }
}

impl Filter for MitchellFilter {
    #[inline]
    fn radius(&self) -> (f64, f64) {
        (self.radius_x, self.radius_y)
    }

    #[inline]
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.mitchell(x / self.radius_x) * self.mitchell(y / self.radius_y)
    }
}
