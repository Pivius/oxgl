pub mod gl;
pub mod mesh;
pub mod shader;
pub mod particles;
pub mod scene;
pub mod primitive;
pub mod camera;
pub mod util;
pub mod object;

use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL, wasm_bindgen::JsCast};
use crate::renderer::{camera::Camera, mesh::Mesh, scene::Transform, shader::{compile_shader, link_program}};

pub struct Renderer {
	pub gl: GL,
	pub canvas: HtmlCanvasElement,
	pub program: Option<web_sys::WebGlProgram>,
	pub camera: Camera,
}

impl Renderer {
	pub fn new(canvas_id: &str) -> Self {
		let window = web_sys::window().unwrap();
		let document = window.document().unwrap();
		let canvas = document
			.get_element_by_id(canvas_id)
			.unwrap()
			.dyn_into::<HtmlCanvasElement>()
			.unwrap();
		let gl: GL = canvas
			.get_context("webgl")
			.unwrap()
			.unwrap()
			.dyn_into()
			.unwrap();
		let width = canvas.width() as f32;
		let height = canvas.height() as f32;
		let aspect = width / height;
		let camera: Camera = Camera::new(aspect);

		Self { gl, canvas, program: None, camera }
	}

	pub fn set_shader(&mut self, vert_code: &str, frag_code: &str) {
		let vert_shader = compile_shader(&self.gl, vert_code, GL::VERTEX_SHADER)
			.expect("vertex shader");
		let frag_shader = compile_shader(&self.gl, frag_code, GL::FRAGMENT_SHADER)
			.expect("fragment shader");
		let program = link_program(&self.gl, &vert_shader, &frag_shader)
			.expect("link program");

		self.gl.use_program(Some(&program));
		self.program = Some(program);
		self.set_camera_uniforms();
	}

	pub fn set_camera_uniforms(&self) {
		if let Some(program) = &self.program {
			// Projection
			if let Some(loc_proj) = self.gl.get_uniform_location(program, "projection") {
				let proj = self.camera.projection_matrix();

				self.gl.uniform_matrix4fv_with_f32_array(
					Some(&loc_proj), 
					false, 
					proj.to_cols_array().as_ref()
				);
			}
			// View
			if let Some(loc_view) = self.gl.get_uniform_location(program, "view") {
				let view = self.camera.view_matrix();

				self.gl.uniform_matrix4fv_with_f32_array(
					Some(&loc_view), 
					false, 
					view.to_cols_array().as_ref()
				);
			}
		}
	}

	pub fn clear(&self) {
		self.gl.clear_color(0.1, 0.1, 0.12, 1.0);
		self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
	}

	pub fn draw_mesh(&self, mesh: &Mesh, transform: &Transform) {
		if let Some(program) = &self.program {
			let pos_attrib = self.gl.get_attrib_location(program, "position") as u32;

			self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&mesh.vertex_buffer));
			self.gl.enable_vertex_attrib_array(pos_attrib);
			self.gl.vertex_attrib_pointer_with_i32(
				pos_attrib, 
				3, 
				GL::FLOAT, 
				false, 
				0, 
				0
			);

			let model = transform.model_matrix();

			if let Some(loc) = self.gl.get_uniform_location(program, "model") {
				self.gl.uniform_matrix4fv_with_f32_array(
					Some(&loc), 
					false, 
					model.to_cols_array().as_ref()
				);
			}

			self.gl.draw_arrays(GL::TRIANGLES, 0, mesh.vertex_count);
		}
	}

	pub fn resize(&mut self, width: f32, height: f32) {
		self.camera.set_aspect(width / height);
		self.set_camera_uniforms();
	}
}