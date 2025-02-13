use image::{DynamicImage, ImageBuffer, Rgba, GenericImageView};
use img_corroder::colorfx;
use img_corroder::edgesfx;
use img_corroder::glitchfx;
use img_corroder::fx_json_generator;

fn create_test_image() -> DynamicImage {
    let width = 100;
    let height = 100;
    let mut img_buf = ImageBuffer::new(width, height);
    // Create a gradient image for testing.
    for y in 0..height {
        for x in 0..width {
            img_buf.put_pixel(x, y, Rgba([x as u8, y as u8, 128, 255]));
        }
    }
    DynamicImage::ImageRgba8(img_buf)
}

#[test]
fn test_brightness() {
    let img = create_test_image();
    let processed = colorfx::brightness(&img, 1.2).expect("Brightness failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_contrast() {
    let img = create_test_image();
    let processed = colorfx::contrast(&img, 1.5);
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_saturation() {
    let img = create_test_image();
    let processed = colorfx::saturation(&img, 1.5).expect("Saturation failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_add_noise() {
    let img = create_test_image();
    let processed = colorfx::add_noise(&img, 0.2).expect("Add noise failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_sepia() {
    let img = create_test_image();
    let processed = colorfx::sepia(&img).expect("Sepia failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_color_replacer() {
    let img = create_test_image();
    // assuming our color map has keys "red" and "blue"
    let processed = colorfx::color_replacer(&img, "red", "blue", 50).expect("Color replacer failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_vaporwave() {
    let img = create_test_image();
    let processed = colorfx::vaporwave(&img).expect("Vaporwave failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_deep_fry() {
    let img = create_test_image();
    let processed = colorfx::deep_fry(&img, 1.0).expect("Deep fry failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_hue_rotate() {
    let img = create_test_image();
    let processed = colorfx::hue_rotate(&img, 45.0).expect("Hue rotate failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_neon_edge() {
    let img = create_test_image();
    let processed = edgesfx::neon_edge(&img, 1.0, 0.0, 1.0).expect("Neon edge failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_sketch() {
    let img = create_test_image();
    let processed = edgesfx::sketch(&img, 10.0, 1.0, false).expect("Sketch failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_emboss() {
    let img = create_test_image();
    let processed = edgesfx::emboss(&img, 100000.0, 45.0).expect("Emboss failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_pixelate() {
    let img = create_test_image();
    let processed = glitchfx::pixelate(&img, 10);
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_oil_painting() {
    let img = create_test_image();
    let processed = glitchfx::oil_painting(&img, 4, 30);
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_glitch() {
    let img = create_test_image();
    let processed = glitchfx::glitch(&img, 10, 5, "vertical", false);
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_pixel_sort() {
    let img = create_test_image();
    let processed = glitchfx::pixel_sort(&img, "horizontal", 150, 200, 100);
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_rotate() {
    let img = create_test_image();
    let processed = glitchfx::rotate(&img, 90.0).expect("Rotate failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_desync() {
    let img = create_test_image();
    let processed = glitchfx::desync(&img, 10, 10).expect("Desync failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_wind() {
    let img = create_test_image();
    let processed = glitchfx::wind(&img, "right", 10).expect("Wind failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_scan_lines() {
    let img = create_test_image();
    let processed = glitchfx::scan_lines(&img, Some(2), Some(10), Some(0.0), Some(0.5)).expect("Scan lines failed");
    assert_eq!(img.dimensions(), processed.dimensions());
}

#[test]
fn test_generate_random_pipeline() {
    // Verify we generate valid JSON
    let json = fx_json_generator::generate_random_pipeline(5);
    assert!(json.get("transformations").is_some());
}

#[test]
fn test_dither() {
    let img = create_test_image();
    let processed = img_corroder::colorfx::dither(&img, 4, None, None, None).expect("Dither filter failed");
    assert_eq!(img.dimensions(), processed.dimensions());
} 