use image::{Rgb, RgbImage};
use rand::Rng;
use std::cmp::max;
use crate::algorithms::Algorithm;

/// Lightning
/// Based on the LIGHTNING algorithm implementation by Attila Bagyoni, 2018
/// https://github.com/bagyoni/procedural-wallpapers
pub struct Lightning {
    step: u32,
    distribution: u8,
    fg_color: [u8; 3],
}

impl Default for Lightning {
    fn default() -> Self {
        Lightning {
            step: 0,
            distribution: 0,
            fg_color: [255, 255, 255],
        }
    }
}

impl Lightning {
    fn generate_distribution(&mut self, rng: &mut impl Rng) {
        self.distribution = rng.gen_range(0..=3);
    }

    fn place_charge(&self, rng: &mut impl Rng, img: &RgbImage) -> (u32, u32) {
        let ran1 = rng.gen_range(2..img.height() - 1);
        let ran2 = rng.gen_range(1..=img.width());
        match self.distribution {
            0 => (
                rng.gen_range(0..img.width()),
                img.height() - rng.gen_range(0..=ran1) - 2,
            ),
            1 => (rng.gen_range(0..img.width()), rng.gen_range(0..=ran1)),
            2 => (
                img.width() - rng.gen_range(0..ran2) - 1,
                rng.gen_range(0..(img.height() - 1)),
            ),
            3 => (rng.gen_range(0..ran2), rng.gen_range(0..(img.height() - 1))),

            _ => panic!("Illegal distribution: {}", self.distribution),
        }
    }
    fn next_step(&mut self, rng: &mut impl Rng, img: &mut RgbImage, particles: &mut [(u32, u32)]) {
        let (x, y) = self.place_charge(rng, img);
        // Get the index of the nearest particle
        let (nearest, _) = (0..self.step)
            .map(|i| {
                (
                    Some(i),
                    ((particles[i as usize].0 as i32 - x as i32).pow(2)
                        + (particles[i as usize].1 as i32 - y as i32).pow(2))
                        as u32,
                )
            })
            .fold((None, img.width().pow(2) + img.height().pow(2)), |i, j| {
                if j.1 < i.1 {
                    (j.0, j.1)
                } else {
                    (i.0, i.1)
                }
            });
        // Unwrap the index
        let nearest = nearest.expect("No nearest was found!") as usize;
        let mut dx: i32 = if particles[nearest].0 < x { 1 } else { -1 };
        let mut dy: i32 = if particles[nearest].1 < y { 1 } else { -1 };
        if dx * (x as i32 - particles[nearest].0 as i32)
            > 3 * dy * (y as i32 - particles[nearest].1 as i32)
        {
            dy = 0;
        } else if dy * (y as i32 - particles[nearest].1 as i32)
            > 3 * dx * (x as i32 - particles[nearest].0 as i32)
        {
            dx = 0;
        }
        assert!((-1..=1).contains(&dx));
        assert!((-1..=1).contains(&dy));
        particles[self.step as usize] = (
            max(particles[nearest].0 as i32 + dx, 0) as u32,
            max(0, particles[nearest].1 as i32 + dy) as u32,
        );
        *img.get_pixel_mut(
            particles[self.step as usize].0,
            particles[self.step as usize].1,
        ) = Rgb(self.fg_color);
        *img.get_pixel_mut(
            particles[self.step as usize].0,
            particles[self.step as usize].1 + 1,
        ) = Rgb(self.fg_color);
        self.step += 1;
    }
}

impl<R: Rng> Algorithm<R> for Lightning {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        let pnum = img.width() * 10;
        let mut particles: Vec<(u32, u32)> = vec![(0, 0); pnum as usize];
        self.generate_distribution(rng);
        particles[0] = (rng.gen_range(0..(img.width() as usize)) as u32, 0);
        let red: u8 = rng.gen_range(0..120);
        let green: u8 = rng.gen_range(0..120);
        self.step = 1;
        while self.step < pnum {
            if (self.step % 2000) == 0 {
                self.generate_distribution(rng);
            }
            self.fg_color = [
                (red as u32 + (200 - red) as u32 * (pnum - self.step) / pnum) as u8,
                (green as u32 + (200 - green) as u32 * (pnum - self.step) / pnum) as u8,
                255,
            ];
            self.next_step(rng, img, &mut particles);
        }
        Ok(())
    }
}
