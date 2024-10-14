# Img-Corroder

A pipeline based image processing tool written in Rust for creating glitch effects.

## Usage

```sh
cargo run -- -i img.jpg -o output.jpg -c examples/all.json
```

## Custom Pipelines

Build your own effect pipeline by following the example provided in `example_fx/pipe.json`.

## TODO

- ASCII filter
- actually implement pixel sort
- edge stuff

## Filters and Parameters

- **grayscale**

- **invert**

- **blur**
  - `sigma` (f64): Standard deviation of the Gaussian blur. Default is 2.0.

- **pixelate**
  - `block_size` (u64): Size of the pixelation blocks. Default is 10.

- **oil_painting**
  - `radius` (u64): Radius of the effect. Default is 4.
  - `intensity` (u64): Intensity of the effect. Default is 30.

- **glitch**
  - `amount` (u64): Amount of glitch effect. Default is 50.
  - `max_offset` (u64): Maximum offset for glitch displacement. Default is 10.
  - `direction` (String): Direction of the glitch effect. Options are "vertical", "horizontal".
  - `noisy` (bool): Whether to add noisy pixels. Default is false.

- **pixel_sort**
  - `mode` (String): Sorting mode.
  - `direction` (String): Sorting direction. Options are "row", "column", "both"
  - `low_threshold` (u64): Threshold for sorting. Default is 0.
  - `high_threshold` (u64): Threshold for sorting. Default is 0.
  - if low_threshold => high_threshold, no sorting will be made

- **rotate**
  - `angle` (u64): Rotation angle in degrees. Default is 90.

- **desync**
  - `x_shift` (i64): Horizontal shift. Default is 10.
  - `y_shift` (i64): Vertical shift. Default is 10.

- **wind**
  - `direction` (String): Direction of the wind effect. Options are "right", "left", "up", "down"
  - `strength` (u64): Strength of the wind effect. Default is 10.

- **scan-lines**
  - `line_thickness` (u64): Thickness of scan lines. Default is 2.
  - `line_spacing` (u64): Spacing between scan lines. Default is 10.
  - `opacity` (f64): Opacity of scan lines. Default is 0.5.
  - `angle` (f64): Angle of scan lines. Default is 0.0.

For more detailed information on each filter and its implementation, please refer to the source code.

## References

- [PPG](https://github.com/tmick0/ppg): Applies transforms to actual decoded images, then introduces errors to the transformed data before reversing the transform to obtain an altered image. More information about the philosophy behind PPG can be found [here](https://lo.calho.st/posts/image-glitching/).
- [glitch_png](https://github.com/KernelEquinox/glitch_png)
- [Rust-Wasm-Image-Glitch](https://github.com/felixfaire/Rust-Wasm-Image-Glitch): Cool ones.
