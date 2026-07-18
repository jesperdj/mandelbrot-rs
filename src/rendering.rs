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

use crate::sampling::Sample;

pub mod mandelbrot;

pub trait Renderer {
    type Output;

    /// Renders a single sample. Returns `None` when the sample has no meaningful value (for the
    /// Mandelbrot renderer: when the point lies inside the set), so that reconstruction can skip it.
    fn render(&self, sample: &Sample) -> Option<Self::Output>;
}
