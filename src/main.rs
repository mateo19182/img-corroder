mod fx;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image file
    #[arg(short, long)]
    input: PathBuf,

    /// Output image file
    #[arg(short, long)]
    output: PathBuf,

    /// Transformation to apply (grayscale, invert, blur, pixelate, oil_painting, glitch)
    #[arg(short, long)]
    transform: String,
}
const BLOCK_SIZE: u32 = 10;
const SIGMA: f32 = 2.0;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut img = image::open(&args.input)?;

    let transformed_img = match args.transform.as_str() {
        "grayscale" => img.grayscale(),
        "invert" => {
            img.invert();
            img
        },
        "blur" => img.blur(SIGMA),
        "pixelate" => fx::pixelate(&img, BLOCK_SIZE),
        "oil_painting" => fx::oil_painting(&img, 4, 30),
        "glitch" => fx::glitch(&img, 50, 10),
        "sort" => fx::sort_pixel(&img),
        _ => return Err("Invalid transformation specified".into()),
    };

    transformed_img.save(&args.output)?;
    println!("Transformation applied and saved to {:?}", args.output);

    Ok(())
}