// Tests that storage class deduction (from entry-point signature) fails correctly
// build-fail

use spirv_std::{spirv, Image};

#[spirv(vertex)]
pub fn main(
    #[spirv(uniform)] error: &Image!(2D, type=f32),
) {
}

// https://github.com/EmbarkStudios/rust-gpu/issues/585
#[spirv(vertex)]
pub fn issue_585(invalid: Image!(2D, type=f32)) {}
