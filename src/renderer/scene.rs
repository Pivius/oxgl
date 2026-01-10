use super::mesh::Mesh;
use glam::{Vec3, Mat4};

#[derive(Clone, Debug)]
pub struct Transform {
	pub position: Vec3,
	pub rotation: f32,
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			position: Vec3::ZERO,
			rotation: 0.0,
		}
	}
}

impl Transform {
	pub fn model_matrix(&self) -> Mat4 {
		Mat4::from_translation(self.position) * Mat4::from_rotation_z(self.rotation)
	}
}

pub struct MeshInstance {
	pub mesh: Mesh,
	pub transform: Transform,
}

pub struct Scene {
	pub objects: Vec<MeshInstance>,
}

impl Scene {
	pub fn new() -> Self {
		Self { objects: Vec::new() }
	}

	pub fn add(&mut self, mesh: Mesh, transform: Transform) {
		self.objects.push(MeshInstance { mesh, transform });
	}

	pub fn render(&self, renderer: &crate::renderer::Renderer) {
		for obj in &self.objects {
			renderer.draw_mesh(&obj.mesh, &obj.transform);
		}
	}
}