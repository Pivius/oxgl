//! Common Rendering Types
//!
//! This module provides shared types used across the rendering system.
//! 

pub mod camera;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod loader;
pub mod postprocessing;

pub use camera::Camera;
pub use loader::MeshData;
pub use material::{Uniform, Material, MaterialBuilder, presets};
pub use mesh::Mesh;
pub use shader::{compile_shader, link_program};
pub use postprocessing::{PostProcessStack, PostProcessEffect, PostProcessEffectBuilder};
