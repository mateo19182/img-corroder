mod fx;

use clap::Parser;
use image::DynamicImage;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let img = image::open(&args.input)?;

    let transformed_img = match args.transform.as_str() {
        "grayscale" => img.grayscale(),
        "invert" => {
            let mut img = img;
            img.invert();
            img
        },
        "blur" => img.blur(2.0),
        "pixelate" => fx::pixelate(&img, 10),
        "oil_painting" => fx::oil_painting(&img, 4, 30),
        "glitch" => fx::glitch(&img, 50, 10),
        _ => return Err("Invalid transformation specified".into()),
    };

    transformed_img.save(&args.output)?;
    println!("Transformation applied and saved to {:?}", args.output);

    Ok(())
}