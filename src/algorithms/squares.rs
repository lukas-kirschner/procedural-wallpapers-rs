use std::cmp::{max, min};
use std::collections::HashSet;
use image::{Rgb, RgbImage};
use rand::Rng;
use crate::Algorithm;

/// A simple algorithm that starts with a random colored square in the upper-right corner.
/// It then iterates from left-to-right and bottom-down over each square
/// and assigns the average color of each surrounding square with a random variation in a
///random RGB channel to each square, until the bottom-right is reached.
pub struct Squares {
    squaresize_h: usize,
    squaresize_v: usize,
    squares_h: usize,
    squares_v: usize,
    variation_amount: u8,
    visited_squares: Vec<Vec<bool>>,
}

impl Default for Squares {
    fn default() -> Self {
        Squares {
            squaresize_v: 10,
            squares_h: 0,
            squaresize_h: 10,
            squares_v: 0,
            variation_amount: 20,
            visited_squares: vec![],
        }
    }
}

impl Squares {
    /// Gets the square color at the given location, or returns None, if the square has not yet been visited
    fn get_square_color(&self, square_x: usize, square_y: usize, img: &mut RgbImage) -> Option<[u8; 3]> {
        if self.visited_squares[square_x][square_y] {
            Some(img.get_pixel(square_x as u32 * self.squaresize_h as u32, square_y as u32 * self.squaresize_v as u32).0)
        } else {
            None
        }
    }
    /// Gets the average color of the given square and all squares surrounding it, i.e. 9 squares.
    /// All unvisited squares are skipped in the calculation.
    /// If none of the 9 squares has been visited, return None
    fn get_average_color_of_squares(&self, square_x: usize, square_y: usize, img: &mut RgbImage) -> Option<[u8; 3]> {
        let mut colors: HashSet<[u8; 3]> = HashSet::new();
        for xx in max(0, square_x as i32 - 1)..=min(self.squares_h as i32 - 1, square_x as i32 + 1) {
            for yy in max(0, square_y as i32 - 1)..=min(self.squares_v as i32 - 1, square_y as i32 + 1) {
                if let Some(color) = self.get_square_color(xx as usize, yy as usize, img) {
                    colors.insert(color);
                }
            }
        }
        if colors.len() > 0 {
            Some([
                (colors.iter().map(|color| { color[0] as u32 }).sum::<u32>() as u32 / colors.len() as u32) as u8,
                (colors.iter().map(|color| { color[1] as u32 }).sum::<u32>() as u32 / colors.len() as u32) as u8,
                (colors.iter().map(|color| { color[2] as u32 }).sum::<u32>() as u32 / colors.len() as u32) as u8,
            ])
        } else {
            None
        }
    }
    /// Color the given square in the given color and mark it as visited.
    fn color_square(&mut self, square_x: usize, square_y: usize, img: &mut RgbImage, color: [u8; 3]) {
        let x_start = square_x * self.squaresize_h;
        let y_start = square_y * self.squaresize_v;
        let x_end = (square_x + 1) * self.squaresize_h;
        let y_end = (square_y + 1) * self.squaresize_v;
        for x in x_start..x_end {
            for y in y_start..y_end {
                let pixel = img.get_pixel_mut(x as u32, y as u32);
                *pixel = Rgb(color);
            }
        }
        self.visited_squares[square_x][square_y] = true;
    }
    fn color_square_average(&mut self, rng: &mut impl Rng, square_x: usize, square_y: usize, img: &mut RgbImage) {
        if let Some(average_color) = self.get_average_color_of_squares(square_x, square_y, img) {
            let color_offset: i32 = rng.gen_range(-(self.variation_amount as i32)..=(self.variation_amount as i32));
            let mut variant_color: [u8; 3] = average_color;
            let channel_id: usize = rng.gen_range(0..3);
            if color_offset > 0 {
                variant_color[channel_id] = min(255, average_color[channel_id] as i32 + color_offset) as u8;
            } else {
                variant_color[channel_id] = max(0, average_color[channel_id] as i32 + color_offset) as u8;
            }
            self.color_square(square_x, square_y, img, variant_color);
        } else {
            self.color_square(square_x, square_y, img, [
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            ])
        }
    }
}

impl<R: Rng> Algorithm<R> for Squares {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        self.squares_v = max(1, img.height() as usize / self.squaresize_v);
        self.squares_h = max(1, img.width() as usize / self.squaresize_h);
        self.visited_squares = vec![vec![false; self.squares_v]; self.squares_h];

        for x in 0..self.squares_h {
            for y in 0..self.squares_v {
                self.color_square_average(rng, x, y, img);
            }
        }
        Ok(())
    }
}