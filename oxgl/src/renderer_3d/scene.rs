//! Scene Management
//!
//! Provides a container for 3D objects, lights, and rendering configuration.
//! The scene handles object lifecycle, shadow mapping, and post-processing.
//!
//! ## Examples
//!
//! ```
//! use oxgl::renderer_3d::{Scene, Light, LightType, Primitive};
//! use oxgl::common::{Camera, Mesh, material::presets};
//! use oxgl::core::Transform3D;
//! use glam::Vec3;
//!
//! // Create a scene with a camera
//! let camera = Camera::new(Vec3::new(3.0, 3.0, 3.0), Vec3::ZERO, 16.0 / 9.0);
//! let mut scene = Scene::new(camera);
//!
//! // Add a cube
//! let mesh = Mesh::with_normals(&gl, &Primitive::Cube.vertices_with_normals(), material);
//! let cube_id = scene.add(mesh, Transform3D::new());
//!
//! // Add a light
//! let light = Light::directional(Vec3::new(-1.0, -1.0, -1.0), Vec3::ONE, 1.0);
//! scene.add_light(light);
//!
//! // Enable shadows
//! scene.enable_shadows(&gl)?;
//! ```
//!

use glam::{Vec3, Mat4};
use slotmap::SlotMap;
use web_sys::WebGl2RenderingContext as GL;
use super::{Light, LightType, GizmoRenderer, ShadowMap};
use crate::{
	common::{Mesh, Camera, Material, PostProcessStack}, 
	core::{ObjectId, LightId, Transform3D, Transformable},
	Renderer
};

/// A renderable object in the scene.
///
/// Combines a mesh with a transform to define both the geometry
/// and its position/orientation/scale in world space.
pub struct SceneObject {
	pub mesh: Mesh,
	pub transform: Transform3D,
}

/// Container for 3D objects, lights, and rendering state.
///
/// The scene manages:
/// - Object and light storage with stable IDs
/// - Shadow map generation
/// - Post-processing pipeline
/// - Camera configuration
///
/// ## Object Management
///
/// Objects and lights are stored in slot maps with stable IDs that remain
/// valid even after other items are removed.
///
pub struct Scene {
	pub camera: Camera,
	pub objects: SlotMap<ObjectId, SceneObject>,
	pub lights: SlotMap<LightId, Light>,
	pub shadow_map: Option<ShadowMap>,
	shadow_material: Option<Material>,
	pub shadows_enabled: bool,
	pub post_process: Option<PostProcessStack>,
}

/// Configuration for debug visualization.
///
/// Controls which debug gizmos are rendered when calling
/// [`Scene::render_debug`].
///
/// ## Defaults
///
/// All visualization options are disabled by default.
///
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
			show_grid: false,
			show_axes: false,
			show_light_gizmos: false,
			show_object_bounds: false,
			grid_size: 10.0,
			grid_divisions: 10,
		}
	}
}

impl Scene {
	/// Creates a new empty scene with the given camera.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::renderer_3d::Scene;
	/// use oxgl::common::Camera;
	/// use glam::Vec3;
	///
	/// let camera = Camera::new(Vec3::new(0.0, 5.0, 10.0), Vec3::ZERO, 16.0 / 9.0);
	/// let scene = Scene::new(camera);
	/// ```
	pub fn new(camera: Camera) -> Self {
		Self { 
			camera, 
			objects: SlotMap::with_key(),
			lights: SlotMap::with_key(),
			shadow_map: None,
			shadow_material: None,
			shadows_enabled: false,
			post_process: None,
		}
	}

