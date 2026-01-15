//! Primitive Shape Generation
//!
//! Provides built-in geometric primitives with vertex data ready for rendering.
//! Primitives can be generated with or without normal data for use with
//! different material types.
//!
//! ## Examples
//!
//! ```ignore
//! use oxgl::renderer_3d::Primitive;
//! use oxgl::common::{Mesh, material::presets};
//! use glam::Vec3;
//!
//! // Create a lit cube
//! let cube_data = Primitive::Cube.vertices_with_normals();
//! let cube = Mesh::with_normals(&gl, &cube_data, presets::phong(&gl, Vec3::ONE));
//!
//! // Create an unlit quad
//! let quad_vertices = Primitive::Quad.vertices();
//! let quad = Mesh::new(&gl, &quad_vertices, presets::unlit(&gl, Vec3::ONE));
//! ```
//!

/// Built-in geometric primitive shapes.
pub enum Primitive {
	Quad,
	Triangle,
	Cube,
}

/// Interleaved vertex data with position and normal attributes.
///
/// Used with [`Mesh::with_normals`](crate::common::Mesh::with_normals) to create
/// meshes that support lighting calculations.
///
/// ## Data Layout
///
/// Each vertex consists of 6 floats:
/// ```text
/// [px, py, pz, nx, ny, nz, px, py, pz, nx, ny, nz, ...]
/// ```
///
pub struct VertexData {
	pub data: Vec<f32>,
	pub vertex_count: i32,
}

impl Primitive {
	/// Returns position-only vertex data.
	///
	/// Use this for unlit materials or when normals are not needed.
	/// The returned array contains only position data (3 floats per vertex).
	///
	pub fn vertices(&self) -> Vec<f32> {
		match self {
			Primitive::Quad => vec![
				-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0,
				-0.5, 0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0,
			],
			Primitive::Triangle => vec![
				0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0,
			],
			Primitive::Cube => vec![
				// Front face
				-0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5,
				-0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5,
				// Back face
				-0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
				-0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5,
				// Left face
				-0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5,
				-0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
				// Right face
				0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5,
				0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5,
				// Top face
				-0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
				-0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5,
				// Bottom face
				-0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5,
				-0.5, -0.5, -0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5,
			],
		}
	}

	
	/// Returns vertex data with interleaved positions and normals.
	///
	/// Use this for lit materials that require normal vectors for lighting
	/// calculations. Each vertex includes both position and normal data.
	///
	/// Accessing the raw data:
	///
	/// ```
	/// use oxgl::renderer_3d::Primitive;
	///
	/// let data = Primitive::Cube.vertices_with_normals();
	/// println!("Cube has {} vertices", data.vertex_count);
	/// println!("Data size: {} floats", data.data.len());
	/// ```
	pub fn vertices_with_normals(&self) -> VertexData {
		match self {
			Primitive::Cube => {
				// Each vertex: position (3) + normal (3)
				let data = vec![
					// Front face (normal: 0, 0, 1)
					-0.5, -0.5, 0.5, 0.0, 0.0, 1.0,
					0.5, -0.5, 0.5, 0.0, 0.0, 1.0,
					0.5, 0.5, 0.5, 0.0, 0.0, 1.0,
					-0.5, -0.5, 0.5, 0.0, 0.0, 1.0,
					0.5, 0.5, 0.5, 0.0, 0.0, 1.0,
					-0.5, 0.5, 0.5, 0.0, 0.0, 1.0,
					
					// Back face (normal: 0, 0, -1)
					-0.5, -0.5, -0.5, 0.0, 0.0, -1.0,
					-0.5, 0.5, -0.5, 0.0, 0.0, -1.0,
					0.5, 0.5, -0.5, 0.0, 0.0, -1.0,
					-0.5, -0.5, -0.5, 0.0, 0.0, -1.0,
					0.5, 0.5, -0.5, 0.0, 0.0, -1.0,
					0.5, -0.5, -0.5, 0.0, 0.0, -1.0,
					
					// Left face (normal: -1, 0, 0)
					-0.5, -0.5, -0.5, -1.0, 0.0, 0.0,
					-0.5, -0.5, 0.5, -1.0, 0.0, 0.0,
					-0.5, 0.5, 0.5, -1.0, 0.0, 0.0,
					-0.5, -0.5, -0.5, -1.0, 0.0, 0.0,
					-0.5, 0.5, 0.5, -1.0, 0.0, 0.0,
					-0.5, 0.5, -0.5, -1.0, 0.0, 0.0,
					
					// Right face (normal: 1, 0, 0)
					0.5, -0.5, -0.5, 1.0, 0.0, 0.0,
					0.5, 0.5, -0.5, 1.0, 0.0, 0.0,
					0.5, 0.5, 0.5, 1.0, 0.0, 0.0,
					0.5, -0.5, -0.5, 1.0, 0.0, 0.0,
					0.5, 0.5, 0.5, 1.0, 0.0, 0.0,
					0.5, -0.5, 0.5, 1.0, 0.0, 0.0,
					
					// Top face (normal: 0, 1, 0)
					-0.5, 0.5, -0.5, 0.0, 1.0, 0.0,
					-0.5, 0.5, 0.5, 0.0, 1.0, 0.0,
					0.5, 0.5, 0.5, 0.0, 1.0, 0.0,
					-0.5, 0.5, -0.5, 0.0, 1.0, 0.0,
					0.5, 0.5, 0.5, 0.0, 1.0, 0.0,
					0.5, 0.5, -0.5, 0.0, 1.0, 0.0,
					
					// Bottom face (normal: 0, -1, 0)
					-0.5, -0.5, -0.5, 0.0, -1.0, 0.0,
					0.5, -0.5, -0.5, 0.0, -1.0, 0.0,
					0.5, -0.5, 0.5, 0.0, -1.0, 0.0,
					-0.5, -0.5, -0.5, 0.0, -1.0, 0.0,
					0.5, -0.5, 0.5, 0.0, -1.0, 0.0,
					-0.5, -0.5, 0.5, 0.0, -1.0, 0.0,
				];
				VertexData { data, vertex_count: 36 }
			}
			Primitive::Quad => {
				let data = vec![
					-0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
					-0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
					0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
					-0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
					0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
					0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
				];
				VertexData { data, vertex_count: 6 }
			}
			Primitive::Triangle => {
				let data = vec![
					0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
					-0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
					0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
				];
				VertexData { data, vertex_count: 3 }
			}
		}
	}
}