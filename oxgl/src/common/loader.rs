//! Mesh Data Loading and Parsing
//!
//! Provides utilities for loading 3D mesh data from various file formats.
//! Currently supports OBJ file parsing with automatic normal computation.
//!
//! ## Examples
//!
//! ```
//! use oxgl::common::MeshData;
//!
//! let obj_content = include_str!("model.obj");
//! let meshes = MeshData::from_obj(obj_content)?;
//!
//! for mesh in meshes {
//!		let interleaved = mesh.interleaved_vertices();
//!		// Use with Mesh::with_normals...
//! }
//! ```
//!

use glam::Vec3;

/// Raw mesh data containing vertex attributes.
///
/// Stores position, normal, and UV data in separate flat arrays.
/// Can be loaded from OBJ files or constructed manually.
///
/// ## Construction
///
/// - [`MeshData::from_obj`] - Parse from OBJ file content
/// - [`MeshData::default`] - Create empty mesh data
///
#[derive(Clone, Debug, Default)]
pub struct MeshData {
	pub positions: Vec<f32>,
	pub normals: Vec<f32>,
	pub uvs: Vec<f32>,
}

impl MeshData {
	/// Parses mesh data from OBJ file content.
	///
	/// If the OBJ file doesn't contain normals, they are computed automatically
	/// using face normals.
	///
	/// # Errors
	///
	/// Returns an error if the OBJ content is malformed. Currently this is
	/// lenient and will skip malformed lines rather than failing.
	///
	/// # Examples
	///
	/// Loading from an embedded file:
	///
	/// ```
	/// use oxgl::common::MeshData;
	///
	/// let obj_content = include_str!("assets/cube.obj");
	/// let meshes = MeshData::from_obj(obj_content)?;
	/// ```
	///
	pub fn from_obj(content: &str) -> Result<Vec<MeshData>, String> {
		let mut positions: Vec<Vec3> = Vec::new();
		let mut normals: Vec<Vec3> = Vec::new();
		let mut uvs: Vec<[f32; 2]> = Vec::new();

		let mut out_positions: Vec<f32> = Vec::new();
		let mut out_normals: Vec<f32> = Vec::new();
		let mut out_uvs: Vec<f32> = Vec::new();

		for line in content.lines() {
			let line = line.trim();

			if line.is_empty() || line.starts_with('#') {
				continue;
			}

			let parts: Vec<&str> = line.split_whitespace().collect();

			if parts.is_empty() {
				continue;
			}

			match parts[0] {
				"v" => {
					if parts.len() >= 4 {
						let x: f32 = parts[1].parse().unwrap_or(0.0);
						let y: f32 = parts[2].parse().unwrap_or(0.0);
						let z: f32 = parts[3].parse().unwrap_or(0.0);

						positions.push(Vec3::new(x, y, z));
					}
				}
				"vn" => {
					if parts.len() >= 4 {
						let x: f32 = parts[1].parse().unwrap_or(0.0);
						let y: f32 = parts[2].parse().unwrap_or(0.0);
						let z: f32 = parts[3].parse().unwrap_or(0.0);

						normals.push(Vec3::new(x, y, z));
					}
				}
				"vt" => {
					if parts.len() >= 3 {
						let u: f32 = parts[1].parse().unwrap_or(0.0);
						let v: f32 = parts[2].parse().unwrap_or(0.0);

						uvs.push([u, v]);
					}
				}
				"f" => {
					let face_verts: Vec<_> = parts[1..].iter().map(|p| parse_face_vertex(p)).collect();

					for i in 1..face_verts.len() - 1 {
						for &idx in &[0, i, i + 1] {
							let (vi, ti, ni) = face_verts[idx];

							if let Some(pos) = positions.get(vi) {
								out_positions.extend_from_slice(&[pos.x, pos.y, pos.z]);
							}

							if let Some(norm) = ni.and_then(|i| normals.get(i)) {
								out_normals.extend_from_slice(&[norm.x, norm.y, norm.z]);
							} else {
								out_normals.extend_from_slice(&[0.0, 1.0, 0.0]);
							}

							if let Some(uv) = ti.and_then(|i| uvs.get(i)) {
								out_uvs.extend_from_slice(uv);
							}
						}
					}
				}
				_ => {}
			}
		}

		if out_normals.iter().all(|&n| n == 0.0 || n == 1.0) {
			out_normals = compute_normals(&out_positions);
		}

		Ok(vec![MeshData {
			positions: out_positions,
			normals: out_normals,
			uvs: out_uvs,
		}])
	}

