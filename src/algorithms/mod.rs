use image::{RgbImage};
use crate::algorithms::clouds::Clouds;
use crate::algorithms::flow::Flow;
use crate::algorithms::islands::Islands;
use crate::algorithms::lightning::Lightning;
use crate::Mode;

mod clouds;
mod flow;
mod islands;
mod lightning;

/// This module contains all the image generation algorithms.

pub trait Algorithm {
    /// Build an image using this algorithm
    fn build(&mut self, img: &mut RgbImage) -> Result<(), String>;
}

impl Mode {
    pub fn to_algorithm(&self) -> Box<dyn Algorithm> {
        match self {
            Mode::CLOUDS => { Box::new(Clouds {}) }
            Mode::FLOW => { Box::new(Flow::default()) }
            Mode::ISLANDS => { Box::new(Islands::default()) }
            Mode::LIGHTNING => {Box::new(Lightning::default())}
        }
    }
}