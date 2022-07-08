use image::{RgbImage};
use crate::algorithms::clouds::Clouds;
use crate::Mode;

mod clouds;

/// This module contains all the image generation algorithms.

pub trait Algorithm {
    /// Build an image using this algorithm
    fn build(&self, img: &mut RgbImage) -> Result<(), String>;
}

impl Mode {
    pub fn to_algorithm(&self) -> Box<dyn Algorithm> {
        match self {
            Mode::CLOUDS => { Box::new(Clouds {}) }
        }
    }
}