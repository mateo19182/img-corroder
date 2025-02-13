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
    
    let mut rgb_image = img.to_rgb8();
    for (_, _, pixel) in rgb_image.enumerate_pixels_mut() {
        if color_distance(pixel, target_rgb) <= tolerance {
            *pixel = *replacement_rgb;
        }
    }

    Ok(DynamicImage::ImageRgb8(rgb_image))
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

// Helper function to generate a Bayer matrix of size n x n (n must be a power of two)
fn generate_bayer_matrix(n: u32) -> Vec<Vec<f32>> {
    if n == 2 {
        vec![vec![0.0, 2.0],
             vec![3.0, 1.0]]
    } else {
        let half = n / 2;
        let smaller = generate_bayer_matrix(half);
        let mut matrix = vec![vec![0.0; n as usize]; n as usize];
        for i in 0..half {
            for j in 0..half {
                let val = smaller[i as usize][j as usize];
                matrix[i as usize][j as usize] = 4.0 * val;
                matrix[i as usize][(j + half) as usize] = 4.0 * val + 2.0;
                matrix[(i + half) as usize][j as usize] = 4.0 * val + 3.0;
                matrix[(i + half) as usize][(j + half) as usize] = 4.0 * val + 1.0;
            }
        }
        matrix
    }
}

// Dithering filter: applies ordered dithering effect using a Bayer matrix.
// 'levels' is the number of quantization levels (minimum 2).
// 'size' is the requested dimension for the Bayer matrix; if not a power of two, defaults to 4.
pub fn dither(
    img: &DynamicImage,
    levels: u8,
    matrix_size: Option<u32>,
    point_size: Option<u32>,
    threshold_bias: Option<f32>
) -> Result<DynamicImage, String> {
    if levels < 2 {
        return Err("Levels must be at least 2".to_string());
    }

    // Set defaults and validate
    let matrix_size = matrix_size.unwrap_or(4);
    let matrix_size = if matrix_size.is_power_of_two() { matrix_size } else { 4 };
    let point_size = point_size.unwrap_or(1).max(1); // Ensure at least 1
    let bias = threshold_bias.unwrap_or(0.0);

    // Generate the Bayer matrix of the specified size
    let mut bayer_matrix = generate_bayer_matrix(matrix_size);
    
    // Normalize the matrix values to [0, 1] range
    let divisor = (matrix_size * matrix_size) as f32;
    for row in bayer_matrix.iter_mut() {
        for val in row.iter_mut() {
            *val = (*val + 0.5) / divisor;
        }
    }

    let (width, height) = img.dimensions();
    let mut output = image::DynamicImage::new_rgba8(width, height);
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut new_pixel = pixel; // preserve alpha
            for c in 0..3 {
                let old_val = pixel[c] as f32;
                let normalized = old_val / 255.0;
                let scaled = normalized * ((levels - 1) as f32);
                let threshold = bayer_matrix[((y / point_size) % matrix_size) as usize][((x / point_size) % matrix_size) as usize] + bias;
                let new_level = (scaled + threshold).floor().min((levels - 1) as f32);
                let new_val = (new_level * (255.0 / ((levels - 1) as f32))).round();
                new_pixel[c] = new_val.clamp(0.0, 255.0) as u8;
            }
            output.put_pixel(x, y, new_pixel);
        }
    }

    Ok(output)
}