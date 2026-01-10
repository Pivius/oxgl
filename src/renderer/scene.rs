use super::mesh::Mesh;
use web_sys::WebGlRenderingContext as GL;

pub struct Scene {
	pub meshes: Vec<Mesh>,
}

impl Scene {
	pub fn new() -> Self {
		Self { meshes: vec![] }
	}

	pub fn add_mesh(&mut self, mesh: Mesh) {
		self.meshes.push(mesh);
	}

	pub fn render(&self, gl: &GL) {
		for mesh in &self.meshes {
			mesh.draw(gl);
		}
	}
}