	pub fn add(&mut self, mesh: Mesh, transform: Transform3D) -> ObjectId {
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

	/// Enables shadow mapping for the scene.
	///
	/// Creates the shadow map framebuffer and compiles the shadow depth shader.
	/// Shadows will be cast from the first light with `cast_shadows` enabled.
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - Shadow map framebuffer creation fails
	/// - Shadow shader compilation fails
	///
	/// # Examples
	///
	/// ```
	/// scene.enable_shadows(&gl)?;
	///
	/// // Make a light cast shadows
	/// let mut light = Light::directional(Vec3::new(-1.0, -1.0, -1.0), Vec3::ONE, 1.0);
	/// light.cast_shadows = true;
	/// scene.add_light(light);
	/// ```
	pub fn enable_shadows(&mut self, gl: &GL) -> Result<(), String> {
		self.shadow_map = Some(ShadowMap::new(gl)?);
		self.shadows_enabled = true;
		
		let shadow_vert = include_str!("../shaders/shadow_depth.vert");
		let shadow_frag = include_str!("../shaders/shadow_depth.frag");
		self.shadow_material = Some(Material::from_source(gl, shadow_vert, shadow_frag)?);
		
		Ok(())
	}

	/// Disables shadow rendering.
	///
	/// Shadows will no longer be rendered, but the shadow map resources
	/// are retained for quick re-enabling.
	pub fn disable_shadows(&mut self) {
		self.shadows_enabled = false;
	}

	/// Checks if any light in the scene casts shadows.
	fn has_shadow_casting_light(&self) -> bool {
		self.lights.values().any(|l| l.cast_shadows)
	}

	/// Renders the shadow depth pass.
	///
	/// Renders all objects from the light's perspective into the shadow map.
	fn render_shadow_pass(&mut self, gl: &GL, canvas_width: i32, canvas_height: i32) {
		if !self.shadows_enabled || !self.has_shadow_casting_light() {
			return;
		}

		let shadow_light = self.lights.values().find(|l| l.cast_shadows);
		
		let (shadow_map, shadow_material) = match (&mut self.shadow_map, &self.shadow_material) {
			(Some(sm), Some(mat)) => (sm, mat),
			_ => return,
		};

		let light = match shadow_light {
			Some(l) => l.clone(),
			None => return,
		};

		match &light.light_type {
			LightType::Directional => {
				shadow_map.update_directional(light.direction, Vec3::ZERO, 10.0);
			}
			LightType::Point { radius } => {
				let target = Vec3::ZERO;
				shadow_map.update_point(light.position, target, std::f32::consts::FRAC_PI_2, 0.1, *radius);
			}
			LightType::Spot { angle, .. } => {
				let target = light.position + light.direction;
				shadow_map.update_point(light.position, target, *angle, 0.1, 50.0);
			}
		}

		shadow_map.bind(gl);

		gl.enable(GL::DEPTH_TEST);
		gl.clear(GL::DEPTH_BUFFER_BIT);

		let program = shadow_material.program();
		gl.use_program(Some(program));

		if let Some(loc) = gl.get_uniform_location(program, "lightSpace") {
			gl.uniform_matrix4fv_with_f32_array(
				Some(&loc), false, &shadow_map.light_space.to_cols_array()
			);
		}

		for obj in self.objects.values() {
			if let Some(loc) = gl.get_uniform_location(program, "model") {
				gl.uniform_matrix4fv_with_f32_array(
					Some(&loc), false, &obj.transform.to_matrix().to_cols_array()
				);
			}

			obj.mesh.draw_depth_only(gl, program);
		}

		shadow_map.unbind(gl, canvas_width, canvas_height);
	}

	/// Sets the post-processing effect stack.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::common::{PostProcessStack, pp_presets};
	///
	/// let mut pp = PostProcessStack::new(&gl, 800, 600)?;
	/// pp.push(pp_presets::vignette(&gl, 0.8, 0.4));
	/// pp.push(pp_presets::chromatic_aberration(&gl, 5.0));
	///
	/// scene.set_post_process(pp);
	/// ```
	pub fn set_post_process(&mut self, stack: PostProcessStack) {
		self.post_process = Some(stack);
	}

	/// Renders the scene.
	///
	/// Executes the full rendering pipeline:
	/// 1. Binds post-process framebuffer (if enabled)
	/// 2. Clears color and depth buffers
	/// 3. Renders shadow pass (if enabled)
	/// 4. Renders all objects with lighting
	/// 5. Applies post-processing effects (if enabled)
	///
	/// # Examples
	///
	/// ```
	/// // In your render loop
	/// scene.render(&renderer, elapsed_time);
	/// ```
	pub fn render(&mut self, renderer: &Renderer, time: f32) {
		let gl = &renderer.gl;
		let canvas = renderer.canvas();
		let width = canvas.width() as i32;
		let height = canvas.height() as i32;
		let shadows_active = self.shadows_enabled && self.has_shadow_casting_light();

		if let Some(pp) = &self.post_process {
			pp.begin(gl);
		} else {
			gl.bind_framebuffer(GL::FRAMEBUFFER, None);
			gl.viewport(0, 0, width, height);
		}

		gl.clear_color(0.1, 0.1, 0.1, 1.0);
		gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

		if shadows_active {
			self.render_shadow_pass(gl, width, height);

			if let Some(pp) = &self.post_process {
				pp.begin(gl);
			}
		}

		gl.enable(GL::DEPTH_TEST);
		
		let lights: Vec<Light> = self.lights.values().cloned().collect();
		
		let light_space = if shadows_active {
			self.shadow_map.as_ref()
				.map(|sm| {
					sm.bind_texture(gl, 0);
					sm.light_space
				})
				.unwrap_or(Mat4::IDENTITY)
		} else {
			Mat4::IDENTITY
		};

		for obj in self.objects.values_mut() {
			let program = obj.mesh.material.program();

			gl.use_program(Some(program));
			
			if let Some(loc) = gl.get_uniform_location(program, "shadowsEnabled") {
				gl.uniform1i(Some(&loc), if shadows_active { 1 } else { 0 });
			}

			if shadows_active {
				if let Some(loc) = gl.get_uniform_location(program, "lightSpace") {
					gl.uniform_matrix4fv_with_f32_array(
						Some(&loc), false, &light_space.to_cols_array()
					);
				}
				if let Some(loc) = gl.get_uniform_location(program, "shadowMap") {
					gl.uniform1i(Some(&loc), 0);
				}
			}
			
			obj.mesh.draw(gl, &obj.transform, &self.camera, &lights);
		}

		if let Some(pp) = &mut self.post_process {
			pp.end(gl, time);
		}
	}

	/// Renders debug visualization gizmos.
	///
	/// Draws wireframe debug primitives based on the provided settings.
	/// Should be called after [`render`](Self::render) for proper layering.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::renderer_3d::{GizmoRenderer, DebugSettings};
	///
	/// let gizmos = GizmoRenderer::new(&gl);
	/// let settings = DebugSettings {
	///		show_grid: true,
	///		show_axes: true,
	///		show_light_gizmos: true,
	///		..Default::default()
	/// };
	///
	/// scene.render(&renderer, time);
	/// scene.render_debug(&renderer, &gizmos, &settings, true);
	/// ```
	pub fn render_debug(&self, renderer: &Renderer, gizmos: &GizmoRenderer, settings: &DebugSettings, disable_depth: bool) {
		let gl = &renderer.gl;

		if disable_depth {
			gl.disable(GL::DEPTH_TEST);
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
			gl.enable(GL::DEPTH_TEST);
		}
	}
}