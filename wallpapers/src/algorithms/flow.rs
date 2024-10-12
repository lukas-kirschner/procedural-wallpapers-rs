use crate::utils::perlin::Perlin;
use image::{Rgb, RgbImage};
use rand::Rng;
use std::f64::consts::PI;
use crate::algorithms::Algorithm;

/// Flow
/// Based on the FLOW algorithm implementation by Attila Bagyoni, 2018
/// https://github.com/bagyoni/procedural-wallpapers

pub struct Flow {
    num_particles: u32,
    path_len: u32,
    curvature: f64,
    frequency: f64,
    signum: bool,
}

impl Default for Flow {
    fn default() -> Self {
        Flow {
            num_particles: 5000,
            path_len: 300,
            curvature: 0.0,
            frequency: 0.0,
            signum: true,
        }
    }
}

impl Flow {
    /// Get the signum as integer
    fn signum(&self) -> i8 {
        match self.signum {
            true => 1,
            false => -1,
        }
    }
    fn make_single_path(
        &self,
        rng: &mut impl Rng,
        perlin: &Perlin,
        flow: &mut [Vec<f64>],
        img: &RgbImage,
    ) -> Result<(), String> {
        let mut x: f64 = rng.gen_range(0..img.width()) as f64;
        let mut y: f64 = rng.gen_range(0..img.height()) as f64;
        let mut i: u32 = 0;
        while i < self.path_len
            && x > 0.0
            && x < img.width() as f64
            && y > 0.0
            && y < img.height() as f64
        {
            flow[x as usize][y as usize] += ((self.path_len - i) as f64) / self.path_len as f64;
            let angle: f64 =
                2.0 * PI * (perlin.fractal(x, y, self.frequency, 6)? - 0.5) * self.curvature;
            x += angle.cos();
            y += angle.sin();
            i += 1;
        }
        Ok(())
    }
}

impl<R: Rng> Algorithm<R> for Flow {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        let mut flow = vec![vec![0.0; img.height() as usize]; img.width() as usize];
        let mut perlin: Perlin = Perlin::new(img.width() as usize, img.height() as usize);
        perlin.regenerate_noise(rng);
        self.signum = rng.gen_bool(0.5);
        self.curvature = 0.3 + 0.1 * ((rng.gen::<u8>() as f64) / 0xff as f64);
        self.frequency = 0.003 + 0.003 * ((rng.gen::<u8>() as f64) / 0xff as f64);
        for _ in 0..self.num_particles {
            self.make_single_path(rng, &perlin, &mut flow, img)?;
        }
        let max: f64 = flow
            .iter()
            .flatten()
            .fold(f64::NEG_INFINITY, |prev, curr| prev.max(*curr));
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let val: u8 = (256.0
                + (self.signum() as f64) * (55.0 + 200.0 * flow[x as usize][y as usize] / max))
                as u8;
            *pixel = Rgb([val, val, val]);
        }
        Ok(())
    }
}
