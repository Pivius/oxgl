//! Debug Gizmo Rendering
//!
//! Provides immediate-mode debug visualization primitives for 3D scenes.
//!
//! ## Examples
//!
//! ```
//! use oxgl::renderer_3d::GizmoRenderer;
//! use glam::Vec3;
//!
//! let gizmos = GizmoRenderer::new(&gl);
//!
//! // Draw coordinate axes at origin
//! gizmos.axes(&gl, &camera, Vec3::ZERO, 1.0);
//!
//! // Draw a ground grid
//! gizmos.grid(&gl, &camera, 10.0, 10, Vec3::new(0.3, 0.3, 0.3));
//!
//! // Visualize a bounding sphere
//! gizmos.wire_sphere(&gl, &camera, object_pos, radius, Vec3::new(1.0, 1.0, 0.0));
//! ```
//!

use glam::{Vec3, Mat4};
use web_sys::{WebGlBuffer, WebGlProgram, WebGl2RenderingContext as GL};
use std::cell::RefCell;

use crate::common::{compile_shader, link_program, Camera};

const GIZMO_VERT: &str = r#"
	attribute vec3 position;
	uniform mat4 view;
	uniform mat4 projection;
	uniform mat4 model;
	
	void main() {
		gl_Position = projection * view * model * vec4(position, 1.0);
	}
"#;

const GIZMO_FRAG: &str = r#"
	precision mediump float;
	uniform vec3 color;
	
	void main() {
		gl_FragColor = vec4(color, 1.0);
	}
"#;

/// Immediate-mode debug gizmo renderer.
///
/// Provides methods for drawing wireframe primitives useful for debugging
/// and editor visualization. All gizmos are rendered as lines without
/// depth writing by default.
///
pub struct GizmoRenderer {
	program: WebGlProgram,
	line_buffer: WebGlBuffer,
	batch_vertices: RefCell<Vec<f32>>,
	unit_sphere_vertices: Vec<f32>,
	unit_cube_vertices: Vec<f32>,
}

impl GizmoRenderer {
	/// Creates a new gizmo renderer.
	///
	/// Compiles the gizmo shader and pre-generates cached primitive geometry.
	///
	/// # Panics
	///
	/// Panics if shader compilation fails. This should not happen with the
	/// embedded shaders unless the WebGL context is invalid.
	///
	pub fn new(gl: &GL) -> Self {
		let vert = compile_shader(gl, GIZMO_VERT, GL::VERTEX_SHADER).unwrap();
		let frag = compile_shader(gl, GIZMO_FRAG, GL::FRAGMENT_SHADER).unwrap();
		let program = link_program(gl, &vert, &frag).unwrap();
		let line_buffer = gl.create_buffer().expect("Failed to create gizmo buffer");

		Self { 
			program, 
			line_buffer,
			batch_vertices: RefCell::new(Vec::with_capacity(1024)),
			unit_sphere_vertices: Self::generate_sphere_vertices(24),
			unit_cube_vertices: Self::generate_cube_vertices(),
		}
	}

	/// Generates unit sphere wireframe vertices.
	///
	/// Creates three orthogonal circles (XY, XZ, YZ planes) with the
	/// specified number of segments each.
	fn generate_sphere_vertices(segments: usize) -> Vec<f32> {
		let mut vertices = Vec::with_capacity(segments * 6 * 6);
		
		for axis in 0..3 {
			for i in 0..segments {
				let a1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
				let a2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
				let (p1, p2) = match axis {
					0 => (
						Vec3::new(0.0, a1.cos(), a1.sin()),
						Vec3::new(0.0, a2.cos(), a2.sin()),
					),
					1 => (
						Vec3::new(a1.cos(), 0.0, a1.sin()),
						Vec3::new(a2.cos(), 0.0, a2.sin()),
					),
					_ => (
						Vec3::new(a1.cos(), a1.sin(), 0.0),
						Vec3::new(a2.cos(), a2.sin(), 0.0),
					),
				};

				vertices.extend_from_slice(&[p1.x, p1.y, p1.z, p2.x, p2.y, p2.z]);
			}
		}
		vertices
	}

