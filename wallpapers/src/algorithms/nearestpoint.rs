use image::{Rgb, RgbImage};
use rand::Rng;
use std::cmp::max;
use std::collections::HashSet;
use crate::algorithms::Algorithm;

enum PointDrawingMode {
    Hard,
    Smooth { brightness: f64 },
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    /// X Coordinate
    x: u32,
    /// Y Coordinate
    y: u32,
    /// The color of the point
    color: [u8; 3],
}

impl Point {
    /// Get the distance of this point to the other point
    fn distance_to(&self, x: u32, y: u32) -> f64 {
        (((x as i32 - self.x as i32).pow(2) + (y as i32 - self.y as i32).pow(2)) as f64).sqrt()
    }
    /// Get the color that a pixel should get if drawn in connection to this point
    fn color_at(&self, x: u32, y: u32) -> [u8; 3] {
        // If debugging build is used, draw red dots where the actual points are placed
        if cfg!(debug_assertions) && self.x == x && self.y == y {
            [255, 0, 0]
        } else {
            self.color
        }
    }
    /// Get a default black point at u32::MIN
    fn black() -> Self {
        Point {
            x: u32::MIN,
            y: u32::MIN,
            color: [0, 0, 0],
        }
    }
}

/// Nearest Points Algorithm Implementation by Lukas Kirschner, 2021
pub struct NearestPoint {
    points: HashSet<Point>,
    mode: PointDrawingMode,
}

impl Default for NearestPoint {
    fn default() -> Self {
        NearestPoint {
            points: HashSet::new(),
            mode: PointDrawingMode::Hard,
        }
    }
}

impl NearestPoint {
    pub fn new_soft() -> Self {
        NearestPoint {
            points: HashSet::new(),
            mode: PointDrawingMode::Smooth { brightness: 1.25 },
        }
    }
}

impl NearestPoint {
    fn populate_points(&mut self, rng: &mut impl Rng, num_points: usize, img: &RgbImage) {
        for _ in 0..num_points {
            self.points.insert(Point {
                x: rng.gen_range(0..img.width()),
                y: rng.gen_range(0..img.height()),
                color: [
                    rng.gen_range(0..128) + 128,
                    rng.gen_range(0..128) + 128,
                    rng.gen_range(0..128) + 128,
                ],
            });
        }
    }
    fn color_image(&self, img: &mut RgbImage) {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            match self.mode {
                PointDrawingMode::Hard => {
                    let nearest_point = self.points.iter().min_by(|first, second| {
                        first.distance_to(x, y).total_cmp(&second.distance_to(x, y))
                    });
                    *pixel = Rgb(nearest_point.unwrap_or(&Point::black()).color_at(x, y));
                },
                PointDrawingMode::Smooth { brightness } => {
                    let new_color: [f64; 3] = self
                        .points
                        .iter()
                        .map(|point| {
                            [
                                point.color[0] as f64
                                    / (point.distance_to(x, y) + 1.0).powf(1.0 / brightness),
                                point.color[1] as f64
                                    / (point.distance_to(x, y) + 1.0).powf(1.0 / brightness),
                                point.color[2] as f64
                                    / (point.distance_to(x, y) + 1.0).powf(1.0 / brightness),
                            ]
                        })
                        .fold([0.0, 0.0, 0.0], |a, b| {
                            [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
                        });
                    let new_color = [
                        f64::min(255.0, new_color[0]) as u8,
                        f64::min(255.0, new_color[1]) as u8,
                        f64::min(255.0, new_color[2]) as u8,
                    ];
                    *pixel = Rgb(new_color);
                },
            }
        }
    }
}

impl<R: Rng> Algorithm<R> for NearestPoint {
    fn build(&mut self, rng: &mut R, img: &mut RgbImage) -> Result<(), String> {
        let num_points = max(2, img.width() * img.height() / 20000);
        self.populate_points(
            rng,
            num_points.try_into().unwrap_or_else(|_| {
                panic!(
                    "There were too many points for this algorithm to handle: {}",
                    num_points
                )
            }),
            img,
        );
        self.color_image(img);
        Ok(())
    }
}
