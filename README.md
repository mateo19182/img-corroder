# Img-Corroder

A pipeline based image processing tool written in Rust for creating glitch effects.

## Usage

```sh
cargo run -- -i input-img.jpg -o output.jpg -c examples/all.json
```

## Custom Pipelines

Build your own effect pipeline by following the example provided in `example_fx/pipe.json`.

## TODO

- ASCII filter
- edge stuff
- SAM
  - <https://crates.io/crates/usls>
- depth perception
  - <https://github.com/rozgo/monodepth-rs>

## Image Filters and Parameters

Below is a list of available filters and their parameters:

- **Grayscale**
  - No parameters
  - Converts the image to grayscale

- **Invert**
  - No parameters
  - Inverts the colors of the image

- **Brightness**
  - `factor` (float, default: 1.0): Adjusts the brightness of the image

- **Sepia**
  - No parameters
  - Applies a sepia tone filter to the image

- **Contrast**
  - `factor` (float, default: 1.0): Adjusts the contrast of the image

- **Saturation**
  - `factor` (float, default: 1.0): Adjusts the color saturation of the image

- **Add Noise**
  - `intensity` (float, default: 0.1): Determines the intensity of noise added to the image

- **Deepfry**
  - `factor` (float, default: 1.0): Intensity of the deepfry effect

- **Hue Rotate**
  - `angle` (float, default: 90.0): Angle of hue rotation in degrees

- **Color Replacer**
  - `target_color` (string): The color to be replaced
  - `replacement_color` (string): The color to replace with
  - `tolerance` (integer, default: 50): Color matching tolerance

- **Vaporwave**
  - No parameters
  - Applies a vaporwave aesthetic filter to the image

- **Blur**
  - `sigma` (float, default: 2.0): Blur intensity

- **Pixelate**
  - `block_size` (integer, default: 10): Size of pixelation blocks

- **Oil Painting**
  - `radius` (integer, default: 4): Radius of the effect
  - `intensity` (integer, default: 30): Intensity of the effect

- **Glitch**
  - `amount` (integer, default: 50): Amount of glitch effect
  - `max_offset` (integer, default: 10): Maximum pixel offset
  - `direction` (string): Direction of the glitch effect
  - `noisy` (boolean, default: false): Adds noisy pixels to the glitch

- **Pixel Sort**
  - `low-threshold` (integer, default: 0): Lower threshold for pixel sorting
  - `high-threshold` (integer, default: 0): Upper threshold for pixel sorting
  - `direction` (string): Direction of pixel sorting
  - `window_size` (integer, default: 0): Size of sorting window

- **Rotate**
  - `angle` (float, default: 90): Rotation angle in degrees

- **Desync**
  - `x_shift` (integer, default: 10): Horizontal shift amount
  - `y_shift` (integer, default: 10): Vertical shift amount

- **Wind**
  - `direction` (string, default: "right"): Direction of the wind effect
  - `strength` (integer, default: 10): Strength of the wind effect

- **Scan Lines**
  - `line_thickness` (integer, default: 2): Thickness of scan lines
  - `line_spacing` (integer, default: 10): Spacing between scan lines
  - `opacity` (float, default: 0.5): Opacity of scan lines
  - `angle` (float, default: 0.0): Angle of scan lines

Each filter can be applied to an image, and the parameters allow for fine-tuning of the effect. When using these filters, you can adjust the parameters to achieve the desired visual result. For more detailed information on each filter and its implementation, please refer to the source code.

## References

- [PPG](https://github.com/tmick0/ppg): Applies transforms to actual decoded images, then introduces errors to the transformed data before reversing the transform to obtain an altered image. More information about the philosophy behind PPG can be found [here](https://lo.calho.st/posts/image-glitching/).
- [glitch_png](https://github.com/KernelEquinox/glitch_png)
- [Rust-Wasm-Image-Glitch](https://github.com/felixfaire/Rust-Wasm-Image-Glitch): Cool ones.
