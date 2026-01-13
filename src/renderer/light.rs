use glam::Vec3;
use web_sys::{WebGlProgram, WebGlRenderingContext as GL};

#[derive(Clone, Debug)]
pub enum LightType {
	Directional,
	Point { radius: f32 },
	Spot { angle: f32, outer_angle: f32 },
}

#[derive(Clone, Debug)]
pub struct Light {
	pub light_type: LightType,
	pub position: Vec3,
	pub direction: Vec3,
	pub color: Vec3,
	pub intensity: f32,
}

impl Light {
	pub fn directional(direction: Vec3, color: Vec3, intensity: f32) -> Self {
		Self {
			light_type: LightType::Directional,
			position: Vec3::ZERO,
			direction: direction.normalize(),
			color,
			intensity,
		}
	}

	pub fn point(position: Vec3, color: Vec3, intensity: f32, radius: f32) -> Self {
		Self {
			light_type: LightType::Point { radius },
			position,
			direction: Vec3::ZERO,
			color,
			intensity,
		}
	}

	pub fn spot(position: Vec3, direction: Vec3, color: Vec3, intensity: f32, angle: f32) -> Self {
		Self {
			light_type: LightType::Spot { angle, outer_angle: angle * 1.2 },
			position,
			direction: direction.normalize(),
			color,
			intensity,
		}
	}

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

pub fn apply_lights(gl: &GL, program: &WebGlProgram, lights: &[Light]) {
	const MAX_LIGHTS: usize = 4;

	if let Some(loc) = gl.get_uniform_location(program, "numLights") {
		gl.uniform1i(Some(&loc), lights.len().min(MAX_LIGHTS) as i32);
	}

	for (i, light) in lights.iter().take(MAX_LIGHTS).enumerate() {
		let prefix = format!("lights[{}].", i);

		if let Some(loc) = gl.get_uniform_location(program, &format!("{}type", prefix)) {
			gl.uniform1i(Some(&loc), light.type_id());
		}
		if let Some(loc) = gl.get_uniform_location(program, &format!("{}direction", prefix)) {
			gl.uniform3fv_with_f32_array(Some(&loc), &light.direction.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, &format!("{}position", prefix)) {
			gl.uniform3fv_with_f32_array(Some(&loc), &light.position.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, &format!("{}color", prefix)) {
			gl.uniform3fv_with_f32_array(Some(&loc), &light.color.to_array());
		}
		if let Some(loc) = gl.get_uniform_location(program, &format!("{}intensity", prefix)) {
			gl.uniform1f(Some(&loc), light.intensity);
		}
		if let Some(loc) = gl.get_uniform_location(program, &format!("{}radius", prefix)) {
			gl.uniform1f(Some(&loc), light.radius());
		}
	}
}