//! Procedural Wallpapers in Rust - A collection of algorithms for procedural wallpaper generation.
//!
pub use rand_chacha::rand_core::SeedableRng;
pub use rand_chacha::ChaCha8Rng;
pub use image::{ImageBuffer, RgbImage};
pub use rand::Rng;

pub mod algorithms;
pub mod layers;
pub mod patterns;
pub mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
