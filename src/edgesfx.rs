use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb,Rgba, Luma};
use std::f32::consts::PI;

// Helper function to apply convolution
fn convolve(img: &DynamicImage, kernel: &[[f32; 3]; 3]) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);

    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut sum = (0.0, 0.0, 0.0);
            for ky in 0..3 {
                for kx in 0..3 {
                    let pixel = img.get_pixel(x + kx - 1, y + ky - 1);
                    sum.0 += pixel[0] as f32 * kernel[ky as usize][kx as usize];
                    sum.1 += pixel[1] as f32 * kernel[ky as usize][kx as usize];
                    sum.2 += pixel[2] as f32 * kernel[ky as usize][kx as usize];
                }
            }
            output.put_pixel(x, y, Rgb([
                sum.0.clamp(0.0, 255.0) as u8,
                sum.1.clamp(0.0, 255.0) as u8,
                sum.2.clamp(0.0, 255.0) as u8
            ]));
        }
    }

    DynamicImage::ImageRgb8(output)
}

pub fn neon_edge(img: &DynamicImage, strength: f32, color_shift: f32, brightness: f32) -> Result<DynamicImage, String> {
    println!("Applying Neon Edge filter with strength: {}, color_shift: {}, brightness: {}", strength, color_shift, brightness);
    
    let sobel_x = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];
    let sobel_y = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

    let edges_x = convolve(img, &sobel_x);
    let edges_y = convolve(img, &sobel_y);

    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let px_x = edges_x.get_pixel(x, y);
            let px_y = edges_y.get_pixel(x, y);
            let edge_strength = (
                (px_x[0] as f32).hypot(px_y[0] as f32),
                (px_x[1] as f32).hypot(px_y[1] as f32),
                (px_x[2] as f32).hypot(px_y[2] as f32),
            );

            // Apply customizable neon effect
            output.put_pixel(x, y, Rgb([
                ((edge_strength.0 * strength + color_shift) * brightness).clamp(0.0, 255.0) as u8,
                ((edge_strength.1 * strength) * brightness).clamp(0.0, 255.0) as u8,
                ((edge_strength.2 * strength - color_shift) * brightness).clamp(0.0, 255.0) as u8
            ]));
        }
    }

    Ok(DynamicImage::ImageRgb8(output))
}

pub fn sketch(img: &DynamicImage, intensity: f32, contrast: f32, invert: bool) -> Result<DynamicImage, String> {
    println!("Applying Sketch filter with intensity: {}, contrast: {}, invert: {}", intensity, contrast, invert);
    
    let laplacian = [[0.0, 1.0, 0.0], [1.0, -4.0, 1.0], [0.0, 1.0, 0.0]];

    let edges = convolve(img, &laplacian);
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let edge_px = edges.get_pixel(x, y);
            let mut sketch_value = (edge_px[0] as f32 * 0.3 + edge_px[1] as f32 * 0.59 + edge_px[2] as f32 * 0.11) as f32;
            
            // Apply intensity and contrast
            sketch_value = ((sketch_value - 128.0) * contrast + 128.0) * intensity;
            sketch_value = sketch_value.clamp(0.0, 255.0);
            
            // Invert if requested
            if invert {
                sketch_value = 255.0 - sketch_value;
            }

            let final_value = sketch_value as u8;
            output.put_pixel(x, y, Rgb([final_value, final_value, final_value]));
        }
    }

    Ok(DynamicImage::ImageRgb8(output))
}

pub fn emboss(img: &DynamicImage, strength: f32, angle: f32) -> Result<DynamicImage, String> {
    println!("Applying emboss filter with strength {} and angle {}", strength, angle);

    // Calculate kernel based on angle and strength
    let (dx, dy) = angle.to_radians().sin_cos();
    let kernel = [
        [-1.0 * strength * dx, -1.0 * strength * dy, 0.0],
        [-1.0 * strength * dy, 0.0, 1.0 * strength * dy],
        [0.0, 1.0 * strength * dx, 1.0 * strength * dy],
    ];

    let embossed = convolve(img, &kernel);

    // Normalize the result
    let (width, height) = embossed.dimensions();
    let mut normalized = ImageBuffer::new(width, height);

    for (x, y, pixel) in embossed.to_rgb8().enumerate_pixels() {
        let normalized_value = (pixel[0] as f32 / 2.0 + 128.0).clamp(0.0, 255.0) as u8;
        normalized.put_pixel(x, y, Rgb([normalized_value, normalized_value, normalized_value]));
    }

    Ok(DynamicImage::ImageRgb8(normalized))
}