	/// Converts the mesh data to interleaved vertex format.
	///
	/// Produces a flat array with interleaved position and normal data:
	/// `[px, py, pz, nx, ny, nz, px, py, pz, nx, ny, nz, ...]`
	///
	/// This format is suitable for use with [`Mesh::with_normals`](crate::common::Mesh::with_normals).
	///
	/// # Returns
	///
	/// A flat `Vec<f32>` with 6 floats per vertex (3 position + 3 normal).
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::common::{MeshData, Mesh};
	/// use oxgl::renderer_3d::VertexData;
	///
	/// let mesh_data = MeshData::from_obj(obj_content)?[0].clone();
	/// let interleaved = mesh_data.interleaved_vertices();
	///
	/// let vertex_data = VertexData {
	///		data: interleaved,
	///		vertex_count: (mesh_data.positions.len() / 3) as i32,
	/// };
	///
	/// let mesh = Mesh::with_normals(&gl, &vertex_data, material);
	/// ```
	pub fn interleaved_vertices(&self) -> Vec<f32> {
		let vertex_count = self.positions.len() / 3;
		let mut result = Vec::with_capacity(vertex_count * 6);

		for i in 0..vertex_count {
			result.push(self.positions[i * 3]);
			result.push(self.positions[i * 3 + 1]);
			result.push(self.positions[i * 3 + 2]);

			if self.normals.len() > i * 3 + 2 {
				result.push(self.normals[i * 3]);
				result.push(self.normals[i * 3 + 1]);
				result.push(self.normals[i * 3 + 2]);
			} else {
				result.push(0.0);
				result.push(1.0);
				result.push(0.0);
			}
		}

		result
	}
}

/// Parses a single face vertex definition from OBJ format.
///
/// Returns tuple of (position_index, texture_index, normal_index).
/// Indices are converted from 1-based (OBJ) to 0-based.
fn parse_face_vertex(s: &str) -> (usize, Option<usize>, Option<usize>) {
	let parts: Vec<&str> = s.split('/').collect();

	let v = parts.get(0)
		.and_then(|p| p.parse::<usize>().ok())
		.map(|i| i - 1)
		.unwrap_or(0);

	let t = parts.get(1)
		.filter(|p| !p.is_empty())
		.and_then(|p| p.parse::<usize>().ok())
		.map(|i| i - 1);

	let n = parts.get(2)
		.and_then(|p| p.parse::<usize>().ok())
		.map(|i| i - 1);

	(v, t, n)
}

/// Computes flat-shaded normals from triangle positions.
///
/// For each triangle, computes the face normal using the cross product
/// of two edges. The same normal is assigned to all three vertices.
///
/// # Returns
///
/// A flat array of normals with the same length as positions.
fn compute_normals(positions: &[f32]) -> Vec<f32> {
	let mut normals = Vec::with_capacity(positions.len());

	for tri in positions.chunks(9) {
		if tri.len() < 9 {
			normals.extend_from_slice(&[0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]);
			continue;
		}

		let v0 = Vec3::new(tri[0], tri[1], tri[2]);
		let v1 = Vec3::new(tri[3], tri[4], tri[5]);
		let v2 = Vec3::new(tri[6], tri[7], tri[8]);
		
		let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();

		for _ in 0..3 {
			normals.extend_from_slice(&[normal.x, normal.y, normal.z]);
		}
	}

	normals
}