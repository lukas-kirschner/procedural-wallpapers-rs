use std::borrow::Borrow;
use std::cmp::max;
use image::{Rgb, RgbImage};
use crate::layers::{Layer, MixMode};

/// A Squares Layer, representing "large pixels" with certain dimensions that can be drawn
/// on top of an image.
pub struct SquaresLayer<Color: Default + Clone> {
    squaresize_h: usize,
    squaresize_v: usize,
    squares_h: usize,
    squares_v: usize,
    mixmode: MixMode,
    data: Vec<Vec<Color>>,
}

// Constructor
impl<Color: Default + Clone> SquaresLayer<Color> {
    pub fn new(squares_h: usize, squares_v: usize, squaresize_h: usize, squaresize_v: usize) -> Self {
        SquaresLayer {
            squaresize_h,
            squaresize_v,
            squares_h,
            squares_v,
            mixmode: MixMode::NORMAL,
            data: vec![vec![Color::default(); squares_v]; squares_h],
        }
    }
}

// Getters
impl<Color: Default + Clone> SquaresLayer<Color> {
    // pub fn squaresize_h(&self) -> usize {
    //     self.squaresize_h
    // }
    // pub fn squaresize_v(&self) -> usize {
    //     self.squaresize_v
    // }
    pub fn squares_v(&self) -> usize {
        self.squares_v
    }
    pub fn squares_h(&self) -> usize {
        self.squares_h
    }
    pub fn get_color_at(&self, x: usize, y: usize) -> &Color {
        self.data[x][y].borrow()
    }
}

// Setters
impl<Color: Default + Clone> SquaresLayer<Color> {
    // /// Set the new horizontal square size and return the newly-set value
    // pub fn set_squaresize_h(&mut self, new_squaresize_h: usize) -> usize {
    //     self.squaresize_h = new_squaresize_h;
    //     self.squaresize_h
    // }
    // /// Set the new vertical square size and return the newly-set value
    // pub fn set_squaresize_v(&mut self, new_squaresize_v: usize) -> usize {
    //     self.squaresize_v = new_squaresize_v;
    //     self.squaresize_v
    // }
    // /// Set the new vertical square count and return the newly-set value.
    // /// This operation is expensive, since it needs to re-allocate memory
    // pub fn set_squares_v(&mut self, new_squares_v: usize) -> usize {
    //     self.squares_v = new_squares_v;
    //     self.squares_v
    // }
    // /// Set the new horizontal square count and return the newly-set value
    // /// This operation is expensive, since it needs to re-allocate memory
    // pub fn set_squares_h(&mut self, new_squares_h: usize) -> usize {
    //     self.squares_h = new_squares_h;
    //     self.squares_h
    // }

    /// Adjust the dimensions of the squares to the given image.
    /// This will adjust the data accordingly, filling up all new squares with Color::default()
    pub fn adjust_square_count_to_image_dimensions(&mut self, img_width: usize, img_height: usize) {
        self.squares_v = max(1, img_height / self.squaresize_v);
        self.squares_h = max(1, img_width / self.squaresize_h);
        self.data.resize_with(self.squares_h, || { vec![Color::default(); self.squares_v] });
        for colvec in self.data.iter_mut() {
            colvec.resize_with(self.squares_v, || { Color::default() });
        }
    }
    pub fn set_color_at(&mut self, x: usize, y: usize, color: Color) {
        self.data[x][y] = color;
    }
}

// Helpers for internal use
impl SquaresLayer<[u8; 3]> {
    /// Color the given square in the given color and mark it as visited.
    fn color_square(&self, square_x: usize, square_y: usize, img: &mut RgbImage, color: [u8; 3]) {
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
    }
}

impl Layer for SquaresLayer<[u8; 3]> {
    /// Draw the squares onto an image
    fn draw(&self, img: &mut RgbImage) -> Result<(), String> {
        let layerheight = self.squares_v * self.squaresize_v;
        let layerwidth = self.squares_h * self.squaresize_h;
        if layerheight > img.height() as usize {
            return Err(format!("The image height of {} was smaller than the layer height of {}", img.height(), layerheight));
        }
        if layerwidth > img.width() as usize {
            return Err(format!("The image width of {} was smaller than the layer width of {}", img.width(), layerwidth));
        }
        for x in 0..self.squares_h() {
            for y in 0..self.squares_v() {
                self.color_square(x, y, img, self.data[x][y]);
            }
        }
        Ok(())
    }

    fn set_mix_mode(&mut self, mode: MixMode) {
        self.mixmode = mode;
    }

    fn get_mix_mode(&self) -> MixMode {
        self.mixmode
    }
}