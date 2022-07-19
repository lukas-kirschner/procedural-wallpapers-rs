use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use clap::ArgEnum;
use clap::ValueHint;
use image::{ImageBuffer, RgbImage};
use crate::algorithms::Algorithm;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;

mod algorithms;
mod utils;
mod layers;

#[derive(Debug, Copy, Clone, PartialEq, ArgEnum)]
enum Mode {
    CLOUDS,
    FLOW,
    ISLANDS,
    LIGHTNING,
    NEARESTPOINT,
    TANGLES,
    CELLULARONE,
    SQUARES,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "clouds" => Ok(Mode::CLOUDS),
            "flow" => Ok(Mode::FLOW),
            "islands" => Ok(Mode::ISLANDS),
            "lightning" => Ok(Mode::LIGHTNING),
            "nearest-point" => Ok(Mode::NEARESTPOINT),
            "tangles" => Ok(Mode::TANGLES),
            "cellularone" => Ok(Mode::CELLULARONE),
            "squares" => Ok(Mode::SQUARES),
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
    let mut algorithm: Box<dyn Algorithm<ChaCha8Rng>> = args.mode.to_algorithm::<ChaCha8Rng>();
    let mut rng = if args.seed != 0 {
        ChaCha8Rng::seed_from_u64(args.seed as u64)
    } else {
        ChaCha8Rng::from_entropy()
    };
    algorithm.build(&mut rng, img.borrow_mut()).unwrap();
    img.save(args.output).unwrap();
    println!("Hello, world! The mode is {:?}", args.mode);
}
