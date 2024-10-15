use image::{DynamicImage, Rgb};
use rand::Rng;
mod utils;
use utils::{color_distance, create_color_map, hsv_to_rgb, rgb_to_hsv};

pub fn contrast(img: &DynamicImage, factor: f32) -> Result<DynamicImage, String> {
    println!("Applied contrast adjustment with factor: {}", factor);
    
    if factor < 0.0 {
        return Err("Contrast factor must be non-negative".into());
    }
    
    let mut contrasted_img = img.clone();
    
    for (_, _, pixel) in contrasted_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        for c in 0..3 {
            let value = pixel[c] as f32;
            let adjusted = 128.0 + factor * (value - 128.0);
            pixel[c] = adjusted.min(255.0).max(0.0) as u8;
        }
    }
    
    Ok(DynamicImage::ImageRgb8(contrasted_img.to_rgb8()))
}


pub fn saturation(img: &DynamicImage, factor: f32) -> Result<DynamicImage, String> {
    println!("Applied saturation adjustment with factor: {}", factor);
    
    if factor < 0.0 {
        return Err("Saturation factor must be non-negative".into());
    }
    
    let mut saturated_img = img.clone();
    
    for (_, _, pixel) in saturated_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let mut hsv = rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
        hsv.1 = (hsv.1 * factor).min(1.0);
        let rgb = hsv_to_rgb(hsv.0, hsv.1, hsv.2);
        *pixel = Rgb([rgb.0, rgb.1, rgb.2]);
    }
    
    Ok(DynamicImage::ImageRgb8(saturated_img.to_rgb8()))
}


pub fn add_noise(img: &DynamicImage, intensity: f32) -> Result<DynamicImage, String> {
    println!("Applied noise with intensity: {}", intensity);
    
    if intensity < 0.0 || intensity > 1.0 {
        return Err("Noise intensity must be between 0.0 and 1.0".into());
    }
    
    let mut noisy_img = img.clone();
    let mut rng = rand::thread_rng();
    
    for (_, _, pixel) in noisy_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        if rng.gen::<f32>() < intensity {
            for c in 0..3 {
                let noise = rng.gen_range(-50..=50);
                pixel[c] = (pixel[c] as i16 + noise).clamp(0, 255) as u8;
            }
        }
    }
    
    Ok(DynamicImage::ImageRgb8(noisy_img.to_rgb8()))
}

pub fn brightness(img: &DynamicImage, factor: f32) -> Result<DynamicImage, String> {
    println!("Applied brightness adjustment with factor: {}", factor);
    
    if factor < 0.0 || factor > 2.0 {
        return Err("Brightness factor must be between 0.0 and 2.0".into());
    }
    
    let mut adjusted_img = img.clone();
    
    for (_, _, pixel) in adjusted_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        pixel[0] = ((pixel[0] as f32 * factor).min(255.0)) as u8;
        pixel[1] = ((pixel[1] as f32 * factor).min(255.0)) as u8;
        pixel[2] = ((pixel[2] as f32 * factor).min(255.0)) as u8;
    }
    
    Ok(DynamicImage::ImageRgb8(adjusted_img.to_rgb8()))
}



pub fn sepia(img: &DynamicImage) -> Result<DynamicImage, String> {
    println!("Applied sepia filter");
    let mut sepia_img = img.clone();
    for (_, _, pixel) in sepia_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let sepia_r = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
        let sepia_g = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
        let sepia_b = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;
        *pixel = image::Rgb([sepia_r, sepia_g, sepia_b]);
    }
    Ok(DynamicImage::ImageRgb8(sepia_img.to_rgb8()))
}

pub fn color_replacer(img: &DynamicImage, target_color: &str, replacement_color: &str, tolerance: u8) -> Result<DynamicImage, String> {
    println!("Applied color replacer: {} to {}", target_color, replacement_color);
    
    let color_map = create_color_map();
    let target_rgb = color_map.get(target_color)
        .ok_or_else(|| format!("Invalid target color: {}", target_color))?;
    let replacement_rgb = color_map.get(replacement_color)
        .ok_or_else(|| format!("Invalid replacement color: {}", replacement_color))?;
    
    let mut replaced_img = img.clone();
    
    for (_, _, pixel) in replaced_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        if color_distance(pixel, target_rgb) <= tolerance {
            *pixel = *replacement_rgb;
        }
    }
    
    Ok(DynamicImage::ImageRgb8(replaced_img.to_rgb8()))
}

pub fn vaporwave(img: &DynamicImage) -> Result<DynamicImage, String> {
    println!("Applied vaporwave aesthetic filter");
    let mut vapor_img = img.clone();
    for (_, _, pixel) in vapor_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        pixel[0] = ((pixel[0] as f32 * 0.8 + 50.0).min(255.0)) as u8; // Boost red
        pixel[2] = ((pixel[2] as f32 * 1.2 + 30.0).min(255.0)) as u8; // Boost blue
        // Add a slight purple tint
        pixel[0] = ((pixel[0] as f32 * 0.9 + 25.0).min(255.0)) as u8;
        pixel[1] = ((pixel[1] as f32 * 0.8).min(255.0)) as u8;
        pixel[2] = ((pixel[2] as f32 * 1.1).min(255.0)) as u8;
    }
    Ok(DynamicImage::ImageRgb8(vapor_img.to_rgb8()))
}

pub fn deep_fry(img: &DynamicImage) -> Result<DynamicImage, String> {
    println!("Applied deep-fried effect");
    let contrasted = contrast(img, 2.0)?;
    let brightened = brightness(&contrasted, 1.4)?;
    let saturated = saturation(&brightened, 1.8)?;
    let noisy = add_noise(&saturated, 0.05)?;
    Ok(noisy)
}

pub fn hue_rotate(img: &DynamicImage, angle: f32) -> Result<DynamicImage, String> {
    println!("Rotated hue by {}", angle);
    let mut rotated_img = img.clone();
    for (_, _, pixel) in rotated_img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let hsv = rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
        let new_hue = (hsv.0 + angle) % 360.0;
        let rgb = hsv_to_rgb(new_hue, hsv.1, hsv.2);
        *pixel = Rgb([rgb.0, rgb.1, rgb.2]);
    }
    Ok(DynamicImage::ImageRgb8(rotated_img.to_rgb8()))
}