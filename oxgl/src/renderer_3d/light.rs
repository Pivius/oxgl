//! Lighting System
//!
//! Provides light types and utilities for scene illumination.
//!

use glam::Vec3;
use web_sys::{WebGlProgram, WebGl2RenderingContext as GL};

/// Maximum number of lights supported per draw call.
pub const MAX_LIGHTS: usize = 4;

/// The type of a light source.
#[derive(Clone, Debug)]
pub enum LightType {
	Directional,
	Point { radius: f32 },
	Spot { angle: f32, outer_angle: f32 },
}

/// A light source in the scene.
///
/// ## Examples
///
/// ```ignore
/// // Point light
/// let point = Light::point(
///		Vec3::new(0.0, 5.0, 0.0),  // position
///		Vec3::new(1.0, 1.0, 1.0),  // white color
///		1.0,                       // intensity
///		10.0,                      // radius
/// );
///
/// // Directional light
/// let sun = Light::directional(
///		Vec3::new(-1.0, -1.0, 0.0), // direction
///		Vec3::new(1.0, 0.95, 0.8),  // warm white
///		0.8,                        // intensity
/// );
///
/// // Light with shadows
/// let shadow_light = Light::point(pos, color, intensity, radius)
///		.with_shadows(true);
/// ```
#[derive(Clone, Debug)]
pub struct Light {
	pub light_type: LightType,
	pub position: Vec3,
	pub direction: Vec3,
	pub color: Vec3,
	pub intensity: f32,
	pub cast_shadows: bool,
}

impl Light {
	pub fn directional(direction: Vec3, color: Vec3, intensity: f32) -> Self {
		Self {
			light_type: LightType::Directional,
			position: Vec3::ZERO,
			direction: direction.normalize(),
			color,
			intensity,
			cast_shadows: false,
		}
	}

	pub fn point(position: Vec3, color: Vec3, intensity: f32, radius: f32) -> Self {
		Self {
			light_type: LightType::Point { radius },
			position,
			direction: Vec3::ZERO,
			color,
			intensity,
			cast_shadows: false,
		}
	}

	pub fn spot(position: Vec3, direction: Vec3, color: Vec3, intensity: f32, angle: f32) -> Self {
		Self {
			light_type: LightType::Spot { angle, outer_angle: angle * 1.2 },
			position,
			direction: direction.normalize(),
			color,
			intensity,
			cast_shadows: false,
		}
	}

	/// Returns the light type as an integer for shader use.
	pub fn type_id(&self) -> i32 {
		match self.light_type {
			LightType::Directional => 0,
			LightType::Point { .. } => 1,
			LightType::Spot { .. } => 2,
		}
	}

	pub fn radius(&self) -> f32 {
		match self.light_type {
			LightType::Point { radius } => radius,
			_ => 0.0,
		}
	}

	pub fn with_shadows(mut self, cast: bool) -> Self {
		self.cast_shadows = cast;
		self
	}

	pub fn apply_uniforms(&self, gl: &GL, program: &WebGlProgram) {
		if let Some(loc) = gl.get_uniform_location(program, "lightType") {
			gl.uniform1i(Some(&loc), self.type_id());
		}
		if let Some(loc) = gl.get_uniform_location(program, "lightDirection") {
			gl.uniform3fv_with_f32_array(Some(&loc), &self.direction.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, "lightPosition") {
			gl.uniform3fv_with_f32_array(Some(&loc), &self.position.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, "lightColor") {
			gl.uniform3fv_with_f32_array(Some(&loc), &self.color.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, "lightIntensity") {
			gl.uniform1f(Some(&loc), self.intensity);
		}
		if let Some(loc) = gl.get_uniform_location(program, "lightRadius") {
			gl.uniform1f(Some(&loc), self.radius());
		}
	}
}

// Hacky, but better than creating a new string every call
const LIGHT_UNIFORM_NAMES: [[&str; 6]; 4] = [
	["lights[0].type", "lights[0].direction", "lights[0].position", "lights[0].color", "lights[0].intensity", "lights[0].radius"],
	["lights[1].type", "lights[1].direction", "lights[1].position", "lights[1].color", "lights[1].intensity", "lights[1].radius"],
	["lights[2].type", "lights[2].direction", "lights[2].position", "lights[2].color", "lights[2].intensity", "lights[2].radius"],
	["lights[3].type", "lights[3].direction", "lights[3].position", "lights[3].color", "lights[3].intensity", "lights[3].radius"],
];

/// Uploads light data to shader uniforms.
///
/// Supports up to [`MAX_LIGHTS`] lights per draw call.
pub fn apply_lights(gl: &GL, program: &WebGlProgram, lights: &[Light]) {

	if let Some(loc) = gl.get_uniform_location(program, "numLights") {
		gl.uniform1i(Some(&loc), lights.len().min(MAX_LIGHTS) as i32);
	}

	for (i, light) in lights.iter().take(MAX_LIGHTS).enumerate() {
		let names = &LIGHT_UNIFORM_NAMES[i];

		if let Some(loc) = gl.get_uniform_location(program, names[0]) {
			gl.uniform1i(Some(&loc), light.type_id());
		}
		if let Some(loc) = gl.get_uniform_location(program, names[1]) {
			gl.uniform3fv_with_f32_array(Some(&loc), &light.direction.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, names[2]) {
			gl.uniform3fv_with_f32_array(Some(&loc), &light.position.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, names[3]) {
			gl.uniform3fv_with_f32_array(Some(&loc), &light.color.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, names[4]) {
			gl.uniform1f(Some(&loc), light.intensity);
		}
		if let Some(loc) = gl.get_uniform_location(program, names[5]) {
			gl.uniform1f(Some(&loc), light.radius());
		}
	}
}