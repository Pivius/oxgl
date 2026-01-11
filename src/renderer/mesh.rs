use web_sys::{WebGlBuffer, WebGlRenderingContext as GL};
use super::{Renderer, scene::Transform};

pub struct Mesh {
	pub vertex_buffer: WebGlBuffer,
	pub vertex_count: i32,
}

pub trait Drawable {
	fn draw(&self, renderer: &Renderer, transform: &Transform);
}

impl Mesh {
	pub fn new(gl: &GL, vertices: &[f32]) -> Self {
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
		}
	}
}

impl Drawable for Mesh {
	fn draw(&self, renderer: &Renderer, transform: &Transform) {
		renderer.draw_mesh(self, transform);
	}
}