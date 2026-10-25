# light2d-rs

## Introduction

![beerlambert_sample](./docs/beerlambert_sample.png)

light2d-rs is a 2D ray tracer written in Rust. It represents geometry using ***SDF (Signed Distance Fields)*** and performs intersection with ***Ray Marching***. The renderer supports physically based optical phenomena such as reflection, refraction, total internal reflection, Fresnel effects, and Beer–Lambert absorption. Scenes can be described in ***JSON files***, so you can iterate on new looks without recompiling.

## Features

- SDF shape system: A Shape trait with compositional operators (Union / Intersect / Subtract / Complement / Offset) for building complex geometry.
- Physical optics: Specular reflection, refraction, total internal reflection, Fresnel equations, Beer–Lambert attenuation.
- Anti-aliasing: Halton low-discrepancy sequence with random offsets for cleaner images with fewer samples.
- Parallel rendering: Pixel-level parallelism via rayon, saturating multi-core CPUs.
- JSON scene description: Shapes, materials, and ambient light are all file-configurable.
- Friendly 0–255 color input: Specify emissive and absorption colors as intuitive RGB triplets.
- Configurable ambient light: none, constant, or gradient modes.
- Built-in demo scenes: shape / reflect / refraction / beer, ready to go.

## Quick Start

### Requirements

- Rust 1.70+ (latest stable recommended)
- Multi-core CPU recommended (parallelism via rayon)

### Build

```bash
git clone <repo-url>
cd light2d-rs
cargo build --release
```

### Run built-in scenes

```bash
# Beer–Lambert demo (default scene)
cargo run --release -- beer_demo --scene beer --width 800 --height 600 --samples 512

# Reflection demo
cargo run --release -- reflect_demo --scene reflect --width 800 --height 600 --samples 512

# Refraction demo
cargo run --release -- refraction_demo --scene refraction --samples 512

# Shape demo
cargo run --release -- shape_demo --scene shape --samples 512
```

### Run a custom JSON scene

```bash
cargo run --release -- my_scene --scene-file examples/beer_scene.json --width 800 --height 600 --samples 512
```

### Use the compiled binary directly

```bash
./target/release/light2d-rs my_scene --scene-file examples/beer_scene.json --width 1024 --height 768 --samples 1024
```

## CLI Options

```text
Usage: light2d-rs [OPTIONS] <OUTPUT_NAME>

Arguments:
  <OUTPUT_NAME>                  Output filename prefix (no extension)

Options:
      --width <WIDTH>            Image width (default 512)
      --height <HEIGHT>          Image height (default 512)
      --samples <SAMPLES>        Samples per pixel (default 1024)
      --output-dir <DIR>         Output directory (default `output`)
      --scene <SCENE>            Preset scene: shape | reflect | refraction | beer (default beer)
      --scene-file <FILE>        Load scene from JSON file (overrides --scene)
  -h, --help                     Print help
  -V, --version                  Print version
```

Output file: `<output-dir>/N<samples>_<output_name>_<timestamp>.png`

## Scene File Format

The scene is a JSON document. The recommended format (with optional ambient light) is:

```json
{
  "ambient": {
    "type": "gradient",
    "top": [0.12, 0.15, 0.25],
    "bottom": [0.02, 0.02, 0.04]
  },
  "shape": {
    "type": "Union",
    "children": [
      {
        "type": "Circle",
        "center": [0.5, 0.5],
        "radius": 0.2,
        "material": { "reflectivity": 0.8 }
      },
      {
        "type": "Box",
        "center": [0.7, 0.3],
        "theta": 0.3927,
        "half_size": [0.1, 0.2],
        "material": {
          "eta": 1.5,
          "absorption_color": [200, 100, 220]
        }
      }
    ]
  }
}
```

Backward compatible: The legacy format (top-level shape node) is also accepted; ambient light defaults to `none`.

### Supported primitives

