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

use num_complex::Complex64;

use crate::rendering::Renderer;
use crate::sampling::Sample;

pub struct MandelbrotRenderer {
    max_iterations: u64,
    offset_re: f64,
    offset_im: f64,
    scale_re: f64,
    scale_im: f64,
}

// ===== MandelbrotRenderer ====================================================================================================================================

impl MandelbrotRenderer {
    pub fn new(center: Complex64, scale: f64, max_iterations: u64, width: u32, height: u32) -> MandelbrotRenderer {
        let aspect_ratio = width as f64 / height as f64;
        let (aspect_x, aspect_y) = if aspect_ratio >= 1.0 { (1.0, 1.0 / aspect_ratio) } else { (1.0 / aspect_ratio, 1.0) };

        let min_c = Complex64::new(center.re - scale * aspect_x, center.im - scale * aspect_y);
        let max_c = Complex64::new(center.re + scale * aspect_x, center.im + scale * aspect_y);

        let offset_re = min_c.re;
        let offset_im = max_c.im;

        let scale_re = (max_c.re - min_c.re) / width as f64;
        let scale_im = (max_c.im - min_c.im) / height as f64;

        MandelbrotRenderer { max_iterations, offset_re, offset_im, scale_re, scale_im }
    }
}

impl Renderer<f64> for MandelbrotRenderer {
    fn render(&self, sample: &Sample) -> f64 {
        let (x, y) = sample.location();
        let c = Complex64::new(self.offset_re + x * self.scale_re, self.offset_im - y * self.scale_im);

        let mut z = Complex64::new(0.0, 0.0);
        let mut i = 0u64;
        while z.norm_sqr() <= 4.0 && i < self.max_iterations {
            z = z * z + c;
            i += 1;
        }

        (i as f64 - z.norm().log2().log2()) / (self.max_iterations as f64)
    }
}
