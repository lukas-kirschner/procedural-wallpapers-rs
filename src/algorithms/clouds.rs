use image::RgbImage;
use rand::Rng;
use crate::algorithms::Algorithm;
use crate::utils::perlin::Perlin;

fn sigmoid(x: f64) -> f64 {
    ((2.0 * x - 0.5).tanh() * 2.0).tanh()
}

pub struct Clouds {}

impl Algorithm for Clouds {
    fn build(&mut self, img: &mut RgbImage) -> Result<(), String> {
        let mut perlin = Perlin::new(img.width() as usize, img.height() as usize);
        perlin.regenerate_noise();
        let mut rng = rand::thread_rng();
        let freq: f64 = 0.002 * (rng.gen::<u8>() & 0xff) as f64 / 0xff as f64 + 0.003;
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let val: f64 = 0.5 * sigmoid(perlin.fractal(x as f64, y as f64, freq, 7)?) + 0.5;
            *pixel = image::Rgb([
                (val * 230.0) as u8 + 25,
                (val * 255.0) as u8,
                255
            ]);
        }
        Ok(())
    }
}