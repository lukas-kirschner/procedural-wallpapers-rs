use crate::patterns::PatternColor;

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
    pub fn from_boolarray<const n: usize, const m: usize>(arr: &[[bool; n]; m], pixel: &PatternColor) -> Self {
        let mut new_data = vec![vec![PatternColor::NONE; arr[0].len()]; arr.len()];
        for (x, xx) in arr.iter().enumerate() {
            for (y, yy) in xx.iter().enumerate() {
                if *yy {
                    new_data[x][y] = pixel.clone();
                }
            }
        }
        Self {
            data: new_data,
            width: n,
            height: m,
        }
    }
    pub fn enumerate_pixels(&self) -> impl Iterator<Item=(usize, usize, &PatternColor)> {
        (0..self.height)
            .map(move |h| (0..self.width)
                .map(move |w| (w, h)))
            .flatten()
            .map(|(x, y)| (x, y, &self.data[x][y]))
    }
}

/// Pre-defined patterns
pub struct Patterns {}

impl Patterns {
    fn diamond() -> Pattern {
        Pattern::from_boolarray(&[
            [false, false, true, false, false],
            [false, true, false, true, false],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
        ], &PatternColor::RANDOM { minvalue: [40, 40, 40], maxvalue: [255, 255, 255] })
    }
}