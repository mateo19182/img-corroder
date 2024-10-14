# Img-Corroder

A pipeline based image processing tool written in Rust for creating glitch effects.

## Usage

```sh
cargo run -- -i img.jpg -o output.jpg -c example_fx/pipe.json
```

## Included Effects

- **grayscale**: No parameters.
- **invert**: No parameters.
- **blur**:
  - `sigma` (f64): Standard deviation of the Gaussian blur. Default is 2.0.
- **pixelate**:
  - `block_size` (u32): Size of the pixel blocks. Default is 10.
- **oil_painting**:
  - `radius` (u32): Radius of the effect. Default is 4.
  - `intensity` (u32): Intensity of the effect. Default is 30.
- **glitch**:
  - `amount` (u32): Amount of glitch effect. Default is 50.
  - `seed` (u32): Seed for random number generator. Default is 10.
- **sort**: No parameters.

## Custom Pipelines

Build your own effect pipeline by following the example provided in `example_fx/pipe.json`.

## References

- [PPG](https://github.com/tmick0/ppg): Applies transforms to actual decoded images, then introduces errors to the transformed data before reversing the transform to obtain an altered image. More information about the philosophy behind PPG can be found [here](https://lo.calho.st/posts/image-glitching/).
- [glitch_png](https://github.com/KernelEquinox/glitch_png)
- [glitch](https://github.com/strangeglyph/glitch): Classic desync.
- [Rust-Wasm-Image-Glitch](https://github.com/felixfaire/Rust-Wasm-Image-Glitch): Cool ones.
