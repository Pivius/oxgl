//! Mesh Management
//!
//! Provides mesh creation and rendering functionality for 3D objects.
//! Meshes combine vertex data with materials and handle GPU buffer management.
//!
//! ## Examples
//!
//! ```ignore
//! use oxgl::common::{Mesh, material::presets};
//! use oxgl::renderer_3d::Primitive;
//! use glam::Vec3;
//!
//! // Create a cube mesh with a phong material
//! let mesh = Mesh::with_normals(
//!		&gl,
//!		&Primitive::Cube.vertices_with_normals(),
//!		presets::phong(&gl, Vec3::new(0.8, 0.2, 0.2))
//! );
//!
//! // Load from OBJ file
//! let meshes = Mesh::from_obj(&gl, obj_content, material).unwrap();
//! ```
//!

use web_sys::{WebGlBuffer, WebGlProgram, WebGl2RenderingContext as GL};

use super::{Camera, Material, MeshData};
use crate::{
	renderer_3d::{VertexData, Light},
	core::{Transform3D, Transformable}
};

/// A renderable 3D mesh with associated material.
///
/// Manages vertex buffer data on the GPU and provides methods for rendering
/// with lighting and camera transforms. Supports meshes with or without normals.
///
/// ## Construction
///
/// - [`Mesh::new`] - Basic mesh with position-only vertices
/// - [`Mesh::with_normals`] - Mesh with interleaved position and normal data
/// - [`Mesh::from_data`] - From [`MeshData`] struct
/// - [`Mesh::from_obj`] - Parse from OBJ file content
///
/// ## Rendering
///
/// - [`Mesh::draw`] - Full render with material, lighting, and transforms
/// - [`Mesh::draw_depth_only`] - Depth-only render for shadow passes
///
pub struct Mesh {
	vertex_buffer: WebGlBuffer,
	vertex_count: i32,
	stride: i32,
	has_normals: bool,
	pub material: Material,
}

