use image::{RgbImage};
use rand::Rng;
use crate::algorithms::cellularone::CellularOne;
use crate::algorithms::clouds::Clouds;
use crate::algorithms::flow::Flow;
use crate::algorithms::islands::Islands;
use crate::algorithms::lightning::Lightning;
use crate::algorithms::nearestpoint::NearestPoint;
use crate::algorithms::squaresonedirection::SquaresOneDirection;
use crate::algorithms::tangles::Tangles;
use crate::Mode;

mod clouds;
mod flow;
mod islands;
mod lightning;
mod nearestpoint;
mod tangles;
mod cellularone;
mod squaresonedirection;

/// This module contains all the image generation algorithms.

pub trait Algorithm<R: Rng> {
    /// Build an image using this algorithm
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String>;
}

impl Mode {
    pub fn to_algorithm<R: Rng>(&self) -> Box<dyn Algorithm<R>> {
        match self {
            Mode::CLOUDS => { Box::new(Clouds {}) }
            Mode::FLOW => { Box::new(Flow::default()) }
            Mode::ISLANDS => { Box::new(Islands::default()) }
            Mode::LIGHTNING => { Box::new(Lightning::default()) }
            Mode::NEARESTPOINT => { Box::new(NearestPoint::default()) }
            Mode::TANGLES => { Box::new(Tangles::default()) }
            Mode::CELLULARONE => { Box::new(CellularOne::default()) }
            Mode::SQUARES => { Box::new(SquaresOneDirection::new_nodir()) }
            Mode::SQUARESHOR => { Box::new(SquaresOneDirection::new_horiz()) }
            Mode::SQUARESVER => { Box::new(SquaresOneDirection::new_vert()) }
            Mode::SQUARESDIAG => { Box::new(SquaresOneDirection::new_diag()) }
        }
    }
}