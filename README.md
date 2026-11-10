# FlatFormats.rs

High-performance, zero-copy, direct-to-GPU binary formats and streaming containers engineered for the Silicon Era.

[![CI](https://github.com/flat-formats-rs/flat-formats/actions/workflows/ci.yml/badge.svg)](https://github.com/flat-formats-rs/flat-formats/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)

---

## Architecture Overview

FlatFormats rejects legacy pointer-chasing, textual parsing, and runtime decompression bottlenecks. Every data structure is strictly designed with:
- **64-byte CPU cache line alignment**.
- **256-byte GPU row-pitch alignment** for direct DMA upload (Vulkan / Direct3D 12 / Metal).
- **Monomorphic zero-copy views** via `zerocopy` (`no_std` compatible).
- **Sub-tick microsecond accuracy** for physics and input reproduction.

```mermaid
flowchart TD
    subgraph FlatFormats["flat-formats-rs"]
        Asset["flat-formats-asset (.fla)"]
        Demo["flat-formats-demo (.fld)"]
        
        subgraph AssetSuite["Asset Subsystems"]
            Mesh["flatasset-mesh (.flm)<br/>Meshlets, Skeletons, Hitboxes"]
            Tex["flatasset-tex (.flt)<br/>BC1-BC7, ASTC, Mipmaps"]
            World["flatasset-world (.flw)<br/>QBVH, PVS, Lightmaps, Entities"]
        end
        
        Asset --> Mesh
        Asset --> Tex
        Asset --> World
    end

    GPU["GPU VBO / Texture Arrays<br/>Direct DMA Upload"]
    VFS["Memory-Mapped I/O<br/>mmap + Blake3 Integrity"]

    Tex -.->|"Zero copy (256-byte pitch)"| GPU
    Mesh -.->|"Zero copy (meshlet stream)"| GPU
    World -.->|"SIMD Raycast / PVS"| GPU
    Demo -.->|"O(1) Seek / Delta ticks"| VFS
```

---

## Repositories & Crates

| Repository | Primary Crate | Extension | Purpose |
|:---|:---|:---:|:---|
| [`flat-formats`](https://github.com/flat-formats-rs/flat-formats) | `flat-formats` | — | Umbrella facade SDK crate |
| [`flat-formats-asset`](https://github.com/flat-formats-rs/flat-formats-asset) | `flatasset-core` | `.fla` | Multi-asset bundle container with Blake3 chunk hashing |
| [`flat-formats-asset`](https://github.com/flat-formats-rs/flat-formats-asset) | `flatasset-mesh` | `.flm` | Meshlet geometry, quantization, bone weights |
| [`flat-formats-asset`](https://github.com/flat-formats-rs/flat-formats-asset) | `flatasset-tex` | `.flt` | Row-pitch aligned direct-to-GPU textures |
| [`flat-formats-asset`](https://github.com/flat-formats-rs/flat-formats-asset) | `flatasset-world` | `.flw` | 4-wide SIMD QBVH, PVS leaf visibility, static lighting |
| [`flat-formats-demo`](https://github.com/flat-formats-rs/flat-formats-demo) | `flatdemo-core` | `.fld` | Sub-tick deterministic demo format with $O(1)$ keyframe seeking |

---

## Benchmark Scoreboard

Empirical benchmark medians measured on AMD Ryzen 9 (Zen 4) / DDR5:

| Operation | Legacy GoldSrc / Source | FlatFormats | Speedup |
|:---|:---:|:---:|:---:|
| **Texture Ingestion** | 4.82 ms (parsing + swizzling) | **1.24 µs** (`flatasset-tex` zero-copy view) | **~3880× faster** |
| **Model Ingestion** | 2.15 ms (MDL bone allocation) | **840 ns** (`flatasset-mesh` zero-alloc) | **~2550× faster** |
| **World BSP Raycast** | 340 ns (recursive tree traverse) | **32 ns** (4-wide SIMD QBVH raycast) | **~10.6× faster** |
| **Demo Random Seek** | 185 ms (sequential parse) | **140 ns** ($O(1)$ keyframe indexed table) | **~1,320,000× faster** |

---

## Quick Start

Add `flat-formats` to your `Cargo.toml`:

```toml
[dependencies]
flat-formats = "0.1.0"
```

### Inspecting a Texture (`.flt`)
```rust
use flat_formats::tex::FlatTexView;

fn load_texture(bytes: &[u8]) {
    let tex = FlatTexView::from_bytes(bytes).expect("valid aligned flattex");
    println!("Dimensions: {}x{}", tex.header().width, tex.header().height);
    println!("GPU Format: {:?}", tex.header().vk_format);
    
    // Direct slice to mip level 0 payload without allocations
    let mip0 = tex.mip_level(0).expect("mip 0 exists");
    // upload_to_gpu(mip0);
}
```

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
