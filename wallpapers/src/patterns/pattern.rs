use crate::algorithms::Algorithm;
use crate::patterns::PatternColor;
use image::RgbImage;
use rand::Rng;

/// A simple pattern to be drawn onto an image
pub struct Pattern {
    /// The actual pattern
    data: Vec<Vec<PatternColor>>,
    width: usize,
    height: usize,
    // Add noise parameters, etc. here
}

impl Pattern {
    /// Initialize a new pattern from the given bool array by cloning the given pixel to each
    /// pixel that is set to true.
    pub fn from_boolarray<const N: usize, const M: usize>(
        arr: &[[bool; N]; M],
        pixel: &PatternColor,
    ) -> Self {
        let mut new_data = vec![vec![PatternColor::None; arr[0].len()]; arr.len()];
        for (x, xx) in arr.iter().enumerate() {
            for (y, yy) in xx.iter().enumerate() {
                if *yy {
                    new_data[x][y] = *pixel;
                }
            }
        }
        Self {
            data: new_data,
            width: N,
            height: M,
        }
    }
    pub fn enumerate_pixels(&self) -> impl Iterator<Item = (usize, usize, &PatternColor)> {
        (0..self.height)
            .flat_map(move |h| (0..self.width).map(move |w| (w, h)))
            .map(|(x, y)| (x, y, &self.data[x][y]))
    }
}

impl<R: Rng> Algorithm<R> for Pattern {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let val = self.data[x as usize % self.data.len()][y as usize % self.data[0].len()];
            let old_val = pixel.0;
            *pixel = match val {
                PatternColor::Solid { color } => image::Rgb(color),
                PatternColor::Alpha { .. } => todo!(),
                PatternColor::Perlin { .. } => todo!(),
                PatternColor::None => image::Rgb(old_val),
                PatternColor::Random { minvalue, maxvalue } => image::Rgb([
                    rng.gen_range(minvalue[0]..=maxvalue[0]),
                    rng.gen_range(minvalue[1]..=maxvalue[1]),
                    rng.gen_range(minvalue[2]..=maxvalue[2]),
                ]),
            };
        }
        Ok(())
    }
}

/// Pre-defined patterns
pub struct Patterns {}

impl Patterns {
    pub fn diamond() -> Pattern {
        Pattern::from_boolarray(
            &[
                [false, false, true, false, false],
                [false, true, false, true, false],
                [true, false, false, false, true],
                [false, true, false, true, false],
                [false, false, true, false, false],
            ],
            &PatternColor::Random {
                minvalue: [40, 40, 40],
                maxvalue: [255, 255, 255],
            },
        )
    }
}
