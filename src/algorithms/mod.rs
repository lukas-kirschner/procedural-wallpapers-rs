use crate::algorithms::cellularone::CellularOne;
use crate::algorithms::clouds::Clouds;
use crate::algorithms::flow::Flow;
use crate::algorithms::islands::Islands;
use crate::algorithms::lightning::Lightning;
use crate::algorithms::nearestpoint::NearestPoint;
use crate::algorithms::squaresonedirection::SquaresOneDirection;
use crate::algorithms::tangles::Tangles;
use crate::Mode;
use image::RgbImage;
use rand::Rng;

mod cellularone;
mod clouds;
mod flow;
mod islands;
mod lightning;
mod nearestpoint;
mod squaresonedirection;
mod tangles;

/// This module contains all the image generation algorithms.

pub trait Algorithm<R: Rng> {
    /// Build an image using this algorithm
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String>;
}

impl Mode {
    pub fn to_algorithm<R: Rng>(self) -> Box<dyn Algorithm<R>> {
        match self {
            Mode::Clouds => Box::new(Clouds {}),
            Mode::Flow => Box::new(Flow::default()),
            Mode::Islands => Box::new(Islands::default()),
            Mode::Lightning => Box::new(Lightning::default()),
            Mode::NearestPoint => Box::new(NearestPoint::default()),
            Mode::Tangles => Box::new(Tangles::default()),
            Mode::CellularOne => Box::new(CellularOne::default()),
            Mode::Squares => Box::new(SquaresOneDirection::new_nodir()),
            Mode::SquaresHor => Box::new(SquaresOneDirection::new_horiz()),
            Mode::SquaresVer => Box::new(SquaresOneDirection::new_vert()),
            Mode::SquaresDiag => Box::new(SquaresOneDirection::new_diag()),
            Mode::Squares2 => Box::new(SquaresOneDirection::new_nodir_randomized()),
            Mode::Squares2H => Box::new(SquaresOneDirection::new_horiz_randomized()),
            Mode::Squares2V => Box::new(SquaresOneDirection::new_vert_randomized()),
            Mode::NearestGradient => Box::new(NearestPoint::new_soft()),
        }
    }
}
