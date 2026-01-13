use glam::{Vec3, Mat4, Quat};
use slotmap::{SlotMap, new_key_type};

use super::{mesh::Mesh, camera::Camera, light::{Light, LightType}, gizmo::GizmoRenderer, Renderer};

new_key_type! {
	pub struct ObjectId;
	pub struct LightId;
}

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
	pub objects: SlotMap<ObjectId, SceneObject>,
	pub lights: SlotMap<LightId, Light>,
}

pub struct DebugSettings {
	pub show_grid: bool,
	pub show_axes: bool,
	pub show_light_gizmos: bool,
	pub show_object_bounds: bool,
	pub grid_size: f32,
	pub grid_divisions: u32,
}

impl Default for DebugSettings {
	fn default() -> Self {
		Self {
			show_grid: true,
			show_axes: true,
			show_light_gizmos: true,
			show_object_bounds: false,
			grid_size: 10.0,
			grid_divisions: 10,
		}
	}
}

impl Scene {
	pub fn new(camera: Camera) -> Self {
		Self { 
			camera, 
			objects: SlotMap::with_key(),
			lights: SlotMap::with_key(),
		}
	}

	pub fn add(&mut self, mesh: Mesh, transform: Transform) -> ObjectId {
		self.objects.insert(SceneObject { mesh, transform })
	}

	pub fn add_light(&mut self, light: Light) -> LightId {
		self.lights.insert(light)
	}

	pub fn remove(&mut self, id: ObjectId) -> Option<SceneObject> {
		self.objects.remove(id)
	}

	pub fn remove_light(&mut self, id: LightId) -> Option<Light> {
		self.lights.remove(id)
	}

	pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut SceneObject> {
		self.objects.get_mut(id)
	}

	pub fn get_light_mut(&mut self, id: LightId) -> Option<&mut Light> {
		self.lights.get_mut(id)
	}

	pub fn render(&mut self, renderer: &Renderer) {
		renderer.gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
		
		let lights: Vec<Light> = self.lights.values().cloned().collect();
		
		for obj in self.objects.values_mut() {
			obj.mesh.draw(&renderer.gl, &obj.transform, &self.camera, &lights);
		}
	}

	pub fn render_debug(&self, renderer: &Renderer, gizmos: &GizmoRenderer, settings: &DebugSettings, disable_depth: bool) {
		let gl = &renderer.gl;

		if disable_depth {
			gl.disable(web_sys::WebGlRenderingContext::DEPTH_TEST);
		}

		if settings.show_grid {
			gizmos.grid(
				gl, 
				&self.camera, 
				settings.grid_size, 
				settings.grid_divisions, 
				Vec3::new(0.3, 0.3, 0.3)
			);
		}

		if settings.show_axes {
			gizmos.axes(gl, &self.camera, Vec3::ZERO, 1.0);
		}

		if settings.show_light_gizmos {
			for light in self.lights.values() {
				match &light.light_type {
					LightType::Directional => {
						let origin = Vec3::new(0.0, 3.0, 0.0);
						gizmos.arrow(gl, &self.camera, origin, light.direction, 2.0, Vec3::new(1.0, 1.0, 0.0));
					}
					LightType::Point { radius } => {
						gizmos.wire_sphere(gl, &self.camera, light.position, *radius * 0.1, Vec3::new(1.0, 1.0, 0.0));
						gizmos.wire_sphere(gl, &self.camera, light.position, *radius, Vec3::new(0.5, 0.5, 0.0));
					}
					LightType::Spot { .. } => {
						gizmos.arrow(gl, &self.camera, light.position, light.direction, 1.5, Vec3::new(1.0, 0.8, 0.0));
					}
				}
			}
		}

		if settings.show_object_bounds {
			for obj in self.objects.values() {
				gizmos.wire_cube(gl, &self.camera, obj.transform.position, obj.transform.scale.max_element(), Vec3::new(0.0, 1.0, 1.0));
			}
		}

		if disable_depth {
			gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
		}
	}
}