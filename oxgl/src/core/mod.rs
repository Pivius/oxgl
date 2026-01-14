//! Core Types and Utilities
//!
//! This module provides fundamental types used throughout the library including
//! transforms, object identifiers, animation utilities, and color helpers.
//!
//! ## Types
//!
//! - [`Transform3D`]: Position, rotation, and scale for 3D objects
//! - [`ObjectId`] / [`LightId`]: Type-safe identifiers for scene objects
//! - [`Color`]: Color conversion utilities
//! - [`Animator`]: Browser animation frame loop wrapper
//!

pub mod transform;
pub mod color;
pub mod id;
pub mod animator;

pub use transform::{Transform3D, Transformable};
pub use id::{ObjectId, LightId};
pub use color::Color;
pub use animator::Animator;