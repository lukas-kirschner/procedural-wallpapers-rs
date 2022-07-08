/// Perlin Noise Implementation in Rust
use rand::Rng;

pub struct Perlin {
    /// Width of the perlin noise
    width: usize,
    /// Height of the perlin noise
    height: usize,
    /// Noise Gradient
    gradient: Vec<Vec<(f64, f64)>>,
}

impl Perlin {
    pub fn new(width: usize, height: usize) -> Self {
        Perlin {
            width,
            height,
            gradient: vec![vec![(0.0, 0.0); height]; width],
        }
    }
    fn distance_along_gradient(&self, x: f64, y: f64, gridx: usize, gridy: usize) -> Result<f64, String> {
        if gridx >= self.width || gridy >= self.height {
            Err(format!("Coordinates ({},{}) out of bounds for image of size ({},{})!", gridx, gridy, self.width, self.height))
        } else {
            Ok((x - gridx as f64) * self.gradient[gridx][gridy].0 +
                (y - gridy as f64) * self.gradient[gridx][gridy].1)
        }
    }
    fn inter(x: f64, y: f64, weight: f64) -> f64 {
        let yweight: f64 = weight * weight * (2.0 - weight);
        x * (1.0 - yweight) + y * yweight
    }
    /// Regenerate the noise
    pub fn regenerate_noise(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.width {
            for y in 0..self.height {
                let val: f64 = (rng.gen::<u32>() & 0xfff) as f64;
                self.gradient[x][y].0 = val.sin();
                self.gradient[x][y].1 = val.cos();
            }
        }
    }
    /// Get the value of the perlin noise at the given coordinate
    pub fn perlin(&self, x: f64, y: f64) -> Result<f64, String> {
        let d1: f64 = self.distance_along_gradient(x, y, (x as usize), (y as usize))?;
        let d2: f64 = self.distance_along_gradient(x, y, (x as usize) + 1, (y as usize))?;
        let d3: f64 = self.distance_along_gradient(x, y, (x as usize), (y as usize) + 1)?;
        let d4: f64 = self.distance_along_gradient(x, y, (x as usize) + 1, (y as usize) + 1)?;
        let i1: f64 = Perlin::inter(d1, d2, x - x.floor());
        let i2: f64 = Perlin::inter(d3, d4, x - x.floor());
        Ok(Perlin::inter(i1, i2, y - y.floor()))
    }
    /// Recursive Fractal Implementation
    pub fn fractal(&self, x: f64, y: f64, freq: f64, depth: u32) -> Result<f64, String> {
        match depth {
            0 => Ok(0.0),
            d => Ok(self.perlin(x * freq, y * freq)? + self.fractal(x, y, freq * 2.0, depth - 1)? / 2.0)
        }
    }
}
