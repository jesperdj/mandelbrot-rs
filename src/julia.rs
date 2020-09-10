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

use num_complex::Complex64;
use renderbase::renderer::RenderFunction;
use renderbase::sampler::PixelSample;

/// Julia fractal render function.
pub struct JuliaRenderFunction {
    c: Complex64,
    offset_re: f64,
    offset_im: f64,
    scale_re: f64,
    scale_im: f64,
    max_iterations: usize,
}

// ===== JuliaRenderFunction ===================================================================================================================================

impl JuliaRenderFunction {
    pub fn new(c: Complex64, center: Complex64, scale: f64, max_iterations: usize, width: u32, height: u32) -> JuliaRenderFunction {
        let aspect_ratio = width as f64 / height as f64;
        let (aspect_x, aspect_y) = if aspect_ratio >= 1.0 {
            (1.0, 1.0 / aspect_ratio)
        } else {
            (1.0 / aspect_ratio, 1.0)
        };

        let min_z = Complex64::new(center.re - scale * aspect_x, center.im - scale * aspect_y);
        let max_z = Complex64::new(center.re + scale * aspect_x, center.im + scale * aspect_y);

        let offset_re = min_z.re;
        let offset_im = max_z.im;

        let scale_re = (max_z.re - min_z.re) / width as f64;
        let scale_im = (max_z.im - min_z.im) / height as f64;

        JuliaRenderFunction { c, offset_re, offset_im, scale_re, scale_im, max_iterations }
    }
}

impl RenderFunction for JuliaRenderFunction {
    type Value = f32;

    fn evaluate(&self, sample: &PixelSample) -> f32 {
        let mut z = Complex64::new(
            self.offset_re + sample.pixel_x as f64 * self.scale_re,
            self.offset_im - sample.pixel_y as f64 * self.scale_im,
        );

        let mut i = 0;

        while z.norm_sqr() <= 4.0 && i < self.max_iterations {
            z = z * z + self.c;
            i += 1;
        }

        if i < self.max_iterations {
            ((i as f64 - z.norm().log2().log2()) / (self.max_iterations as f64)) as f32
        } else {
            -1.0
        }
    }
}