| Type       | Parameters                     | Description                |
| :--------- | :----------------------------- | :------------------------- |
| `Circle`   | `center`, `radius`             | Circle                     |
| `Plane`    | `point`, `normal`              | Half-plane                 |
| `Ngon`     | `center`, `radius, `n`         | Regular N-gon              |
| `Capsule`  | `a`, `b`, `radius`             | Capsule (segment + radius) |
| `Box`      | `center`, `theta`, `half_size` | Rotated rectangle          |
| `Triangle` | `a`, `b`, `c`                  | Triangle                   |


### Supported operators

| Type         | Parameters          | Description                                    |
| :----------- | :------------------ | :--------------------------------------------- |
| `Union`      | `children: [...]`   | Union of children                              |
| `Intersect`  | `children: [...]`   | Intersection of children                       |
| `Subtract`   | `a`, `b`            | A minus B                                      |
| `Complement` | `shape`             | SDF negation                                   |
| `Offset`     | `shape`, `distance` | SDF offset (positive dilates, negative erodes) |

### Material parameters

| Field              | Type       | Default   | Description                                        |
| :----------------- | :--------- | :-------- | :------------------------------------------------- |
| `reflectivity`     | `f64`      | `0.0`     | Reflectivity (0–1)                                 |
| `eta`              | `f64`      | `0.0`     | Refractive index (>0 enables refraction, e.g. 1.5) |
| `emissive`         | `[f64; 3]` | `[0,0,0]` | Direct emissive radiance (HDR)                     |
| `emissive_color`   | `[u8; 3]`  | —         | 0–255 emissive color, auto-converted               |
| `absorption`       | `[f64; 3]` | `[0,0,0]` | Absorption coefficient (Beer–Lambert)              |
| `absorption_color` | `[u8; 3]`  | —         | 0–255 target transmittance, auto-converted         |

> Note: When both are present, `emissive` takes priority over `emissive_color`; likewise for `absorption`.

### Ambient light types

| Type       | Parameters                          | Description                            |
| :--------- | :---------------------------------- | :------------------------------------- |
| `none`     | —                                   | Pure black background (default)        |
| `constant` | `color: [f64; 3]`                   | Uniform color                          |
| `gradient` | `top: [f64; 3]`, `bottom: [f64; 3]` | Vertical gradient simulating sky light |

## Project Layout

```text
light2d-rs/
├── Cargo.toml
├── README.md
├── examples/
│   └── *.json                # Example scenes
└── src/
    ├── main.rs               # Binary entry point
    ├── lib.rs                # Library entry, re-exports public API
    ├── color.rs              # Color type
    ├── vec2.rs               # 2D vector
    ├── ambient.rs            # Ambient light
    ├── config.rs             # CLI parsing & config
    ├── render.rs             # Rendering core (trace / sample / gradient)
    ├── sdf/
    │   ├── mod.rs            # SDF module exports
    │   ├── core.rs           # Shape trait & SdfResult
    │   ├── primitives.rs     # Low-level SDF functions (no material)
    │   ├── parser.rs         # JSON scene parser
    │   ├── shapes/           # Concrete shapes
    │   │   ├── circle.rs
    │   │   ├── plane.rs
    │   │   ├── ngon.rs
    │   │   ├── capsule.rs
    │   │   ├── box_shape.rs
    │   │   └── triangle.rs
    │   └── ops/              # Composition operators
    │       ├── union.rs
    │       ├── intersect.rs
    │       ├── subtract.rs
    │       ├── complement.rs
    │       └── offset.rs
    └── scene/                # Preset scene builders
        ├── mod.rs
        ├── shape_scene.rs
        ├── reflect_scene.rs
        ├── refraction_scene.rs
        └── beer_scene.rs
```

## Architecture Overview

- `Shape` trait: Every primitive and composite operator implements it; scenes are `Box<dyn Shape>` trees.
- `SdfResult`: Each SDF evaluation returns distance plus material parameters; composite ops inherit the nearest child's material by default.
- **Pipeline**:
    1. Emit N rays per pixel (Halton-sampled directions).
    2. Ray-march until `sd < EPSILON` (hit) or `MAX_DISTANCE` (escape).
    3. On hit, split energy into reflect/refract via Fresnel; recursively trace with `MAX_DEPTH` limit.
    4. On escape, return ambient light sample.
    5. Average N samples, clamp to [0,1], write PNG.
- **Pixel parallelism**: `rayon` processes pixels in parallel.

## Roadmap

- [x] SDF shapes and composition operators
- [x] Reflection / refraction / Fresnel / Beer–Lambert
- [x] JSON scene description
- [x] Friendly 0–255 color input
- [ ] Configurable ambient light (already designed)
- [ ] BRDF-based rough reflection (roughness material)
- [ ] Soft shadows (Next Event Estimation)
- [ ] Caustics (BDPT or photon mapping)


## License

Released under the `MIT License`. See `LICENSE` for details.

[中文](./docs/README-zh.md)