	/// Generates unit cube wireframe vertices.
	///
	/// Creates the 12 edges of a unit cube centered at origin.
	fn generate_cube_vertices() -> Vec<f32> {
		let h = 0.5;
		vec![
			// Bottom face
			-h, -h, -h, h, -h, -h,
			h, -h, -h, h, -h, h,
			h, -h, h, -h, -h, h,
			-h, -h, h, -h, -h, -h,
			// Top face
			-h, h, -h, h, h, -h,
			h, h, -h, h, h, h,
			h, h, h, -h, h, h,
			-h, h, h, -h, h, -h,
			// Vertical edges
			-h, -h, -h, -h, h, -h,
			h, -h, -h, h, h, -h,
			h, -h, h, h, h, h,
			-h, -h, h, -h, h, h,
		]
	}

	fn upload_vertices(&self, gl: &GL, vertices: &[f32]) {
		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.line_buffer));
		let data = unsafe {
			std::slice::from_raw_parts(
				vertices.as_ptr() as *const u8,
				vertices.len() * 4,
			)
		};
		gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, data, GL::DYNAMIC_DRAW);
	}

	fn setup_draw(&self, gl: &GL, camera: &Camera, model: Mat4, color: Vec3) {
		gl.use_program(Some(&self.program));

		if let Some(loc) = gl.get_uniform_location(&self.program, "view") {
			gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &camera.view_matrix().to_cols_array());
		}
		if let Some(loc) = gl.get_uniform_location(&self.program, "projection") {
			gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &camera.projection_matrix().to_cols_array());
		}
		if let Some(loc) = gl.get_uniform_location(&self.program, "model") {
			gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &model.to_cols_array());
		}
		if let Some(loc) = gl.get_uniform_location(&self.program, "color") {
			gl.uniform3fv_with_f32_array(Some(&loc), &color.to_array());
		}

		let pos_loc = gl.get_attrib_location(&self.program, "position");

		if pos_loc >= 0 {
			gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.line_buffer));
			gl.enable_vertex_attrib_array(pos_loc as u32);
			gl.vertex_attrib_pointer_with_i32(pos_loc as u32, 3, GL::FLOAT, false, 0, 0);
		}
	}

	/// Draws a single line segment.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // Draw a diagonal line
	/// gizmos.line(
	///		&gl, &camera,
	///		Vec3::ZERO,
	///		Vec3::new(1.0, 1.0, 1.0),
	///		Vec3::new(1.0, 1.0, 1.0) // White
	/// );
	/// ```
	pub fn line(&self, gl: &GL, camera: &Camera, from: Vec3, to: Vec3, color: Vec3) {
		let vertices = [from.x, from.y, from.z, to.x, to.y, to.z];

		self.upload_vertices(gl, &vertices);
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, 2);
	}

	/// Draws a directional arrow with an arrowhead.
	///
	/// The arrow starts at `origin` and points in `direction` with the
	/// specified `length`. The arrowhead size is proportional to the length.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // Draw a velocity vector
	/// gizmos.arrow(&gl, &camera, position, velocity.normalize(), velocity.length(), Vec3::new(0.0, 1.0, 0.0));
	///
	/// // Draw a direction indicator
	/// gizmos.arrow(&gl, &camera, Vec3::ZERO, Vec3::Y, 2.0, Vec3::new(1.0, 1.0, 0.0));
	/// ```
	pub fn arrow(&self, gl: &GL, camera: &Camera, origin: Vec3, direction: Vec3, length: f32, color: Vec3) {
		let dir = direction.normalize();
		let end = origin + dir * length;
		
		let perp = if dir.y.abs() < 0.9 {
			dir.cross(Vec3::Y).normalize()
		} else {
			dir.cross(Vec3::X).normalize()
		};
		let perp2 = dir.cross(perp).normalize();
		
		let head_size = length * 0.15;
		let head_back = end - dir * head_size;

		{
			let mut verts = self.batch_vertices.borrow_mut();
			verts.clear();
			verts.extend_from_slice(&[
				origin.x, origin.y, origin.z,
				end.x, end.y, end.z,
				end.x, end.y, end.z,
				head_back.x + perp.x * head_size * 0.5,
				head_back.y + perp.y * head_size * 0.5,
				head_back.z + perp.z * head_size * 0.5,
				end.x, end.y, end.z,
				head_back.x - perp.x * head_size * 0.5,
				head_back.y - perp.y * head_size * 0.5,
				head_back.z - perp.z * head_size * 0.5,
				end.x, end.y, end.z,
				head_back.x + perp2.x * head_size * 0.5,
				head_back.y + perp2.y * head_size * 0.5,
				head_back.z + perp2.z * head_size * 0.5,
				end.x, end.y, end.z,
				head_back.x - perp2.x * head_size * 0.5,
				head_back.y - perp2.y * head_size * 0.5,
				head_back.z - perp2.z * head_size * 0.5,
			]);
		}

		self.upload_vertices(gl, &self.batch_vertices.borrow());
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, 10);
	}

	/// Draws a wireframe cube.
	///
	/// Renders the 12 edges of a cube centered at the given position.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // Draw bounding box
	/// gizmos.wire_cube(&gl, &camera, object.position, object.bounds, Vec3::new(0.0, 1.0, 1.0));
	/// ```
	pub fn wire_cube(&self, gl: &GL, camera: &Camera, center: Vec3, size: f32, color: Vec3) {
		self.upload_vertices(gl, &self.unit_cube_vertices);
		let model = Mat4::from_scale_rotation_translation(
			Vec3::splat(size),
			glam::Quat::IDENTITY,
			center
		);
		self.setup_draw(gl, camera, model, color);
		gl.draw_arrays(GL::LINES, 0, 24);
	}

	/// Draws a wireframe sphere.
	///
	/// Renders three orthogonal circles representing a sphere. This is a
	/// lightweight approximation suitable for debugging.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // Draw collision sphere
	/// gizmos.wire_sphere(&gl, &camera, entity.position, entity.collision_radius, Vec3::new(1.0, 0.0, 0.0));
	///
	/// // Draw light attenuation range
	/// gizmos.wire_sphere(&gl, &camera, light.position, light.range, Vec3::new(1.0, 1.0, 0.0));
	/// ```
	pub fn wire_sphere(&self, gl: &GL, camera: &Camera, center: Vec3, radius: f32, color: Vec3) {
		self.upload_vertices(gl, &self.unit_sphere_vertices);
		let model = Mat4::from_scale_rotation_translation(
			Vec3::splat(radius),
			glam::Quat::IDENTITY,
			center
		);
		self.setup_draw(gl, camera, model, color);
		gl.draw_arrays(GL::LINES, 0, (24 * 6) as i32);
	}

	
	/// Draws a ground plane grid.
	///
	/// Renders a square grid on the XZ plane (Y=0), useful for spatial
	/// reference and scale visualization.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // 10x10 meter grid with 1 meter cells
	/// gizmos.grid(&gl, &camera, 10.0, 10, Vec3::new(0.3, 0.3, 0.3));
	///
	/// // Fine grid for precise positioning
	/// gizmos.grid(&gl, &camera, 5.0, 50, Vec3::new(0.2, 0.2, 0.2));
	/// ```
	pub fn grid(&self, gl: &GL, camera: &Camera, size: f32, divisions: u32, color: Vec3) {
		let half = size * 0.5;
		let step = size / divisions as f32;
		
		{
			let mut verts = self.batch_vertices.borrow_mut();
			verts.clear();
			
			for i in 0..=divisions {
				let offset = -half + step * i as f32;
				verts.extend_from_slice(&[-half, 0.0, offset, half, 0.0, offset]);
				verts.extend_from_slice(&[offset, 0.0, -half, offset, 0.0, half]);
			}
		}

		self.upload_vertices(gl, &self.batch_vertices.borrow());
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, ((divisions + 1) * 4) as i32);
	}

	/// Draws RGB coordinate axes.
	///
	/// Renders three arrows representing the X (red), Y (green), and Z (blue)
	/// axes at the given position.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // World origin axes
	/// gizmos.axes(&gl, &camera, Vec3::ZERO, 1.0);
	///
	/// // Object local axes
	/// gizmos.axes(&gl, &camera, object.position, 0.5);
	/// ```
	pub fn axes(&self, gl: &GL, camera: &Camera, position: Vec3, size: f32) {
		self.arrow(gl, camera, position, Vec3::X, size, Vec3::new(1.0, 0.0, 0.0));
		self.arrow(gl, camera, position, Vec3::Y, size, Vec3::new(0.0, 1.0, 0.0));
		self.arrow(gl, camera, position, Vec3::Z, size, Vec3::new(0.0, 0.0, 1.0));
	}
}