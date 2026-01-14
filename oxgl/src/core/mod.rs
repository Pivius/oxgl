//! Core Types and Utilities
//!
//! This module provides fundamental types used throughout the library.
//!

pub mod transform;
pub mod color;
pub mod id;
pub mod animator;

pub use transform::{Transform3D, Transformable};
pub use id::{ObjectId, LightId};
pub use color::Color;
pub use animator::Animator;