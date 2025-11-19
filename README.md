# Raytracer

CPU raytracer using distributed raytracing, loosely following [Ray Tracing in One Weekend](https://raytracing.github.io/)

## Quick Start

```bash
cargo run
RUST_LOG=debug cargo run // with debug logging
```

Renders 800x600 image to `output/render.png`,

## TODO

- More materials (glossy, metal, glass, etc.)
- Path tracing for global illumination
- Run on GPU instead of CPU
