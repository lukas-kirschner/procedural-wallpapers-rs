use crate::algorithms::Algorithm;
use crate::utils::perlin::Perlin;
use image::{Rgb, RgbImage};
use rand::Rng;

#[derive(Default)]
pub struct Tangles {}

impl Tangles {
    /// Draw the given rectangle and shade the given base color with random perlin noise
    fn draw_rectangle(
        &self,
        rng: &mut impl Rng,
        img: &mut RgbImage,
        x0: usize,
        y0: usize,
        w: usize,
        h: usize,
        base_color: [u8; 3],
    ) -> Result<(), String> {
        let mut perlin = Perlin::new(w, h);
        perlin.regenerate_noise(rng);
        for x in x0..(x0 + w) {
            for y in y0..(y0 + h) {
                let noise_shade: f64 =
                    200.0 + (perlin.fractal((x - x0) as f64, (y - y0) as f64, 0.002, 6)? * 55.0);
                let pixel = img.get_pixel_mut(x as u32, y as u32);
                *pixel = Rgb([
                    (base_color[0] as f64 * (noise_shade / 255.0)) as u8,
                    (base_color[1] as f64 * (noise_shade / 255.0)) as u8,
                    (base_color[2] as f64 * (noise_shade / 255.0)) as u8,
                ]);
            }
        }
        Ok(())
    }
    /// Draw a rectangle at a random position with a maximum size of maxsize
    fn draw_random_rectangle(
        &self,
        rng: &mut impl Rng,
        img: &mut RgbImage,
        maxsize: usize,
        base_color: [u8; 3],
    ) -> Result<(), String> {
        let w: usize = rng.gen_range(0..maxsize);
        let h: usize = maxsize - w;
        let x0: usize = rng.gen_range(0..(img.width() as usize - w));
        let y0: usize = rng.gen_range(0..(img.height() as usize - h));
        self.draw_rectangle(rng, img, x0, y0, w, h, base_color)
    }
}

impl<R: Rng> Algorithm<R> for Tangles {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        // Set Background Color
        for (_, _, pixel) in img.enumerate_pixels_mut() {
            *pixel = Rgb([230, 230, 230]);
        }
        let mut base_r: u8 = 128 + rng.gen_range(0..128);
        let mut base_g: u8 = 128 + rng.gen_range(0..128);
        let mut base_b: u8 = 128 + rng.gen_range(0..128);
        let accent: u8 = 240 + rng.gen_range(0..16);
        match rng.gen_range(0..3) {
            0 => base_r = accent,
            1 => base_g = accent,
            2 => base_b = accent,
            _ => panic!("This code should never be reached!"),
        };
        let num_rectangles = (img.width() * img.height() / 80000).clamp(16, 255);
        for i in 0..num_rectangles {
            let maxsize = (img.height() - i * img.height() / 40)
                .try_into()
                .expect("maxsize out of bounds");
            let base_color = [
                base_r - i as u8 * 5,
                base_g - i as u8 * 5,
                base_b - i as u8 * 5,
            ];
            self.draw_random_rectangle(rng, img, maxsize, base_color)?;
        }
        Ok(())
    }
}
