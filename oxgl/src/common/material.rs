use std::collections::HashMap;
use glam::{Vec2, Vec3, Vec4, Mat4};
use web_sys::{WebGlProgram, WebGl2RenderingContext as GL};

use crate::renderer_3d::{Light, apply_lights};
use super::{compile_shader, link_program};

#[derive(Clone, Debug)]
pub enum Uniform {
	Float(f32),
	Vec2(Vec2),
	Vec3(Vec3),
	Vec4(Vec4),
	Mat4(Mat4),
}

impl Uniform {
	pub fn apply(&self, gl: &GL, location: &web_sys::WebGlUniformLocation) {
		match self {
			Uniform::Float(v) => gl.uniform1f(Some(location), *v),
			Uniform::Vec2(v) => gl.uniform2fv_with_f32_array(Some(location), &v.to_array()),
			Uniform::Vec3(v) => gl.uniform3fv_with_f32_array(Some(location), &v.to_array()),
			Uniform::Vec4(v) => gl.uniform4fv_with_f32_array(Some(location), &v.to_array()),
			Uniform::Mat4(v) => gl.uniform_matrix4fv_with_f32_array(Some(location), false, &v.to_cols_array()),
		}
	}
}

pub struct Material {
	program: WebGlProgram,
	uniforms: HashMap<String, Uniform>,
	pub needs_normals: bool,
}

impl Material {
	pub fn from_source(gl: &GL, vert_src: &str, frag_src: &str) -> Result<Self, String> {
		let vert_shader = compile_shader(gl, vert_src, GL::VERTEX_SHADER)?;
		let frag_shader = compile_shader(gl, frag_src, GL::FRAGMENT_SHADER)?;
		let program = link_program(gl, &vert_shader, &frag_shader)?;
		let needs_normals = vert_src.contains("attribute vec3 normal");

		Ok(Self {
			program,
			uniforms: HashMap::new(),
			needs_normals,
		})
	}

	pub fn set(&mut self, name: &str, value: Uniform) -> &mut Self {
		self.uniforms.insert(name.to_string(), value);
		self
	}

	pub fn set_float(&mut self, name: &str, v: f32) -> &mut Self {
		self.set(name, Uniform::Float(v))
	}

	pub fn set_vec3(&mut self, name: &str, v: Vec3) -> &mut Self {
		self.set(name, Uniform::Vec3(v))
	}

	pub fn set_vec4(&mut self, name: &str, v: Vec4) -> &mut Self {
		self.set(name, Uniform::Vec4(v))
	}

	pub fn set_color(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
		self.set_vec3("color", Vec3::new(r, g, b))
	}

	pub fn set_color4(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
		self.set_vec4("color", Vec4::new(r, g, b, a))
	}

	pub fn program(&self) -> &WebGlProgram {
		&self.program
	}

	pub fn apply(&self, gl: &GL, lights: &[Light]) {
		for (name, value) in &self.uniforms {
			if let Some(loc) = gl.get_uniform_location(&self.program, name) {
				value.apply(gl, &loc);
			}
		}

		apply_lights(gl, &self.program, lights);
	}
}

impl Clone for Material {
	fn clone(&self) -> Self {
		Self {
			program: self.program.clone(),
			uniforms: self.uniforms.clone(),
			needs_normals: self.needs_normals,
		}
	}
}

pub struct MaterialBuilder<'a> {
	gl: &'a GL,
	vert_src: &'a str,
	frag_src: &'a str,
	uniforms: HashMap<String, Uniform>,
}

impl<'a> MaterialBuilder<'a> {
	pub fn new(gl: &'a GL, vert_src: &'a str, frag_src: &'a str) -> Self {
		Self {
			gl,
			vert_src,
			frag_src,
			uniforms: HashMap::new(),
		}
	}

	pub fn uniform(mut self, name: &str, value: Uniform) -> Self {
		self.uniforms.insert(name.to_string(), value);
		self
	}

	pub fn color3(self, r: f32, g: f32, b: f32) -> Self {
		self.uniform("color", Uniform::Vec3(Vec3::new(r, g, b)))
	}

	pub fn color4(self, r: f32, g: f32, b: f32, a: f32) -> Self {
		self.uniform("color", Uniform::Vec4(Vec4::new(r, g, b, a)))
	}

	pub fn ambient(self, v: f32) -> Self {
		self.uniform("ambient", Uniform::Float(v))
	}

	pub fn shininess(self, v: f32) -> Self {
		self.uniform("shininess", Uniform::Float(v))
	}

	pub fn specular(self, v: f32) -> Self {
		self.uniform("specularStrength", Uniform::Float(v))
	}

	pub fn build(self) -> Material {
		let mut mat = Material::from_source(self.gl, self.vert_src, self.frag_src)
			.expect("Failed to compile shader");
		mat.uniforms = self.uniforms;
		mat
	}
}

pub mod presets {
	use super::*;
	use glam::{Vec3, Vec4};
	use web_sys::WebGl2RenderingContext as GL;

	const UNLIT_VERT: &str = include_str!("../shaders/unlit.vert");
	const UNLIT_FRAG: &str = include_str!("../shaders/unlit.frag");
	const LAMBERT_VERT: &str = include_str!("../shaders/lambert.vert");
	const LAMBERT_FRAG: &str = include_str!("../shaders/lambert.frag");
	const PHONG_VERT: &str = include_str!("../shaders/phong.vert");
	const PHONG_FRAG: &str = include_str!("../shaders/phong.frag");

	pub fn unlit(gl: &GL, color: Vec4) -> Material {
		MaterialBuilder::new(gl, UNLIT_VERT, UNLIT_FRAG)
			.color4(color.x, color.y, color.z, color.w)
			.build()
	}

	pub fn lambert(gl: &GL, color: Vec3) -> Material {
		MaterialBuilder::new(gl, LAMBERT_VERT, LAMBERT_FRAG)
			.color3(color.x, color.y, color.z)
			.ambient(0.1)
			.build()
	}

	pub fn phong(gl: &GL, color: Vec3) -> Material {
		MaterialBuilder::new(gl, PHONG_VERT, PHONG_FRAG)
			.color3(color.x, color.y, color.z)
			.ambient(0.1)
			.shininess(32.0)
			.specular(0.5)
			.build()
	}
}