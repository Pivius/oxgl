use web_sys::{WebGlBuffer, WebGlRenderingContext as GL};

pub struct Mesh {
	pub vertex_buffer: WebGlBuffer,
	pub vertex_count: i32,
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

	pub fn draw(&self, gl: &GL) {
		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
		gl.enable_vertex_attrib_array(0);
		gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
		gl.draw_arrays(GL::TRIANGLES, 0, self.vertex_count);
	}
}