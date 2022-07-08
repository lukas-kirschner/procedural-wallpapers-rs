use image::RgbImage;
use crate::algorithms::Algorithm;

pub struct Clouds{}
impl Algorithm for Clouds{
    fn build(&self, img: &mut RgbImage) -> Result<(), String> {
        todo!()
    }
}