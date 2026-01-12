pub mod mesh;
pub mod shader;
pub mod particles;
pub mod scene;
pub mod primitive;
pub mod camera;
pub mod util;
pub mod object;

use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL, wasm_bindgen::JsCast};

pub struct Renderer {
	pub gl: GL,
	pub canvas: HtmlCanvasElement,
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

		Self { gl, canvas }
	}

	pub fn clear(&self) {
		self.gl.clear_color(0.1, 0.1, 0.12, 1.0);
		self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
	}
}