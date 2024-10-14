use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
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

    let total_start = Instant::now();
    for transform in config.transformations {
        let start = Instant::now();
        img = apply_transformation(img, &transform)?;
        let duration = start.elapsed();
        println!(". Time: {} ms\n", duration.as_millis());
    }
    let total_duration = total_start.elapsed();

    img.save(&args.output)?;
    println!("Transformations applied and saved to {:?}", args.output);
    println!("Total time: {} ms", total_duration.as_millis());

    Ok(())
}

fn apply_transformation(img: image::DynamicImage, transform: &TransformConfig) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    match transform.name.as_str() {
        "grayscale" => {
            print!("Grayscaled image");
            Ok(img.grayscale())
        },
        "invert" => {
            let mut inverted = img;
            inverted.invert();
            print!("Inverted image");
            Ok(inverted)
        },
        "blur" => {
            let sigma = transform.params["sigma"].as_f64().unwrap_or(2.0) as f32;
            print!("Blurred image with sigma {}", sigma);
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
            let max_offset = transform.params["max_offset"].as_u64().unwrap_or(10) as i32;
            let direction: String = transform.params["direction"].to_string();
            let noisy_pixels: bool = transform.params["noisy"].as_bool().unwrap_or(false);
            Ok(fx::glitch(&img, amount, max_offset, &direction, noisy_pixels))
        },
        "sort" => {
            let mode: String = transform.params["mode"].to_string();
            let direction: String = transform.params["direction"].to_string();
            let threshold = transform.params.get("threshold").and_then(|v| v.as_u64()).unwrap_or(50) as u8;
            Ok(fx::sort_pixel(&img, &mode, &direction, Some(threshold)))
        },
        "rotate" => {
            let angle = transform.params["angle"].as_u64().unwrap_or(90) as f32;
            Ok(fx::rotate(&img, angle)?)
        },
        "desync" => {
            let x_shift = transform.params["x_shift"].as_i64().unwrap_or(10) as i32;
            let y_shift = transform.params["y_shift"].as_i64().unwrap_or(10) as i32;
            Ok(fx::desync(&img, x_shift, y_shift)?)
        },
        "wind" => {
            let direction = transform.params["direction"].as_str().unwrap_or("right").to_string();
            let strength: u32 = transform.params["strength"].as_u64().unwrap_or(10) as u32;
            Ok(fx::wind(&img, &direction, strength)?)
        },
        "scan-lines" => {
            let line_thickness = transform.params["line_thickness"].as_u64().unwrap_or(2) as u32;
            let line_spacing = transform.params["line_spacing"].as_u64().unwrap_or(10) as u32;
            let opacity = transform.params["opacity"].as_f64().unwrap_or(0.5) as f32;
            let angle = transform.params["angle"].as_f64().unwrap_or(0.0) as f32;
            Ok(fx::scan_lines(&img, Some(line_thickness), Some(line_spacing), Some(angle), Some(opacity))?)
        },
        _ => Err("Invalid transformation specified".into()),
    }
}
