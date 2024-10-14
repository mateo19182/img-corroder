use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
mod fx;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image file
    #[arg(short, long)]
    input: PathBuf,

    /// Output image file
    #[arg(short, long)]
    output: PathBuf,

    /// Configuration file
    #[arg(short, long)]
    config: PathBuf,
}

#[derive(Deserialize, Debug)]
struct TransformConfig {
    name: String,
    params: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct Config {
    transformations: Vec<TransformConfig>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config_content = fs::read_to_string(&args.config)?;
    let config: Config = serde_json::from_str(&config_content)?;

    let mut img = image::open(&args.input)?;

    for transform in config.transformations {
        img = apply_transformation(img, &transform)?;
    }

    img.save(&args.output)?;
    println!("Transformations applied and saved to {:?}", args.output);

    Ok(())
}

fn apply_transformation(img: image::DynamicImage, transform: &TransformConfig) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    match transform.name.as_str() {
        "grayscale" => Ok(img.grayscale()),
        "invert" => {
            let mut inverted = img;
            inverted.invert();
            Ok(inverted)
        },
        "blur" => {
            let sigma = transform.params["sigma"].as_f64().unwrap_or(2.0) as f32;
            Ok(img.blur(sigma))
        },
        "pixelate" => {
            let block_size = transform.params["block_size"].as_u64().unwrap_or(10) as u32;
            Ok(fx::pixelate(&img, block_size))
        },
        "oil_painting" => {
            let radius = transform.params["radius"].as_u64().unwrap_or(4) as u32;
            let intensity = transform.params["intensity"].as_u64().unwrap_or(30) as u32;
            Ok(fx::oil_painting(&img, radius, intensity.try_into().unwrap()))
        },
        "glitch" => {
            let amount = transform.params["amount"].as_u64().unwrap_or(50) as u32;
            let seed = transform.params["seed"].as_u64().unwrap_or(10) as u32;
            Ok(fx::glitch(&img, amount, seed.try_into().unwrap()))
        },
        "sort" => Ok(fx::sort_pixel(&img)),
        _ => Err("Invalid transformation specified".into()),
    }
}