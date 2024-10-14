
use image::{DynamicImage, GenericImage, GenericImageView, RgbImage, Rgba, RgbaImage};
use rand::Rng;
use asdf_pixel_sort::{sort_with_options, PColor, Options, Mode, Direction};

pub fn sort_pixel(img: &DynamicImage, mode: &String, direction: &String, threshold: Option<u8>) -> DynamicImage {
    // Convert the DynamicImage to an RgbImage
    let mut buf: RgbImage = img.to_rgb8();

    // Define a default color and use the provided color if available
    let (r, g, b) = (0, 62, 214);
    let pcolor = PColor::new(r, g, b);

    // Remove surrounding quotes and whitespace, then convert to lowercase
    let clean_mode = mode.trim().trim_matches('"').to_lowercase();
    let clean_direction: String = direction.trim().trim_matches('"').to_lowercase();

    let options = Options {
        mode: match clean_mode.as_str() {
            "black" => Mode::Black(pcolor),
            "white" => Mode::White(pcolor),
            "brightness" => Mode::Brightness(threshold.unwrap_or(50)),
            _ => {
                println!("Unrecognized mode: {}, defaulting to black", clean_mode);
                Mode::Black(pcolor)
            },
        },
        direction: match clean_direction.as_str() {
            "row" => Direction::Row,
            "column" => Direction::Column,
            "both" => Direction::Both,
            _ => {
                println!("Unrecognized direction: {}, defaulting to both", clean_direction);
                Direction::Both
            },
        },
    };

    // Apply the sort_with_options function
    sort_with_options(&mut buf, &options);
    print!("Applied sort_pixel with options: {:?}", options);

    // Convert the buffer back to a DynamicImage and return it
    DynamicImage::ImageRgb8(buf)
}

pub fn rotate(img: &DynamicImage, angle: f32) -> Result<DynamicImage, String> {
    print!("Applied rotate with angle: {}", angle);

    match angle.rem_euclid(360.0) {
        0.0 => Ok(img.clone()),
        90.0 => Ok(img.rotate90()),
        180.0 => Ok(img.rotate180()),
        270.0 => Ok(img.rotate270()),
        _ => Err("Invalid angle specified".into()),
    }
}

pub fn desync(img: &DynamicImage, x_shift: i32, y_shift: i32) -> Result<DynamicImage, String> {
    print!("Applied desync with x_shift: {}, y_shift: {}", x_shift, y_shift);

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

pub fn wind(img: &DynamicImage, direction: &String, strength: u32) -> Result<DynamicImage, String> {
    print!("Applied wind effect with direction: {:?}, strength: {}", direction, strength);

    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut accumulator = [0u32; 4];
            let mut count = 0;
            for i in 0..strength {
                let (sample_x, sample_y) = match direction.as_str() {
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
    print!("Applied pixelate with block_size: {}", block_size);
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
    print!("Applied oil_painting with radius: {}, intensity_levels: {}", radius, intensity_levels);
    output
}

pub fn scan_lines(
    img: &DynamicImage,
    line_thickness: Option<u32>,
    line_spacing: Option<u32>,
    angle: Option<f32>,
    opacity: Option<f32>,
) -> Result<DynamicImage, String> {
    let mut rng = rand::thread_rng();

    // Use provided values or generate random defaults
    let thickness = line_thickness.unwrap_or_else(|| rng.gen_range(1..=5));
    let spacing = line_spacing.unwrap_or_else(|| rng.gen_range(5..=20));
    let angle_rad = angle.unwrap_or_else(|| rng.gen_range(0.0..std::f32::consts::PI));
    let opacity = opacity.unwrap_or_else(|| rng.gen_range(0.3..=0.7));

    print!("Applied scan lines with thickness: {}, spacing: {}, angle: {:.2}rad, opacity: {:.2}", 
             thickness, spacing, angle_rad, opacity);

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

pub fn glitch(
    img: &DynamicImage,
    num_glitches: u32,
    max_offset: i32,
    direction: &str,
    noisy: bool,
) -> DynamicImage {
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

    print!("Applied glitch with num_glitches: {}, max_offset: {}, direction: {}, noisy: {}", num_glitches, max_offset, direction, noisy);
    output
}

fn apply_vertical_glitch(
    output: &mut DynamicImage,
    img: &DynamicImage,
    x: u32,
    glitch_width: u32,
    offset: i32,
    width: u32,
    height: u32,
    noisy: bool,
) {
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

fn apply_horizontal_glitch(
    output: &mut DynamicImage,
    img: &DynamicImage,
    y: u32,
    glitch_height: u32,
    offset: i32,
    width: u32,
    height: u32,
    noisy: bool,
) {
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

fn noisy_pixels(_pixel: Rgba<u8>) -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    Rgba([
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        255,
    ])
}