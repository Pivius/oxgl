use super::{mesh::Mesh, camera::Camera, Renderer};
use glam::{Vec3, Mat4, Quat};

#[derive(Clone, Debug)]
pub struct Transform {
	pub position: Vec3,
	pub rotation: Quat,
	pub scale: Vec3,
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			position: Vec3::ZERO,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
		}
	}
}

impl Transform {
	pub fn new() -> Self { Self::default() }

	pub fn with_position(mut self, pos: Vec3) -> Self {
		self.position = pos;
		self
	}

	pub fn with_rotation(mut self, rot: Quat) -> Self {
		self.rotation = rot;
		self
	}

	pub fn with_scale(mut self, scale: Vec3) -> Self {
		self.scale = scale;
		self
	}

	pub fn model_matrix(&self) -> Mat4 {
		Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
	}
}

pub struct SceneObject {
	pub mesh: Mesh,
	pub transform: Transform,
}

pub struct Scene {
	pub camera: Camera,
	pub objects: Vec<SceneObject>,
}

impl Scene {
	pub fn new(camera: Camera) -> Self {
		Self { camera, objects: Vec::new() }
	}

	pub fn add(&mut self, mesh: Mesh, transform: Transform) -> usize {
		let id = self.objects.len();
		self.objects.push(SceneObject { mesh, transform });
		id
	}

	pub fn get_mut(&mut self, id: usize) -> Option<&mut SceneObject> {
		self.objects.get_mut(id)
	}

	pub fn render(&self, renderer: &Renderer) {
		for obj in &self.objects {
			obj.mesh.draw(&renderer.gl, &obj.transform, &self.camera);
		}
	}
}