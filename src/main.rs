use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use clap::ArgEnum;
use clap::ValueHint;
use image::{ImageBuffer, RgbImage};
use crate::algorithms::Algorithm;

mod algorithms;
mod utils;

#[derive(Debug, Copy, Clone, PartialEq, ArgEnum)]
enum Mode {
    CLOUDS,
    FLOW,
    ISLANDS,
    LIGHTNING,
    NEAREST_POINT,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "clouds" => Ok(Mode::CLOUDS),
            "flow" => Ok(Mode::FLOW),
            "islands" => Ok(Mode::ISLANDS),
            "lightning" => Ok(Mode::LIGHTNING),
            "nearest_point" => Ok(Mode::NEAREST_POINT),
            _ => Err(format!("Invalid Mode: {}", s))
        }
    }
}

/// Generate wallpapers procedurally with the given algorithm
#[derive(Parser, PartialEq, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image generation mode
    #[clap(short, long, arg_enum)]
    mode: Mode,
    /// Desired width (pixels) of the generated image
    #[clap(short, long, value_parser, default_value_t = 1920)]
    width: u32,
    /// Desired height (pixels) of the generated image
    #[clap(short, long, value_parser, default_value_t = 1080)]
    height: u32,
    /// Seed for the random number generator. If a seed of 0 is given, no seed is used
    #[clap(short, long, value_parser, default_value_t = 0)]
    seed: i32,
    /// The output file to save
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut img: RgbImage = ImageBuffer::new(args.width, args.height);
    let mut algorithm: Box<dyn Algorithm> = args.mode.to_algorithm();
    algorithm.build(img.borrow_mut()).unwrap();
    img.save(args.output).unwrap();
    println!("Hello, world! The mode is {:?}", args.mode);
}