pub fn quantized_edge(img: &DynamicImage, levels: u8, threshold: f32) -> Result<DynamicImage, String> {
    println!("Applying quantized edge filter with {} levels and threshold {}", levels, threshold);

    let edge_img = canny_edge_detection(img, threshold);
    let quantized = quantize_image(&edge_img, levels);

    Ok(DynamicImage::ImageRgb8(quantized))
}
pub fn canny_edge_detection(img: &DynamicImage, threshold: f32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    // Step 1: Noise reduction (Gaussian blur)
    let gaussian_kernel = [
        [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
        [2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0],
        [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
    ];
    let blurred = convolve(img, &gaussian_kernel);

    // Step 2: Gradient calculation
    let sobel_x = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];
    let sobel_y = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];
    let gx = convolve(&blurred, &sobel_x);
    let gy = convolve(&blurred, &sobel_y);

    // Step 3-4: Non-maximum suppression and thresholding
    let (width, height) = img.dimensions();
    let mut edges = ImageBuffer::new(width, height);

    for y in 1..height-1 {
        for x in 1..width-1 {
            let gx_val = gx.get_pixel(x, y)[0] as f32;
            let gy_val = gy.get_pixel(x, y)[0] as f32;
            
            let magnitude = (gx_val * gx_val + gy_val * gy_val).sqrt();
            let angle = gy_val.atan2(gx_val);

            // Non-maximum suppression
            let direction = (((angle * 4.0 / PI) + 4.5).floor() % 4.0) as i32;
            let (dx, dy) = match direction {
                0 => (1i32, 0i32),   // 0 degrees
                1 => (1i32, 1i32),   // 45 degrees
                2 => (0i32, 1i32),   // 90 degrees
                _ => (-1i32, 1i32),  // 135 degrees
            };

            if magnitude > gx.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32)[0] as f32 &&
               magnitude > gx.get_pixel((x as i32 - dx) as u32, (y as i32 - dy) as u32)[0] as f32 &&
               magnitude >= threshold {
                edges.put_pixel(x, y, Luma([255]));
            } else {
                edges.put_pixel(x, y, Luma([0]));
            }
        }
    }

    edges
}


fn quantize_image(img: &ImageBuffer<Luma<u8>, Vec<u8>>, levels: u8) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut quantized = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let value = pixel[0] as f32 / 255.0;
        let quantized_value = (value * levels as f32).round() / levels as f32;
        let rgb_value = (quantized_value * 255.0) as u8;
        quantized.put_pixel(x, y, Rgb([rgb_value, rgb_value, rgb_value]));
    }

    quantized
}

pub fn edge_extrusion(img: &DynamicImage, strength: f32, depth: u32, threshold: f32) -> Result<DynamicImage, String> {
    println!("Applying edge extrusion filter with strength {}, depth {}, and threshold {}", strength, depth, threshold);

    let edges = canny_edge_detection(img, threshold);
    let extruded = extrude_edges(&edges, &img.to_rgba8(), strength, depth);

    Ok(DynamicImage::ImageRgba8(extruded))
}

fn extrude_edges(edges: &ImageBuffer<Luma<u8>, Vec<u8>>, original: &ImageBuffer<Rgba<u8>, Vec<u8>>, strength: f32, depth: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = edges.dimensions();
    let mut extruded = original.clone();

    for d in 1..=depth {
        let factor = 1.0 - (d as f32 / depth as f32);
        for y in 0..height {
            for x in 0..width {
                if edges.get_pixel(x, y)[0] > 0 {
                    let dx = (strength * factor * (x as f32 - width as f32 / 2.0)).round() as i32;
                    let dy = (strength * factor * (y as f32 - height as f32 / 2.0)).round() as i32;
                    let new_x = (x as i32 + dx).clamp(0, width as i32 - 1) as u32;
                    let new_y = (y as i32 + dy).clamp(0, height as i32 - 1) as u32;

                    let original_color = original.get_pixel(x, y);
                    let extruded_color = Rgba([
                        (original_color[0] as f32 * factor) as u8,
                        (original_color[1] as f32 * factor) as u8,
                        (original_color[2] as f32 * factor) as u8,
                        original_color[3],
                    ]);
                    extruded.put_pixel(new_x, new_y, extruded_color);
                }
            }
        }
    }

    extruded
}