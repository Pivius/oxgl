use glam::{Vec3, Mat4};
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL};

use super::{shader::{compile_shader, link_program}, camera::Camera};

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

/// Reusable debug drawing system
pub struct GizmoRenderer {
	program: WebGlProgram,
	line_buffer: WebGlBuffer,
}

impl GizmoRenderer {
	pub fn new(gl: &GL) -> Self {
		let vert = compile_shader(gl, GIZMO_VERT, GL::VERTEX_SHADER).unwrap();
		let frag = compile_shader(gl, GIZMO_FRAG, GL::FRAGMENT_SHADER).unwrap();
		let program = link_program(gl, &vert, &frag).unwrap();
		let line_buffer = gl.create_buffer().expect("Failed to create gizmo buffer");

		Self { program, line_buffer }
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

		// Uniforms
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

		// Attribute
		let pos_loc = gl.get_attrib_location(&self.program, "position");
		if pos_loc >= 0 {
			gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.line_buffer));
			gl.enable_vertex_attrib_array(pos_loc as u32);
			gl.vertex_attrib_pointer_with_i32(pos_loc as u32, 3, GL::FLOAT, false, 0, 0);
		}
	}

	/// Draw a line between two points
	pub fn line(&self, gl: &GL, camera: &Camera, from: Vec3, to: Vec3, color: Vec3) {
		let vertices = [from.x, from.y, from.z, to.x, to.y, to.z];
		self.upload_vertices(gl, &vertices);
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, 2);
	}

	/// Draw an arrow (line with arrowhead)
	pub fn arrow(&self, gl: &GL, camera: &Camera, origin: Vec3, direction: Vec3, length: f32, color: Vec3) {
		let dir = direction.normalize();
		let end = origin + dir * length;
		
		// Find perpendicular vectors for arrowhead
		let perp = if dir.y.abs() < 0.9 {
			dir.cross(Vec3::Y).normalize()
		} else {
			dir.cross(Vec3::X).normalize()
		};
		let perp2 = dir.cross(perp).normalize();
		
		let head_size = length * 0.15;
		let head_back = end - dir * head_size;
		
		let vertices = [
			// Main line
			origin.x, origin.y, origin.z,
			end.x, end.y, end.z,
			// Arrowhead
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
		];
		
		self.upload_vertices(gl, &vertices);
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, 10);
	}

	/// Draw a wireframe cube
	pub fn wire_cube(&self, gl: &GL, camera: &Camera, center: Vec3, size: f32, color: Vec3) {
		let h = size * 0.5;
		
		#[rustfmt::skip]
		let vertices = [
			// Bottom face
			center.x - h, center.y - h, center.z - h,  center.x + h, center.y - h, center.z - h,
			center.x + h, center.y - h, center.z - h,  center.x + h, center.y - h, center.z + h,
			center.x + h, center.y - h, center.z + h,  center.x - h, center.y - h, center.z + h,
			center.x - h, center.y - h, center.z + h,  center.x - h, center.y - h, center.z - h,
			// Top face
			center.x - h, center.y + h, center.z - h,  center.x + h, center.y + h, center.z - h,
			center.x + h, center.y + h, center.z - h,  center.x + h, center.y + h, center.z + h,
			center.x + h, center.y + h, center.z + h,  center.x - h, center.y + h, center.z + h,
			center.x - h, center.y + h, center.z + h,  center.x - h, center.y + h, center.z - h,
			// Vertical edges
			center.x - h, center.y - h, center.z - h,  center.x - h, center.y + h, center.z - h,
			center.x + h, center.y - h, center.z - h,  center.x + h, center.y + h, center.z - h,
			center.x + h, center.y - h, center.z + h,  center.x + h, center.y + h, center.z + h,
			center.x - h, center.y - h, center.z + h,  center.x - h, center.y + h, center.z + h,
		];

		self.upload_vertices(gl, &vertices);
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, 24);
	}

	/// Draw a wireframe sphere (approximation with circles)
	pub fn wire_sphere(&self, gl: &GL, camera: &Camera, center: Vec3, radius: f32, color: Vec3) {
		let segments = 24;
		let mut vertices = Vec::with_capacity(segments * 6 * 3);

		// Three axis-aligned circles
		for axis in 0..3 {
			for i in 0..segments {
				let a1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
				let a2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

				let (p1, p2) = match axis {
					0 => (
						center + Vec3::new(0.0, a1.cos() * radius, a1.sin() * radius),
						center + Vec3::new(0.0, a2.cos() * radius, a2.sin() * radius),
					),
					1 => (
						center + Vec3::new(a1.cos() * radius, 0.0, a1.sin() * radius),
						center + Vec3::new(a2.cos() * radius, 0.0, a2.sin() * radius),
					),
					_ => (
						center + Vec3::new(a1.cos() * radius, a1.sin() * radius, 0.0),
						center + Vec3::new(a2.cos() * radius, a2.sin() * radius, 0.0),
					),
				};

				vertices.extend_from_slice(&[p1.x, p1.y, p1.z, p2.x, p2.y, p2.z]);
			}
		}

		self.upload_vertices(gl, &vertices);
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, (segments * 6) as i32);
	}

	/// Draw a grid on the XZ plane
	pub fn grid(&self, gl: &GL, camera: &Camera, size: f32, divisions: u32, color: Vec3) {
		let half = size * 0.5;
		let step = size / divisions as f32;
		let mut vertices = Vec::new();

		for i in 0..=divisions {
			let offset = -half + step * i as f32;
			// X-axis lines
			vertices.extend_from_slice(&[-half, 0.0, offset, half, 0.0, offset]);
			// Z-axis lines
			vertices.extend_from_slice(&[offset, 0.0, -half, offset, 0.0, half]);
		}

		self.upload_vertices(gl, &vertices);
		self.setup_draw(gl, camera, Mat4::IDENTITY, color);
		gl.draw_arrays(GL::LINES, 0, ((divisions + 1) * 4) as i32);
	}

	/// Draw XYZ axes at a position
	pub fn axes(&self, gl: &GL, camera: &Camera, position: Vec3, size: f32) {
		self.arrow(gl, camera, position, Vec3::X, size, Vec3::new(1.0, 0.0, 0.0)); // Red X
		self.arrow(gl, camera, position, Vec3::Y, size, Vec3::new(0.0, 1.0, 0.0)); // Green Y
		self.arrow(gl, camera, position, Vec3::Z, size, Vec3::new(0.0, 0.0, 1.0)); // Blue Z
	}
}