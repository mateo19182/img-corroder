use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
mod colorfx;
mod glitchfx;
mod edgesfx;
mod fx_json_generator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image file
    #[arg(short, long)]
    input: PathBuf,

    /// Output image file
    #[arg(short, long)]
    output: PathBuf,

    /// Configuration file or number of effects
    #[arg(short, long)]
    config: Option<String>,
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

    let config: Config = if let Some(num_effects) = args.config.as_ref().and_then(|s| s.parse::<usize>().ok()) {
        // If config is a number, generate JSON for that number of effects
        let json = fx_json_generator::generate_random_pipeline(num_effects).to_string();
        serde_json::from_str(&json)?
    } else if let Some(config_path) = &args.config {
        let config_content = fs::read_to_string(config_path)?;
        // If config is a file, read and parse it
        serde_json::from_str(&config_content)?
    } else {
        // If config doesn't exist or is invalid, use only 1 effect
        let json = fx_json_generator::generate_random_pipeline(1).to_string();
        serde_json::from_str(&json)?
    };

    let mut img = image::open(&args.input)?;

    let total_start = Instant::now();
    for transform in config.transformations {
        let start = Instant::now();
        img = apply_transformation(img, &transform)?;
        let duration = start.elapsed();
        println!("Applied {}. Time: {} ms. Params: {:?}", transform.name, duration.as_millis(), transform.params);
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
            Ok(img.grayscale())
        },
        "invert" => {
            let mut inverted = img;
            inverted.invert();
            Ok(inverted)
        },
        "brightness" => {
            let factor = transform.params["factor"].as_f64().unwrap_or(1.0) as f32;
            Ok(colorfx::brightness(&img, factor)?)
        },
        "sepia" => {
            Ok(colorfx::sepia(&img)?)
        },
        "contrast" => {
            let factor = transform.params["factor"].as_f64().unwrap_or(1.0) as f32;
            Ok(colorfx::contrast(&img, factor))
        },
        "saturation" => {
            let factor = transform.params["factor"].as_f64().unwrap_or(1.0) as f32;
            colorfx::saturation(&img, factor).map_err(|e| e.into())
        },
        "add_noise" => {
            let intensity = transform.params["intensity"].as_f64().unwrap_or(0.1) as f32;
            Ok(colorfx::add_noise(&img, intensity)?)
        },
        "deepfry" => {
            let factor = transform.params["factor"].as_f64().unwrap_or(1.0) as f32;
            Ok(colorfx::deep_fry(&img, factor)?)
        },
        "hue_rotate" => {
            let angle = transform.params["angle"].as_f64().unwrap_or(90.0) as f32;
            Ok(colorfx::hue_rotate(&img, angle)?)
        },
        "color_replacer" => {
            let target_color: String = transform.params["target_color"].to_string();
            let replacement_color: String = transform.params["replacement_color"].to_string();
            let tolerance: u8 = transform.params.get("tolerance").and_then(|v| v.as_u64()).unwrap_or(50) as u8;
            Ok(colorfx::color_replacer(&img, &target_color, &replacement_color, tolerance)?)
        },
        "vaporwave" => {
            Ok(colorfx::vaporwave(&img)?)
        },
        "neon_edge" => {
            let strength = transform.params["strength"].as_f64().unwrap_or(1.0) as f32;
            let color_shift = transform.params["color_shift"].as_f64().unwrap_or(0.0) as f32;
            let brightness = transform.params["brightness"].as_f64().unwrap_or(1.0) as f32;
            Ok(edgesfx::neon_edge(&img, strength, color_shift, brightness)?)
        },
        "sketch" => {
            let intensity = transform.params["intensity"].as_f64().unwrap_or(10.0) as f32;
            let contrast = transform.params["contrast"].as_f64().unwrap_or(1.0) as f32;
            let invert = transform.params["invert"].as_bool().unwrap_or(false);
            Ok(edgesfx::sketch(&img, intensity, contrast, invert)?)
        },
        "emboss" => {
            let strength = transform.params["strength"].as_f64().unwrap_or(100000.0) as f32;
            let angle = transform.params["angle"].as_f64().unwrap_or(45.0) as f32;
            Ok(edgesfx::emboss(&img, strength, angle)?)
        },
        "quantized_edge" => {
            let threshold = transform.params["threshold"].as_u64().unwrap_or(80) as f32;
            let level: u8 = transform.params["level"].as_u64().unwrap_or(1) as u8;
            Ok(edgesfx::quantized_edge(&img, level,threshold)?)
        },
        "extrusion_edge" => {
            let threshold = transform.params["threshold"].as_u64().unwrap_or(100) as f32;
            let strength = transform.params["strength"].as_f64().unwrap_or(2.0) as f32;
            let depth = transform.params["depth"].as_u64().unwrap_or(15) as u32;
            Ok(edgesfx::edge_extrusion(&img, strength, depth, threshold)?)
        },
        "blur" => {
            let sigma = transform.params["sigma"].as_f64().unwrap_or(2.0) as f32;
            Ok(img.blur(sigma))
        },
        "pixelate" => {
            let block_size = transform.params["block_size"].as_u64().unwrap_or(10) as u32;
            Ok(glitchfx::pixelate(&img, block_size))
        },
        "oil_painting" => {
            let radius = transform.params["radius"].as_u64().unwrap_or(4) as u32;
            let intensity = transform.params["intensity"].as_u64().unwrap_or(30) as u32;
            Ok(glitchfx::oil_painting(&img, radius, intensity.try_into().unwrap()))
        },
        "glitch" => {
            let amount = transform.params["amount"].as_u64().unwrap_or(50) as u32;
            let max_offset = transform.params["max_offset"].as_u64().unwrap_or(10) as i32;
            let direction: String = transform.params["direction"].to_string();
            let noisy_pixels: bool = transform.params["noisy"].as_bool().unwrap_or(false);
            Ok(glitchfx::glitch(&img, amount, max_offset, &direction, noisy_pixels))
        },
        "pixel_sort" => {
            let low_threshold: u8 = transform.params.get("low-threshold").and_then(|v| v.as_u64()).unwrap_or(150) as u8;
            let high_threshold: u8 = transform.params.get("high-threshold").and_then(|v| v.as_u64()).unwrap_or(200) as u8;
            let direction: String = transform.params["direction"].to_string();
            let window_size: usize = transform.params.get("window_size").and_then(|v| v.as_u64()).unwrap_or(100) as usize;
            Ok(glitchfx::pixel_sort(&img, &direction, low_threshold,high_threshold,  window_size))
        },
        "rotate" => {
            let angle = transform.params["angle"].as_u64().unwrap_or(90) as f32;
            Ok(glitchfx::rotate(&img, angle)?)
        },
        "desync" => {
            let x_shift = transform.params["x_shift"].as_i64().unwrap_or(10) as i32;
            let y_shift = transform.params["y_shift"].as_i64().unwrap_or(10) as i32;
            Ok(glitchfx::desync(&img, x_shift, y_shift)?)
        },
        "wind" => {
            let direction = transform.params["direction"].as_str().unwrap_or("right").to_string();
            let strength: u32 = transform.params["strength"].as_u64().unwrap_or(10) as u32;
            Ok(glitchfx::wind(&img, &direction, strength)?)
        },
        "scan_lines" => {
            let line_thickness = transform.params["line_thickness"].as_u64().unwrap_or(2) as u32;
            let line_spacing = transform.params["line_spacing"].as_u64().unwrap_or(10) as u32;
            let opacity = transform.params["opacity"].as_f64().unwrap_or(0.5) as f32;
            let angle = transform.params["angle"].as_f64().unwrap_or(0.0) as f32;
            Ok(glitchfx::scan_lines(&img, Some(line_thickness), Some(line_spacing), Some(angle), Some(opacity))?)
        },
        _ => {
            print!("Invalid transformation specified: {}", transform.name);
            Ok(img)
        }
    }
}
