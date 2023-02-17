use crate::algorithms::Algorithm;
use clap::ValueHint;
use clap::{Parser, ValueEnum};
use image::{ImageBuffer, RgbImage};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::str::FromStr;

mod algorithms;
mod layers;
mod patterns;
mod utils;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
enum Mode {
    #[clap(name = "clouds")]
    Clouds,
    #[clap(name = "flow")]
    Flow,
    #[clap(name = "islands")]
    Islands,
    #[clap(name = "lightning")]
    Lightning,
    #[clap(name = "nearestpoint")]
    NearestPoint,
    #[clap(name = "tangles")]
    Tangles,
    #[clap(name = "cellularone")]
    CellularOne,
    #[clap(name = "squares")]
    Squares,
    #[clap(name = "squareshor")]
    SquaresHor,
    #[clap(name = "squaresver")]
    SquaresVer,
    #[clap(name = "squaresdiag")]
    SquaresDiag,
    #[clap(name = "squares2")]
    Squares2,
    #[clap(name = "squares2h")]
    Squares2H,
    #[clap(name = "squares2v")]
    Squares2V,
    #[clap(name = "nearestgradient")]
    NearestGradient,
}

impl FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValueEnum::from_str(s, true)
    }
}

/// Generate wallpapers procedurally with the given algorithm
#[derive(Parser, PartialEq, Debug)]
#[clap(author, version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// Image generation mode
    #[clap(short, long)]
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
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    output: PathBuf,
    /// Open the command-line help
    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
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
