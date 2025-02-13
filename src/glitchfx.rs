use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba, RgbaImage};
use rand::Rng;

pub fn pixel_sort(img: &DynamicImage, direction: &str, low_threshold:u8, high_threshold:u8, window_size:usize) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);
    let clean_direction = direction.trim().trim_matches('"').to_lowercase();

    match clean_direction.as_str() { "row" => sort_pixels(img, &mut output, low_threshold, high_threshold,  window_size),
        "column" => {
            let rotated = img.rotate90();
            let mut rotated_output = ImageBuffer::new(height, width);
            sort_pixels(&rotated, &mut rotated_output, low_threshold, high_threshold, window_size);
            output = DynamicImage::ImageRgb8(rotated_output).rotate270().to_rgb8();
        },
        "both" => {
            sort_pixels(img, &mut output, low_threshold, high_threshold, window_size);
            let temp = DynamicImage::ImageRgb8(output);
            let rotated = temp.rotate90();
            let mut rotated_output = ImageBuffer::new(height, width);
            sort_pixels(&rotated, &mut rotated_output, low_threshold, high_threshold, window_size);
            output = DynamicImage::ImageRgb8(rotated_output).rotate270().to_rgb8();
        },
        _ => {
            println!("Unrecognized direction: {}, defaulting to row", direction);
            sort_pixels(img, &mut output, low_threshold, high_threshold, window_size);
        }
    }

    DynamicImage::ImageRgb8(output)
}
pub fn sort_pixels(img: &DynamicImage, output: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, low_threshold: u8, high_threshold: u8, window_size: usize) {
    for y in 0..img.height() {
        let mut row: Vec<(u8, Rgb<u8>)> = (0..img.width())
            .map(|x| {
                let pixel = img.get_pixel(x, y).to_rgb();
                let brightness = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
                (brightness, pixel)
            })
            .collect();

        if window_size == 0 || window_size >= img.width().try_into().unwrap() {
            // Sort the entire row
            row.sort_by_key(|&(brightness, _)| {
                if brightness >= low_threshold && brightness <= high_threshold {
                    brightness
                } else if brightness < low_threshold {
                    low_threshold - 1 // Place below threshold pixels at the start
                } else {
                    u8::MAX // Place above threshold pixels at the end
                }
            });
        } else {
            // Window-based sorting
            let mut sorted_row = Vec::new();
            for chunk in row.chunks(window_size) {
                let mut window_vec = chunk.to_vec();
                window_vec.sort_by_key(|&(brightness, _)| {
                    if brightness >= low_threshold && brightness <= high_threshold {
                        brightness
                    } else if brightness < low_threshold {
                        low_threshold - 1 // Place below threshold pixels at the start
                    } else {
                        u8::MAX // Place above threshold pixels at the end
                    }
                });
                sorted_row.extend(window_vec);
            }
            row = sorted_row;
        }

        for (x, &(_, pixel)) in row.iter().enumerate() {
            output.put_pixel(x as u32, y, pixel);
        }
    }
}

pub fn rotate(img: &DynamicImage, angle: f32) -> Result<DynamicImage, String> {

    match angle.rem_euclid(360.0) {
        0.0 => Ok(img.clone()),
        90.0 => Ok(img.rotate90()),
        180.0 => Ok(img.rotate180()),
        270.0 => Ok(img.rotate270()),
        _ => Err("Invalid angle specified".into()),
    }
}

pub fn desync(img: &DynamicImage, x_shift: i32, y_shift: i32) -> Result<DynamicImage, String> {

    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            
            // Shift red channel
            let red_x = (x as i32 + x_shift).rem_euclid(width as i32) as u32;
            let red_y = (y as i32 + y_shift).rem_euclid(height as i32) as u32;
            output.put_pixel(red_x, red_y, Rgba([pixel[0], 0, 0, 255]));

            // Keep green channel in place
            output.get_pixel_mut(x, y)[1] = pixel[1];

            // Shift blue channel in opposite direction
            let blue_x = (x as i32 - x_shift).rem_euclid(width as i32) as u32;
            let blue_y = (y as i32 - y_shift).rem_euclid(height as i32) as u32;
            output.get_pixel_mut(blue_x, blue_y)[2] = pixel[2];
        }
    }

    Ok(DynamicImage::ImageRgba8(output))
}

pub fn wind(img: &DynamicImage, direction: &str, strength: u32) -> Result<DynamicImage, String> {

    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut accumulator = [0u32; 4];
            let mut count = 0;
            for i in 0..strength {
                let (sample_x, sample_y) = match direction {
                    "up" => (x, (y + i).min(height - 1)),
                    "down" => (x, y.saturating_sub(i)),
                    "left" => ((x + i).min(width - 1), y),
                    "right" => (x.saturating_sub(i), y),
                    _ => (x, y), // Default case if direction is unrecognized
                };

                let pixel = img.get_pixel(sample_x, sample_y);
                for c in 0..4 {
                    accumulator[c] += pixel[c] as u32;
                }
                count += 1;
            }

            let final_pixel = Rgba([
                (accumulator[0] / count) as u8,
                (accumulator[1] / count) as u8,
                (accumulator[2] / count) as u8,
                (accumulator[3] / count) as u8,
            ]);

            output.put_pixel(x, y, final_pixel);
        }
    }

    Ok(DynamicImage::ImageRgba8(output))
}

