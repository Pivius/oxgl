use std::collections::HashMap;
use web_sys::{WebGlBuffer, WebGlRenderingContext as GL, WebGlProgram};
use glam::{Mat4, Vec3, Vec4};

use super::{scene::Transform, camera::Camera, shader::{compile_shader, link_program}};

#[derive(Clone)]
pub enum UniformValue {
	Mat4(Mat4),
	Vec4(Vec4),
	Vec3(Vec3),
	Float(f32),
}

#[derive(Clone)]
pub struct AttributeInfo {
	pub size: i32,
	pub stride: i32,
	pub offset: i32,
}

pub struct Mesh {
	vertex_buffer: WebGlBuffer,
	vertex_count: i32,
	program: Option<WebGlProgram>,
	uniforms: HashMap<String, UniformValue>,
	attributes: HashMap<String, AttributeInfo>,
}

pub struct MeshBuilder<'a> {
	gl: &'a GL,
	vertices: &'a [f32],
	vert_src: Option<&'a str>,
	frag_src: Option<&'a str>,
	uniforms: HashMap<String, UniformValue>,
	attributes: HashMap<String, AttributeInfo>,
}

impl<'a> MeshBuilder<'a> {
	pub fn new(gl: &'a GL, vertices: &'a [f32]) -> Self {
		Self {
			gl,
			vertices,
			vert_src: None,
			frag_src: None,
			uniforms: HashMap::new(),
			attributes: HashMap::new(),
		}
	}

	pub fn shader(mut self, vert: &'a str, frag: &'a str) -> Self {
		self.vert_src = Some(vert);
		self.frag_src = Some(frag);
		self
	}

	pub fn uniform(mut self, name: &str, value: UniformValue) -> Self {
		self.uniforms.insert(name.to_string(), value);
		self
	}

	pub fn attribute(mut self, name: &str, size: i32) -> Self {
		self.attributes.insert(name.to_string(), AttributeInfo {
			size,
			stride: 0,
			offset: 0,
		});
		self
	}

	pub fn attribute_full(mut self, name: &str, size: i32, stride: i32, offset: i32) -> Self {
		self.attributes.insert(name.to_string(), AttributeInfo { size, stride, offset });
		self
	}

	pub fn build(self) -> Mesh {
		let vertex_buffer = self.gl.create_buffer().expect("Failed to create buffer");

		self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));

		let vert_array = unsafe {
			std::slice::from_raw_parts(
				self.vertices.as_ptr() as *const u8,
				self.vertices.len() * std::mem::size_of::<f32>(),
			)
		};

		self.gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, vert_array, GL::STATIC_DRAW);

		let program = match (self.vert_src, self.frag_src) {
			(Some(vert), Some(frag)) => {
				let vert_shader = compile_shader(self.gl, vert, GL::VERTEX_SHADER)
					.expect("Vertex shader compilation failed");
				let frag_shader = compile_shader(self.gl, frag, GL::FRAGMENT_SHADER)
					.expect("Fragment shader compilation failed");
				Some(link_program(self.gl, &vert_shader, &frag_shader)
					.expect("Shader linking failed"))
			}
			_ => None,
		};

		Mesh {
			vertex_buffer,
			vertex_count: (self.vertices.len() / 3) as i32,
			program,
			uniforms: self.uniforms,
			attributes: self.attributes,
		}
	}
}

impl Mesh {
	pub fn builder<'a>(gl: &'a GL, vertices: &'a [f32]) -> MeshBuilder<'a> {
		MeshBuilder::new(gl, vertices)
	}

	pub fn set_uniform(&mut self, name: &str, value: UniformValue) {
		self.uniforms.insert(name.to_string(), value);
	}

	pub fn draw(&self, gl: &GL, transform: &Transform, camera: &Camera) {
		let program = match &self.program {
			Some(p) => p,
			None => return,
		};

		gl.use_program(Some(program));

		for (name, value) in &self.uniforms {
			let location = gl.get_uniform_location(program, name);
			if location.is_some() {
				match value {
					UniformValue::Mat4(m) => {
						gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &m.to_cols_array());
					}
					UniformValue::Vec4(v) => {
						gl.uniform4fv_with_f32_array(location.as_ref(), v.to_array().as_ref());
					}
					UniformValue::Vec3(v) => {
						gl.uniform3fv_with_f32_array(location.as_ref(), v.to_array().as_ref());
					}
					UniformValue::Float(f) => {
						gl.uniform1f(location.as_ref(), *f);
					}
				}
			}
		}

		if let Some(loc) = gl.get_uniform_location(program, "model") {
			gl.uniform_matrix4fv_with_f32_array(
				Some(&loc), 
				false, 
				&transform.model_matrix().to_cols_array()
			);
		}
		if let Some(loc) = gl.get_uniform_location(program, "view") {
			gl.uniform_matrix4fv_with_f32_array(
				Some(&loc), 
				false, 
				&camera.view_matrix().to_cols_array()
			);
		}
		if let Some(loc) = gl.get_uniform_location(program, "projection") {
			gl.uniform_matrix4fv_with_f32_array(
				Some(&loc), 
				false, 
				&camera.projection_matrix().to_cols_array()
			);
		}

		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
		for (name, info) in &self.attributes {
			let loc = gl.get_attrib_location(program, name);
			if loc >= 0 {
				let loc = loc as u32;
				gl.enable_vertex_attrib_array(loc);
				gl.vertex_attrib_pointer_with_i32(
					loc,
					info.size,
					GL::FLOAT,
					false,
					info.stride,
					info.offset,
				);
			}
		}

		gl.draw_arrays(GL::TRIANGLES, 0, self.vertex_count);
	}
}