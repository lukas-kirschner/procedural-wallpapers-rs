use crate::Algorithm;
use image::{Rgb, RgbImage};
use rand::Rng;
use std::cmp::{max, min};
use std::collections::HashSet;

/// A simple cellular automaton that iteratively processes pixel colors based on their neighbors,
/// using a random algorithm for variations.
///
/// The algorithm works as follows:
/// 1. Assign a certain number of initial pixels a random color, at a random position
/// 2. Iteratively visit each unvisited pixel that has at least one neighbor.
///    There is a P% chance that a pixel is skipped in that step, to be processed later.
///    Each pixel is then assigned the average color of each adjacent visited pixel, before a
///    certain number of bytes B is subtracted from each processed pixel.
/// 3. Iteratively repeat that step, until all pixels have been visited.
pub struct CellularOne {
    /// If true, all pixels have been visited once
    all_visited: bool,
    num_visited: usize,
    pixel_skip_probability: f64,
}

impl Default for CellularOne {
    fn default() -> Self {
        CellularOne {
            all_visited: false,
            num_visited: 0,
            pixel_skip_probability: 0.01,
        }
    }
}

/// Get the new color of a pixel, if at least one adjacent pixel has a color
fn get_new_color(x: u32, y: u32, img: &RgbImage, visited: &[Vec<bool>]) -> Option<[u8; 3]> {
    let mut new_color: Vec<[u8; 3]> = vec![];
    for xx in max(0, (x as i32) - 1)..=min(img.width() as i32 - 1, (x as i32) + 1) {
        for yy in max(0, (y as i32) - 1)..=min(img.height() as i32 - 1, (y as i32) + 1) {
            if visited[xx as usize][yy as usize] && xx != x as i32 && yy != y as i32 {
                new_color.push(
                    img.get_pixel(xx.try_into().unwrap(), yy.try_into().unwrap())
                        .0,
                );
            }
        }
    }
    if new_color.is_empty() {
        None
    } else {
        let ret: [u8; 3] = [
            max(
                1,
                (new_color.iter().map(|c| c[0] as i32).sum::<i32>() / new_color.len() as i32) as u8,
            ) - 1,
            max(
                1,
                (new_color.iter().map(|c| c[1] as i32).sum::<i32>() / new_color.len() as i32) as u8,
            ) - 1,
            max(
                1,
                (new_color.iter().map(|c| c[2] as i32).sum::<i32>() / new_color.len() as i32) as u8,
            ) - 1,
        ];
        Some(ret)
    }
}

impl CellularOne {
    /// Create the initial random population of points
    fn populate_points(
        &mut self,
        rng: &mut impl Rng,
        num_points: usize,
        img: &mut RgbImage,
        visited: &mut [Vec<bool>],
    ) {
        for _ in 0..num_points {
            let x = rng.gen_range(0..img.width());
            let y = rng.gen_range(0..img.height());
            let color: [u8; 3] = [
                rng.gen_range(0..128) + 128,
                rng.gen_range(0..128) + 128,
                rng.gen_range(0..128) + 128,
            ];
            let pixel = img.get_pixel_mut(x, y);
            *pixel = Rgb(color);
            if !visited[x as usize][y as usize] {
                self.num_visited += 1;
            }
            visited[x as usize][y as usize] = true;
        }
    }
    fn iterate_once(&mut self, rng: &mut impl Rng, img: &mut RgbImage, visited: &mut [Vec<bool>]) {
        // Keep track of newly-visited pixels and apply them later
        let mut newly_visited: HashSet<(u32, u32)> = HashSet::new();
        for x in 0..img.width() {
            for y in 0..img.height() {
                // Skip visited pixels, they should be kept as-is
                if visited[x as usize][y as usize] {
                    continue;
                }
                if let Some(new_color) = get_new_color(x, y, img, visited) {
                    // Skip each pixel with a certain probability
                    if rng.gen_bool(self.pixel_skip_probability) {
                        continue;
                    }
                    let pixel = img.get_pixel_mut(x, y);
                    *pixel = Rgb(new_color);
                    let succ = newly_visited.insert((x, y));
                    assert!(succ);
                }
            }
        }
        let num_new = newly_visited.len();
        self.num_visited += num_new;
        for (x, y) in newly_visited {
            assert!(!visited[x as usize][y as usize]);
            visited[x as usize][y as usize] = true;
        }
        if self.num_visited >= (img.width() * img.height()) as usize {
            self.all_visited = true;
        }
        if cfg!(debug_assertions) {
            println!("Visited {} pixels ({} more)", self.num_visited, num_new);
        }
    }
}

impl<R: Rng> Algorithm<R> for CellularOne {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        let mut visited: Vec<Vec<bool>> =
            vec![vec![false; img.height() as usize]; img.width() as usize];
        // Build initial population
        let num_points: usize = max(2, img.width() * img.height() / 20000)
            .try_into()
            .expect("There were too many pixels to handle!");
        self.populate_points(rng, num_points, img, &mut visited);
        let mut num_iter: usize = 0;
        while !self.all_visited {
            self.iterate_once(rng, img, &mut visited);
            num_iter += 1;
        }
        if cfg!(debug_assertions) {
            println!("Done in {} iterations.", num_iter);
        }
        Ok(())
    }
}