pub fn pixelate(img: &DynamicImage, block_size: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = DynamicImage::new_rgba8(width, height);

    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            let pixel = img.get_pixel(x, y);
            for by in 0..block_size {
                for bx in 0..block_size {
                    if x + bx < width && y + by < height {
                        output.put_pixel(x + bx, y + by, pixel);
                    }
                }
            }
        }
    }
    output
}

pub fn oil_painting(img: &DynamicImage, radius: u32, intensity_levels: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = DynamicImage::new_rgba8(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut intensity_count = vec![0; intensity_levels as usize];
            let mut avg_r = vec![0; intensity_levels as usize];
            let mut avg_g = vec![0; intensity_levels as usize];
            let mut avg_b = vec![0; intensity_levels as usize];

            for dy in y.saturating_sub(radius)..=y.saturating_add(radius) {
                for dx in x.saturating_sub(radius)..=x.saturating_add(radius) {
                    if dx < width && dy < height {
                        let pixel = img.get_pixel(dx, dy);
                        let intensity = ((pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3 * intensity_levels as u16 / 256) as usize;
                        intensity_count[intensity] += 1;
                        avg_r[intensity] += pixel[0] as u32;
                        avg_g[intensity] += pixel[1] as u32;
                        avg_b[intensity] += pixel[2] as u32;
                    }
                }
            }

            let max_intensity = intensity_count.iter().enumerate().max_by_key(|&(_, &count)| count).unwrap().0;
            let count = intensity_count[max_intensity];
            let r = (avg_r[max_intensity] / count) as u8;
            let g = (avg_g[max_intensity] / count) as u8;
            let b = (avg_b[max_intensity] / count) as u8;

            output.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    output
}

pub fn scan_lines(img: &DynamicImage,line_thickness: Option<u32>,line_spacing: Option<u32>,angle: Option<f32>,opacity: Option<f32>) -> Result<DynamicImage, String> {
    let mut rng = rand::thread_rng();
    // Use provided values or generate random defaults
    let thickness = line_thickness.unwrap_or_else(|| rng.gen_range(1..=5));
    let spacing = line_spacing.unwrap_or_else(|| rng.gen_range(5..=20));
    let angle_rad = angle.unwrap_or_else(|| rng.gen_range(0.0..std::f32::consts::PI));
    let opacity = opacity.unwrap_or_else(|| rng.gen_range(0.3..=0.7));

    let (width, height) = img.dimensions();
    let mut output = img.to_rgba8();

    let sin_angle = angle_rad.sin();
    let cos_angle = angle_rad.cos();

    for y in 0..height {
        for x in 0..width {
            // Calculate the position along the scan line direction
            let pos = (x as f32 * cos_angle + y as f32 * sin_angle) as i32;
            
            // Determine if this pixel is part of a scan line
            if pos.rem_euclid((thickness + spacing) as i32) < thickness as i32 {
                let pixel = output.get_pixel_mut(x, y);
                
                // Adjust pixel color to create scan line effect
                for c in 0..3 {
                    pixel[c] = ((1.0 - opacity) * pixel[c] as f32) as u8;
                }
            }
        }
    }

    Ok(DynamicImage::ImageRgba8(output))
}

pub fn glitch(img: &DynamicImage, num_glitches: u32, max_offset: i32,direction: &str, noisy: bool) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = img.clone();
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    for _ in 0..num_glitches {
        let (start, length, is_vertical) = match direction.trim().trim_matches('"').to_lowercase().as_str() {
            "vertical" => (rng.gen_range(0..width), rng.gen_range(1..20), true),
            "horizontal" => (rng.gen_range(0..height), rng.gen_range(1..20), false),
            _ => panic!("Invalid direction. Use 'vertical', 'horizontal'."),
        };

        let offset = rng.gen_range(-max_offset..=max_offset);

        if is_vertical {
            apply_vertical_glitch(&mut output, img, start, length, offset, width, height, noisy);
        } else {
            apply_horizontal_glitch(&mut output, img, start, length, offset, width, height, noisy);
        }
    }

    output
}

pub fn apply_vertical_glitch(output: &mut DynamicImage, img: &DynamicImage, x: u32, glitch_width: u32, offset: i32, width: u32, height: u32, noisy: bool) {
    for dx in x..x + glitch_width {
        if dx < width {
            for y in 0..height {
                let source_y = (y as i32 + offset).rem_euclid(height as i32) as u32;
                let mut pixel = img.get_pixel(dx, source_y);
                if noisy {
                    pixel = noisy_pixels(pixel);
                }
                output.put_pixel(dx, y, pixel);
            }
        }
    }
}

pub fn apply_horizontal_glitch( output: &mut DynamicImage, img: &DynamicImage, y: u32, glitch_height: u32, offset: i32, width: u32, height: u32, noisy: bool) {
    for dy in y..y + glitch_height {
        if dy < height {
            for x in 0..width {
                let source_x = (x as i32 + offset).rem_euclid(width as i32) as u32;
                let mut pixel = img.get_pixel(source_x, dy);
                if noisy {
                    pixel = noisy_pixels(pixel);
                }
                output.put_pixel(x, dy, pixel);
            }
        }
    }
}

pub fn noisy_pixels(_pixel: Rgba<u8>) -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    Rgba([
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        255,
    ])
}