impl Mesh {
	/// Creates a new mesh with position-only vertex data.
	///
	/// Use this for simple meshes that don't require lighting calculations.
	/// For lit meshes, use [`Mesh::with_normals`] instead.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::common::Mesh;
	///
	/// let vertices = [
	///		0.0, 0.0, 0.0, // vertex 1
	///		1.0, 0.0, 0.0, // vertex 2
	///		0.5, 1.0, 0.0, // vertex 3
	/// ];
	///
	/// let mesh = Mesh::new(&gl, &vertices, material);
	/// ```
	pub fn new(gl: &GL, vertices: &[f32], material: Material) -> Self {
		let vertex_buffer = gl.create_buffer().expect("Failed to create buffer");

		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));

		let vert_array = unsafe {
			std::slice::from_raw_parts(
				vertices.as_ptr() as *const u8,
				vertices.len() * std::mem::size_of::<f32>(),
			)
		};

		gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, vert_array, GL::STATIC_DRAW);

		Self {
			vertex_buffer,
			vertex_count: (vertices.len() / 3) as i32,
			stride: 3 * 4,
			has_normals: false,
			material,
		}
	}

	/// Creates a mesh from [`MeshData`].
	///
	/// Converts the mesh data to interleaved vertex format with normals.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::common::{Mesh, MeshData};
	///
	/// let data = MeshData {
	///		positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0],
	///		normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
	///		indices: vec![0, 1, 2],
	/// };
	///
	/// let mesh = Mesh::from_data(&gl, &data, material);
	/// ```
	pub fn from_data(gl: &GL, data: &MeshData, material: Material) -> Self {
		let vertices = data.interleaved_vertices();
		let vertex_data = VertexData {
			data: vertices,
			vertex_count: data.positions.len() as i32 / 3,
		};

		Self::with_normals(gl, &vertex_data, material)
	}

	/// Creates meshes from OBJ file content.
	///
	/// Parses the OBJ content and creates a mesh for each object/group found.
	/// All meshes share the same material.
	///
	/// # Errors
	///
	/// Returns an error string if the OBJ content is malformed or cannot be parsed.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::common::Mesh;
	///
	/// let obj_content = include_str!("model.obj");
	/// let meshes = Mesh::from_obj(&gl, obj_content, material)?;
	///
	/// for mesh in meshes {
	///		// Add each mesh to the scene
	/// }
	/// ```
	pub fn from_obj(gl: &GL, obj_content: &str, material: Material) -> Result<Vec<Self>, String> {
		let mesh_data = MeshData::from_obj(obj_content)?;

		Ok(mesh_data
			.iter()
			.map(|data| Self::from_data(gl, data, material.clone()))
			.collect())
	}

	/// Creates a mesh with interleaved position and normal data.
	///
	/// This is the preferred constructor for meshes that will be rendered
	/// with lighting.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::common::Mesh;
	/// use oxgl::renderer_3d::Primitive;
	///
	/// // Using a primitive helper
	/// let cube_data = Primitive::Cube.vertices_with_normals();
	/// let mesh = Mesh::with_normals(&gl, &cube_data, material);
	/// ```
	pub fn with_normals(gl: &GL, data: &VertexData, material: Material) -> Self {
		let vertex_buffer = gl.create_buffer().expect("Failed to create buffer");

		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));

		let vert_array = unsafe {
			std::slice::from_raw_parts(
				data.data.as_ptr() as *const u8,
				data.data.len() * std::mem::size_of::<f32>(),
			)
		};

		gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, vert_array, GL::STATIC_DRAW);

		Self {
			vertex_buffer,
			vertex_count: data.vertex_count,
			stride: 6 * 4,
			has_normals: true,
			material,
		}
	}

	/// Renders the mesh for depth-only passes.
	///
	/// Used for shadow map generation where only depth information is needed.
	/// Does not apply material uniforms or lighting calculations.
	///
	/// # Examples
	///
	/// ```
	/// // During shadow pass
	/// gl.use_program(Some(&shadow_program));
	/// 
	/// // Set light space matrix uniform...
	/// 
	/// mesh.draw_depth_only(&gl, &shadow_program);
	/// ```
	pub fn draw_depth_only(&self, gl: &GL, program: &WebGlProgram) {
		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));

		let pos_loc = gl.get_attrib_location(program, "position");

		if pos_loc >= 0 {
			gl.enable_vertex_attrib_array(pos_loc as u32);
			gl.vertex_attrib_pointer_with_i32(
				pos_loc as u32, 3, GL::FLOAT, false, self.stride, 0
			);
		}

		gl.draw_arrays(GL::TRIANGLES, 0, self.vertex_count);
	}

	/// Renders the mesh with full material and lighting.
	///
	/// Applies the mesh's material, sets up model/view/projection matrices,
	/// binds vertex attributes, and issues the draw call.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::core::Transform3D;
	/// use glam::Vec3;
	///
	/// let transform = Transform3D::new()
	///     .with_position(Vec3::new(0.0, 1.0, 0.0));
	///
	/// mesh.draw(&gl, &transform, &camera, &lights);
	/// ```
	pub fn draw(&self, gl: &GL, transform: &Transform3D, camera: &Camera, lights: &[Light]) {
		let program = self.material.program();

		gl.use_program(Some(program));
		self.material.apply(gl, lights);

		if let Some(loc) = gl.get_uniform_location(program, "model") {
			gl.uniform_matrix4fv_with_f32_array(
				Some(&loc), false, &transform.to_matrix().to_cols_array()
			);
		}
		if let Some(loc) = gl.get_uniform_location(program, "view") {
			gl.uniform_matrix4fv_with_f32_array(
				Some(&loc), false, &camera.view_matrix().to_cols_array()
			);
		}
		if let Some(loc) = gl.get_uniform_location(program, "projection") {
			gl.uniform_matrix4fv_with_f32_array(
				Some(&loc), false, &camera.projection_matrix().to_cols_array()
			);
		}
		if let Some(loc) = gl.get_uniform_location(program, "cameraPosition") {
			gl.uniform3fv_with_f32_array(
				Some(&loc), &camera.position.to_array()
			);
		}

		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));

		let pos_loc = gl.get_attrib_location(program, "position");

		if pos_loc >= 0 {
			gl.enable_vertex_attrib_array(pos_loc as u32);
			gl.vertex_attrib_pointer_with_i32(
				pos_loc as u32, 3, GL::FLOAT, false, self.stride, 0
			);
		}

		if self.has_normals {
			let norm_loc = gl.get_attrib_location(program, "normal");

			if norm_loc >= 0 {
				gl.enable_vertex_attrib_array(norm_loc as u32);
				gl.vertex_attrib_pointer_with_i32(
					norm_loc as u32, 3, GL::FLOAT, false, self.stride, 12
				);
			}
		}

		gl.draw_arrays(GL::TRIANGLES, 0, self.vertex_count);
	}
}