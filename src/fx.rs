use image::{RgbImage, DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;
use asdf_pixel_sort::{sort_with_options, PColor, Options, Mode, Direction};

pub fn sort_pixel(img: &DynamicImage) -> DynamicImage {
    // Convert the DynamicImage to an RgbImage
    let mut buf: RgbImage = img.to_rgb8();

    // Define the color and options for sorting
    let color = PColor::new(0, 62, 214);
    let options = Options {
        mode: Mode::Black(color),
        //mode: Mode::White(color),
        //mode: Mode::Brightness(50), // 0-255
        direction: Direction::Both // Row, Column, Both
    };

    // Apply the sort_with_options function
    sort_with_options(&mut buf, &options);

    // Convert the buffer back to a DynamicImage and return it
    DynamicImage::ImageRgb8(buf)
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

pub fn glitch(img: &DynamicImage, num_glitches: u32, max_offset: i32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = img.clone();
    let mut rng = rand::thread_rng();

    for _ in 0..num_glitches {
        let y = rng.gen_range(0..height);
        let offset = rng.gen_range(-max_offset..=max_offset);
        let glitch_height = rng.gen_range(1..20);

        for dy in y..y + glitch_height {
            if dy < height {
                for x in 0..width {
                    let source_x = (x as i32 + offset).rem_euclid(width as i32) as u32;
                    let pixel = img.get_pixel(source_x, dy);
                    output.put_pixel(x, dy, pixel);
                }
            }
        }
    }

    output
}