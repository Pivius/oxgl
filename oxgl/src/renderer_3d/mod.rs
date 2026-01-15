//! 3D Rendering System
//!
//! This module provides the complete 3D rendering pipeline including scene management,
//! lighting, shadow mapping, and debug visualization.
//!

pub mod light;
pub mod gizmo;
pub mod primitive;
pub mod scene;
pub mod shadowmap;
pub mod cssrenderer;

pub use scene::{Scene, DebugSettings, SceneObject};
pub use primitive::{Primitive, VertexData};
pub use light::{LightType, Light, apply_lights};
pub use gizmo::GizmoRenderer;
pub use shadowmap::ShadowMap;
pub use cssrenderer::CSS3DRenderer;