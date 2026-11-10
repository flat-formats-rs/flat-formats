//! # flat-formats
//!
//! Master SDK facade for the **FlatFormats** high-performance zero-copy game format ecosystem:
//!
//! - [`asset`]: Unified multi-asset container (`.fla`), parallel chunk streaming, Blake3 verification.
//! - [`mesh`]: Direct-to-VBO meshlet geometry, quantized vertex streams, and skeletal animations (`.flm`).
//! - [`tex`]: Direct-to-GPU 256-byte row-pitch aligned texture container (`.flt`).
//! - [`world`]: Zero-copy level geometry, SIMD QBVH spatial acceleration, PVS visibility, and lightmaps (`.flw`).
//! - [`demo`]: Microsecond-exact subtick demo recordings with O(1) random access seeking (`.fld`).

#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub use flatasset_core as asset;
pub use flatasset_mesh as mesh;
pub use flatasset_tex as tex;
pub use flatasset_world as world;
pub use flatdemo_core as demo;
pub use silicon_kit as silicon;
