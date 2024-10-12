use image::RgbImage;

pub mod squareslayer;

pub trait Layer {
    /// Draw the layer onto an image.
    /// How the pixels are modified and what kind of values need to be set before drawing depends
    /// entirely on the Layer type!
    fn draw(&self, img: &mut RgbImage) -> Result<(), String>;
    // fn set_mix_mode(&mut self, mode: MixMode);
    fn get_mix_mode(&self) -> MixMode;
}

#[derive(Copy, Clone)]
pub enum MixMode {
    /// Normal Mix Mode, replaces the pixel values with the new color.
    Normal,
}
