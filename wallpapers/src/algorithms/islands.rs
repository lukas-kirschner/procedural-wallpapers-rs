use crate::algorithms::Algorithm;
use crate::utils::perlin::Perlin;
use image::{Rgb, RgbImage};
use rand::Rng;

/// Islands
/// Based on the ISLANDS algorithm implementation by Attila Bagyoni, 2018
/// https://github.com/bagyoni/procedural-wallpapers
pub struct Islands {
    threshold_val_1: u8,
    threshold_val_2: u8,
    background_base_color: [u8; 3],
    border_color: [u8; 3],
    foreground_base_color: [u8; 3],
    dashed_grid_color: [u8; 3],
    grid_margins: usize,
}

impl Default for Islands {
    fn default() -> Self {
        Islands {
            threshold_val_1: 195,
            threshold_val_2: 190,
            background_base_color: [198, 151, 63],
            border_color: [154, 115, 82],
            foreground_base_color: [202, 168, 131],
            dashed_grid_color: [100, 96, 82],
            grid_margins: 10,
        }
    }
}

impl Islands {
    fn compute_threshold(
        &self,
        value: u8,
        replacement_1: u8,
        replacement_2: u8,
        replacement_3: u8,
    ) -> u8 {
        if value > self.threshold_val_1 {
            replacement_1
        } else if value > self.threshold_val_2 {
            replacement_2
        } else {
            replacement_3
        }
    }
    fn draw_horiz_dashed(&self, img: &mut RgbImage, y: usize) {
        for x in 0..img.width() {
            if x % 20 < 10 {
                let pixel = img.get_pixel_mut(x, y as u32);
                *pixel = Rgb(self.dashed_grid_color);
            }
        }
    }
    fn draw_vert_dashed(&self, img: &mut RgbImage, x: usize) {
        for y in 0..img.height() {
            if y % 20 < 10 {
                let pixel = img.get_pixel_mut(x as u32, y);
                *pixel = Rgb(self.dashed_grid_color);
            }
        }
    }
}

impl<R: Rng> Algorithm<R> for Islands {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        // Six grid squares for the longer dimension of the image
        let grid_size_in_px: usize = ((if img.width() > img.height() {
            img.height()
        } else {
            img.width()
        } as f64
            - (2.0 * self.grid_margins as f64))
            / 6.0) as usize;
        let mut perlin = Perlin::new(img.width() as usize, img.height() as usize);
        perlin.regenerate_noise(rng);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let val: u8 = (185.0 + perlin.fractal(x as f64, y as f64, 0.004, 8)? * 70.0) as u8;
            *pixel = Rgb([
                self.compute_threshold(
                    val,
                    (self.foreground_base_color[0] as f64 * val as f64 / 255.0) as u8,
                    self.border_color[0],
                    ((self.background_base_color[0] as u32 + val as u32) as f64 / 2.0) as u8,
                ),
                self.compute_threshold(
                    val,
                    (self.foreground_base_color[1] as f64 * val as f64 / 255.0) as u8,
                    self.border_color[1],
                    ((self.background_base_color[1] as u32 + val as u32) as f64 / 2.0) as u8,
                ),
                self.compute_threshold(
                    val,
                    (self.foreground_base_color[2] as f64 * val as f64 / 255.0) as u8,
                    self.border_color[2],
                    ((self.background_base_color[2] as u32 + val as u32) as f64 / 2.0) as u8,
                ),
            ]);
        }
        for y in (self.grid_margins..(img.height() as usize)).step_by(grid_size_in_px) {
            self.draw_horiz_dashed(img, y);
        }
        for x in (self.grid_margins..(img.width() as usize)).step_by(grid_size_in_px) {
            self.draw_vert_dashed(img, x);
        }
        Ok(())
    }
}
