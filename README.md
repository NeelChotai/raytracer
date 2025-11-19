# Raytracer

CPU raytracer using distributed raytracing, loosely following [Ray Tracing in One Weekend](https://raytracing.github.io/)

## Quick Start

```bash
cargo run
```

Renders 800x600 image to `output/render.png`.

## Examples

`example/direct.png` was done with direct lighting (~3s)
`example/tracing.png` was done with proper path tracing (~15s, more samples required)

## TODO

- More materials (glossy, metal, glass, etc.)
- Run on GPU instead of CPU
