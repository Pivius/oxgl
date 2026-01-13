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

	pub fn apply_uniforms(&self, gl: &GL, program: &WebGlProgram) {
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
	}
}