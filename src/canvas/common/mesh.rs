use web_sys::{WebGlBuffer, WebGlRenderingContext as GL};

use super::{camera::Camera, material::Material};
use crate::{
	canvas::renderer_3d::{primitive::VertexData, light::Light},
	core::{Transform3D, Transformable}
};

pub struct Mesh {
	vertex_buffer: WebGlBuffer,
	vertex_count: i32,
	stride: i32,
	has_normals: bool,
	pub material: Material,
}

impl Mesh {
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
		// specular calculations
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