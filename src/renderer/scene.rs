use super::{object::{Object, ObjectKind}, mesh::Drawable};
use glam::{Vec3, Mat4, Quat};

#[derive(Clone, Debug)]
pub struct Transform {
	pub position: Vec3,
	pub rotation: Quat,
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			position: Vec3::ZERO,
			rotation: Quat::IDENTITY,
		}
	}
}

impl Transform {
	pub fn model_matrix(&self) -> Mat4 {
		Mat4::from_translation(self.position) * Mat4::from_quat(self.rotation)
	}
}

pub struct Scene {
	pub objects: Vec<Object>,
}

impl Scene {
	pub fn new() -> Self {
		Self { objects: Vec::new() }
	}

	pub fn add(&mut self, object: Object) {
		self.objects.push(object);
	}

	pub fn render(&self, renderer: &crate::renderer::Renderer) {
		for obj in &self.objects {
			match &obj.kind {
				ObjectKind::Mesh(mesh) => {
					mesh.draw(renderer, &obj.transform);
				}
				_ => {}
			}
		}
	}
}