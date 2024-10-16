// File: src/fx_json_generator.rs

use rand::prelude::*;
use serde_json::{json, Value};

pub fn generate_random_pipeline(num_effects: usize) -> Value {
    let mut rng = rand::thread_rng();
    let effects = vec![
        "grayscale", "invert", "brightness", "sepia", "contrast", "saturation",
        "add_noise", "deepfry", "hue_rotate", "vaporwave",
        "blur", "pixelate", "oil_painting", "glitch", "pixel_sort", "rotate",
        "desync", "wind", "scan_lines", "neon_edge", "sketch", "emboss",
        "quantized_edge", "extrusion_edge"
    ];

    let transformations: Vec<Value> = (0..num_effects)
        .map(|_| {
            let effect = effects.choose(&mut rng).unwrap();
            json!({
                "name": effect,
                "params": generate_params(effect, &mut rng)
            })
        })
        .collect();

    json!({ "transformations": transformations })
}

fn generate_params(effect: &str, rng: &mut impl Rng) -> Value {
    match effect {
        "grayscale" | "invert" | "sepia" | "vaporwave" => json!({}),
        "brightness" | "contrast" | "saturation" => {
            json!({ "factor": rng.gen_range(0.5..2.0) })
        },
        "add_noise" => json!({ "intensity": rng.gen_range(0.05..0.3) }),
        "deepfry" => json!({ "factor": rng.gen_range(0.5..2.0) }),
        "hue_rotate" => json!({ "angle": rng.gen_range(0.0..360.0) }),
        "blur" => json!({ "sigma": rng.gen_range(0.5..5.0) }),
        "pixelate" => json!({ "block_size": rng.gen_range(5..20) }),
        "oil_painting" => json!({
            "radius": rng.gen_range(1..10),
            "intensity": rng.gen_range(10..50)
        }),
        "glitch" => json!({
            "amount": rng.gen_range(10..100),
            "max_offset": rng.gen_range(5..20),
            "direction": random_choice(rng, &["horizontal", "vertical"]),
            "noisy": rng.gen_bool(0.5)
        }),
        "pixel_sort" => json!({
            "low-threshold": rng.gen_range(0..100),
            "high-threshold": rng.gen_range(100..255),
            "direction": random_choice(rng, &["horizontal", "vertical"]),
            "window_size": rng.gen_range(0..50)
        }),
        "rotate" => json!({ "angle": rng.gen_range(0.0..360.0) }),
        "desync" => json!({
            "x_shift": rng.gen_range(-20..21),
            "y_shift": rng.gen_range(-20..21)
        }),
        "wind" => json!({
            "direction": random_choice(rng, &["left", "right"]),
            "strength": rng.gen_range(5..20)
        }),
        "scan_lines" => json!({
            "line_thickness": rng.gen_range(1..5),
            "line_spacing": rng.gen_range(5..20),
            "opacity": rng.gen_range(0.1..1.0),
            "angle": rng.gen_range(0.0..180.0)
        }),
        "neon_edge" => json!({
            "strength": rng.gen_range(0.5..2.0),
            "color_shift": rng.gen_range(0.0..1.0),
            "brightness": rng.gen_range(0.5..2.0)
        }),
        "sketch" => json!({
            "intensity": rng.gen_range(5.0..20.0),
            "contrast": rng.gen_range(0.5..2.0),
            "invert": rng.gen_bool(0.5)
        }),
        "emboss" => json!({
            "strength": rng.gen_range(50000.0..150000.0),
            "angle": rng.gen_range(0.0..360.0)
        }),
        "quantized_edge" => json!({
            "threshold": rng.gen_range(50.0..150.0),
            "level": rng.gen_range(1..5)
        }),
        "extrusion_edge" => json!({
            "threshold": rng.gen_range(50.0..150.0),
            "strength": rng.gen_range(1.0..5.0),
            "depth": rng.gen_range(5..30)
        }),
        _ => json!({})
    }
}

fn random_choice<T>(rng: &mut impl Rng, choices: &[T]) -> T 
where
    T: Clone,
{
    choices.choose(rng).unwrap().clone()
}