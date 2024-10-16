use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use rand::Rng;
mod utils;
use utils::{color_distance, create_color_map, hsv_to_rgb, rgb_to_hsv};

pub fn contrast(img: &DynamicImage, factor: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = DynamicImage::new_rgba8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut adjusted = Rgba([0, 0, 0, pixel[3]]); // Preserve alpha

            for c in 0..3 {
                let value = pixel[c] as f32;
                let adjusted_value = 128.0 + factor * (value - 128.0);
                adjusted[c] = adjusted_value.min(255.0).max(0.0) as u8;
            }

            output.put_pixel(x, y, adjusted);
        }
    }

    output
}


pub fn saturation(img: &DynamicImage, factor: f32) -> Result<DynamicImage, String> {
    
    if factor < 0.0 {
        return Err("Saturation factor must be non-negative".into());
    }
    
    let (width, height) = img.dimensions();
    let mut output = DynamicImage::new_rgba8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut hsv = rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
            hsv.1 = (hsv.1 * factor).min(1.0);
            let rgb = hsv_to_rgb(hsv.0, hsv.1, hsv.2);
            output.put_pixel(x, y, Rgba([rgb.0, rgb.1, rgb.2, pixel[3]])); // Preserve alpha
        }
    }

    Ok(output)
}



pub fn add_noise(img: &DynamicImage, intensity: f32) -> Result<DynamicImage, String> {
    
    if intensity < 0.0 || intensity > 1.0 {
        return Err("Noise intensity must be between 0.0 and 1.0".into());
    }
    
    let (width, height) = img.dimensions();
    let mut output = DynamicImage::new_rgba8(width, height);
    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y);
            
            if rng.gen::<f32>() < intensity {
                for c in 0..3 {  // Only apply noise to RGB channels, not alpha
                    let noise = rng.gen_range(-50..=50);
                    pixel[c] = (pixel[c] as i16 + noise).clamp(0, 255) as u8;
                }
            }
            
            output.put_pixel(x, y, pixel);
        }
    }

    Ok(output)
}

pub fn brightness(img: &DynamicImage, factor: f32) -> Result<DynamicImage, String> {
    
    if factor < 0.0 || factor > 2.0 {
        return Err("Brightness factor must be between 0.0 and 2.0".into());
    }
    
    let (width, height) = img.dimensions();
    let mut output = DynamicImage::new_rgba8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut adjusted = Rgba([0, 0, 0, pixel[3]]); // Preserve alpha

            for c in 0..3 {
                adjusted[c] = ((pixel[c] as f32 * factor).min(255.0)) as u8;
            }

            output.put_pixel(x, y, adjusted);
        }
    }

    Ok(output)
}

pub fn sepia(img: &DynamicImage) -> Result<DynamicImage, String> {
    let (width, height) = img.dimensions();
    let mut sepia_img = DynamicImage::new_rgba8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            
            let r = rgb[0] as f32;
            let g = rgb[1] as f32;
            let b = rgb[2] as f32;

            let sepia_r = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
            let sepia_g = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
            let sepia_b = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;

            sepia_img.put_pixel(x, y, Rgba([sepia_r, sepia_g, sepia_b, pixel[3]]));
        }
    }

    Ok(sepia_img)
}

pub fn color_replacer(img: &DynamicImage, target_color: &str, replacement_color: &str, tolerance: u8) -> Result<DynamicImage, String> {
    
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
    let (width, height) = img.dimensions();
    let mut vapor_img = DynamicImage::new_rgb8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            
            let mut new_pixel = [0u8; 3];
            new_pixel[0] = ((rgb[0] as f32 * 0.8 + 50.0).min(255.0)) as u8; // Boost red
            new_pixel[2] = ((rgb[2] as f32 * 1.2 + 30.0).min(255.0)) as u8; // Boost blue
            // Add a slight purple tint
            new_pixel[0] = ((new_pixel[0] as f32 * 0.9 + 25.0).min(255.0)) as u8;
            new_pixel[1] = ((rgb[1] as f32 * 0.8).min(255.0)) as u8;
            new_pixel[2] = ((new_pixel[2] as f32 * 1.1).min(255.0)) as u8;

            vapor_img.put_pixel(x, y, Rgba([new_pixel[0], new_pixel[1], new_pixel[2], 255]));
        }
    }

    Ok(vapor_img)
}

pub fn deep_fry(img: &DynamicImage, factor: f32) -> Result<DynamicImage, String> {
    let contrasted = contrast(img, 2.0*factor);
    let brightened = brightness(&contrasted, 1.5+factor/10.0)?;
    let saturated = saturation(&brightened, 1.8*factor)?;
    let noisy = add_noise(&saturated, 0.05+factor/10.0)?;
    Ok(noisy)
}

pub fn hue_rotate(img: &DynamicImage, angle: f32) -> Result<DynamicImage, String> {
    let (width, height) = img.dimensions();
    let mut rotated_img = DynamicImage::new_rgb8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            
            let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
            let new_hue = (hsv.0 + angle) % 360.0;
            let new_rgb = hsv_to_rgb(new_hue, hsv.1, hsv.2);
            
            rotated_img.put_pixel(x, y, Rgba([new_rgb.0, new_rgb.1, new_rgb.2, pixel[3]]));
        }
    }

    Ok(rotated_img)
}