use std::cmp::{max, min};
use image::{RgbImage};
use rand::Rng;
use crate::Algorithm;
use crate::layers::Layer;
use crate::layers::squareslayer::SquaresLayer;

enum Direction {
    HORIZONTAL,
    VERTICAL,
    DIAGONAL,
    NONE,
}

/// A simple algorithm that starts with a random colored square in the upper-right corner.
/// It then iterates from left-to-right and bottom-down over each square
/// and assigns the average color of each surrounding square with a random variation in a
///random RGB channel to each square, until the bottom-right is reached.
/// The surrounding square with the direction that this Struct is designed to prioritize
/// is weighted N times as high as the surrounding squares.
pub struct SquaresOneDirection {
    squares: SquaresLayer<[u8; 3]>,
    variation_amount: u8,
    visited_squares: Vec<Vec<bool>>,
    direction: Direction,
    weight: usize,
    additional_random_points: usize,
}

impl SquaresOneDirection {
    pub fn new_horiz() -> Self {
        SquaresOneDirection {
            squares: SquaresLayer::new(0, 0, 10, 10),
            variation_amount: 25,
            visited_squares: vec![],
            direction: Direction::HORIZONTAL,
            weight: 6,
            additional_random_points: 0,
        }
    }
    pub fn new_vert() -> Self {
        SquaresOneDirection {
            squares: SquaresLayer::new(0, 0, 10, 10),
            variation_amount: 25,
            visited_squares: vec![],
            direction: Direction::VERTICAL,
            weight: 4,
            additional_random_points: 0,
        }
    }
    pub fn new_diag() -> Self {
        SquaresOneDirection {
            squares: SquaresLayer::new(0, 0, 10, 10),
            variation_amount: 25,
            visited_squares: vec![],
            direction: Direction::DIAGONAL,
            weight: 4,
            additional_random_points: 0,
        }
    }
    pub fn new_nodir() -> Self {
        SquaresOneDirection {
            squares: SquaresLayer::new(0, 0, 10, 10),
            variation_amount: 20,
            visited_squares: vec![],
            direction: Direction::NONE,
            weight: 0,
            additional_random_points: 0,
        }
    }
    pub fn new_nodir_randomized() -> Self {
        SquaresOneDirection {
            squares: SquaresLayer::new(0, 0, 10, 10),
            variation_amount: 20,
            visited_squares: vec![],
            direction: Direction::NONE,
            weight: 0,
            additional_random_points: 32,
        }
    }
    pub fn new_horiz_randomized() -> Self {
        SquaresOneDirection {
            squares: SquaresLayer::new(0, 0, 10, 10),
            variation_amount: 25,
            visited_squares: vec![],
            direction: Direction::HORIZONTAL,
            weight: 20,
            additional_random_points: 48,
        }
    }
    pub fn new_vert_randomized() -> Self {
        SquaresOneDirection {
            squares: SquaresLayer::new(0, 0, 10, 10),
            variation_amount: 25,
            visited_squares: vec![],
            direction: Direction::VERTICAL,
            weight: 8,
            additional_random_points: 48,
        }
    }
}

impl SquaresOneDirection {
    /// Create the initial additional random population of points
    fn populate_points(&mut self, rng: &mut impl Rng) {
        for _ in 0..self.additional_random_points {
            let x = rng.gen_range(0..self.squares.squares_h());
            let y = rng.gen_range(0..self.squares.squares_v());
            let color: [u8; 3] = [
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            ];
            self.squares.set_color_at(x, y, color);
            self.visited_squares[x][y] = true;
        }
    }
    /// Gets the square color at the given location, or returns None, if the square has not yet been visited
    fn get_square_color(&self, square_x: usize, square_y: usize) -> Option<&[u8; 3]> {
        if self.visited_squares[square_x][square_y] {
            Some(self.squares.get_color_at(square_x, square_y))
        } else {
            None
        }
    }
    /// Gets the average color of the given square and all squares surrounding it, i.e. 9 squares.
    /// All unvisited squares are skipped in the calculation.
    /// If none of the 9 squares has been visited, return None
    fn get_average_color_of_squares(&self, square_x: usize, square_y: usize) -> Option<[u8; 3]> {
        // We need a vector here because elements may occur more than once
        let mut colors: Vec<&[u8; 3]> = Vec::new();
        for xx in max(0, square_x as i32 - 1)..=min(self.squares.squares_h() as i32 - 1, square_x as i32 + 1) {
            for yy in max(0, square_y as i32 - 1)..=min(self.squares.squares_v() as i32 - 1, square_y as i32 + 1) {
                if let Some(color) = self.get_square_color(xx as usize, yy as usize) {
                    match self.direction {
                        Direction::HORIZONTAL => {
                            if yy as usize == square_y && xx as usize != square_x {
                                for _ in 0..self.weight {
                                    colors.push(color);
                                }
                            } else {
                                colors.push(color);
                            }
                        }
                        Direction::VERTICAL => {
                            if xx as usize == square_x && yy as usize != square_y {
                                for _ in 0..self.weight {
                                    colors.push(color);
                                }
                            } else {
                                colors.push(color);
                            }
                        }
                        Direction::DIAGONAL => {
                            if xx as usize != square_x && yy as usize != square_y {
                                for _ in 0..self.weight {
                                    colors.push(color);
                                }
                            } else {
                                colors.push(color);
                            }
                        }
                        Direction::NONE => {
                            colors.push(color);
                        }
                    }
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

    fn color_square_average(&mut self, rng: &mut impl Rng, square_x: usize, square_y: usize) {
        if let Some(average_color) = self.get_average_color_of_squares(square_x, square_y) {
            let color_offset: i32 = rng.gen_range(-(self.variation_amount as i32)..=(self.variation_amount as i32));
            let mut variant_color: [u8; 3] = average_color;
            let channel_id: usize = rng.gen_range(0..3);
            if color_offset > 0 {
                variant_color[channel_id] = min(255, average_color[channel_id] as i32 + color_offset) as u8;
            } else {
                variant_color[channel_id] = max(0, average_color[channel_id] as i32 + color_offset) as u8;
            }
            self.squares.set_color_at(square_x, square_y, variant_color);
        } else {
            // Setting a color without any adjacent neighbors --> Completely random color
            self.squares.set_color_at(square_x, square_y, [
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            ]);
        }
        self.visited_squares[square_x][square_y] = true;
    }
}

impl<R: Rng> Algorithm<R> for SquaresOneDirection {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        self.squares.adjust_square_count_to_image_dimensions(img.width() as usize, img.height() as usize);
        self.visited_squares = vec![vec![false; self.squares.squares_v()]; self.squares.squares_h()];
        // n points per 1000x1000 pixels
        self.additional_random_points = (self.additional_random_points as f64 * (img.width() as f64 * img.height() as f64) / (1000.0 * 1000.0)) as usize;
        self.populate_points(rng);

        for x in 0..self.squares.squares_h() {
            for y in 0..self.squares.squares_v() {
                self.color_square_average(rng, x, y);
            }
        }
        self.squares.draw(img)?;
        Ok(())
    }
}