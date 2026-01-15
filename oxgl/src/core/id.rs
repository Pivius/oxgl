//! Type-Safe Object Identifiers
//!
//! Provides strongly-typed identifiers for scene objects and lights
//! using slotmap for efficient storage and retrieval.
//!

use slotmap::new_key_type;

new_key_type! {
	/// Identifier for scene objects (meshes).
	pub struct ObjectId;
	/// Identifier for lights in a scene.
	pub struct LightId;
	/// Identifier for 3D css elements;
	pub struct CSS3DElementId;
}