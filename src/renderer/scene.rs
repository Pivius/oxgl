use super::{mesh::Mesh, camera::Camera, light::Light, Renderer};
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
	pub lights: Vec<Light>,
}

impl Scene {
	pub fn new(camera: Camera) -> Self {
		Self { 
			camera, 
			objects: Vec::new(),
			lights: Vec::new(),
		}
	}

	pub fn add(&mut self, mesh: Mesh, transform: Transform) -> usize {
		let id = self.objects.len();
		self.objects.push(SceneObject { mesh, transform });
		id
	}

	pub fn add_light(&mut self, light: Light) -> usize {
		let id = self.lights.len();
		self.lights.push(light);
		id
	}

	pub fn get_mut(&mut self, id: usize) -> Option<&mut SceneObject> {
		self.objects.get_mut(id)
	}

	pub fn get_light_mut(&mut self, id: usize) -> Option<&mut Light> {
		self.lights.get_mut(id)
	}

	pub fn render(&self, renderer: &Renderer) {
		renderer.gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
		
		for obj in &self.objects {
			obj.mesh.draw(&renderer.gl, &obj.transform, &self.camera, &self.lights);
		}
	}
}