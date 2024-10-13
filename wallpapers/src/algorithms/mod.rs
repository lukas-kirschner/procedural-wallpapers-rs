use image::RgbImage;
use rand::Rng;

mod cellularone;
mod clouds;
mod flow;
mod islands;
mod lightning;
mod nearestpoint;
mod randompatterns;
mod squaresonedirection;
mod tangles;
// Exports of Algorithms:
pub use cellularone::CellularOne;
pub use clouds::Clouds;
pub use flow::Flow;
pub use islands::Islands;
pub use lightning::Lightning;
pub use nearestpoint::NearestPoint;
pub use randompatterns::RandomPatterns;
pub use squaresonedirection::SquaresOneDirection;
pub use tangles::Tangles;

/// This module contains all the image generation algorithms.

pub trait Algorithm<R: Rng> {
    /// Build an image using this algorithm
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String>;
}
