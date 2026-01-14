use std::collections::HashMap;
use web_sys::{
	WebGlFramebuffer, WebGlTexture, WebGlRenderbuffer, WebGlBuffer, WebGlProgram,
	WebGl2RenderingContext as GL,
};
use glam::{Vec2, Vec3, Vec4};
use crate::common::{compile_shader, link_program};

#[derive(Clone, Debug)]
pub enum PostProcessUniform {
	Float(f32),
	Vec2(Vec2),
	Vec3(Vec3),
	Vec4(Vec4),
	Int(i32),
}

impl PostProcessUniform {
	pub fn apply(&self, gl: &GL, location: &web_sys::WebGlUniformLocation) {
		match self {
			PostProcessUniform::Float(v) => gl.uniform1f(Some(location), *v),
			PostProcessUniform::Vec2(v) => gl.uniform2fv_with_f32_array(Some(location), &v.to_array()),
			PostProcessUniform::Vec3(v) => gl.uniform3fv_with_f32_array(Some(location), &v.to_array()),
			PostProcessUniform::Vec4(v) => gl.uniform4fv_with_f32_array(Some(location), &v.to_array()),
			PostProcessUniform::Int(v) => gl.uniform1i(Some(location), *v),
		}
	}
}

pub struct PostProcessEffect {
	program: WebGlProgram,
	uniforms: HashMap<String, PostProcessUniform>,
	pub enabled: bool,
}

impl PostProcessEffect {
	pub fn new(gl: &GL, frag_src: &str) -> Result<Self, String> {
		let vert_src = include_str!("../pp_shaders/postprocess.vert");
		let vert_shader = compile_shader(gl, vert_src, GL::VERTEX_SHADER)?;
		let frag_shader = compile_shader(gl, frag_src, GL::FRAGMENT_SHADER)?;
		let program = link_program(gl, &vert_shader, &frag_shader)?;

		Ok(Self {
			program,
			uniforms: HashMap::new(),
			enabled: true,
		})
	}

	pub fn set(&mut self, name: &str, value: PostProcessUniform) -> &mut Self {
		self.uniforms.insert(name.to_string(), value);
		self
	}

	pub fn set_float(&mut self, name: &str, v: f32) -> &mut Self {
		self.set(name, PostProcessUniform::Float(v))
	}

	pub fn set_vec2(&mut self, name: &str, v: Vec2) -> &mut Self {
		self.set(name, PostProcessUniform::Vec2(v))
	}

	pub fn set_vec3(&mut self, name: &str, v: Vec3) -> &mut Self {
		self.set(name, PostProcessUniform::Vec3(v))
	}

	pub fn program(&self) -> &WebGlProgram {
		&self.program
	}

	pub fn apply_uniforms(&self, gl: &GL) {
		for (name, value) in &self.uniforms {
			if let Some(loc) = gl.get_uniform_location(&self.program, name) {
				value.apply(gl, &loc);
			}
		}
	}
}

pub struct PostProcessEffectBuilder<'a> {
	gl: &'a GL,
	frag_src: &'a str,
	uniforms: HashMap<String, PostProcessUniform>,
}

impl<'a> PostProcessEffectBuilder<'a> {
	pub fn new(gl: &'a GL, frag_src: &'a str) -> Self {
		Self {
			gl,
			frag_src,
			uniforms: HashMap::new(),
		}
	}

	pub fn uniform(mut self, name: &str, value: PostProcessUniform) -> Self {
		self.uniforms.insert(name.to_string(), value);
		self
	}

	pub fn float(self, name: &str, v: f32) -> Self {
		self.uniform(name, PostProcessUniform::Float(v))
	}

	pub fn vec2(self, name: &str, v: Vec2) -> Self {
		self.uniform(name, PostProcessUniform::Vec2(v))
	}

	pub fn vec3(self, name: &str, v: Vec3) -> Self {
		self.uniform(name, PostProcessUniform::Vec3(v))
	}

	pub fn int(self, name: &str, v: i32) -> Self {
		self.uniform(name, PostProcessUniform::Int(v))
	}

	pub fn build(self) -> PostProcessEffect {
		let mut effect = PostProcessEffect::new(self.gl, self.frag_src)
			.expect("Failed to compile post-process shader");
		effect.uniforms = self.uniforms;
		effect
	}
}

struct PingPongBuffer {
	framebuffers: [WebGlFramebuffer; 2],
	textures: [WebGlTexture; 2],
	current: usize,
}

impl PingPongBuffer {
	fn new(gl: &GL, width: i32, height: i32) -> Result<Self, String> {
		let mut framebuffers = Vec::with_capacity(2);
		let mut textures = Vec::with_capacity(2);

		for _ in 0..2 {
			let fb = gl.create_framebuffer()
				.ok_or("Failed to create ping-pong framebuffer")?;
			let tex = gl.create_texture()
				.ok_or("Failed to create ping-pong texture")?;

			gl.bind_texture(GL::TEXTURE_2D, Some(&tex));
			gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
				GL::TEXTURE_2D, 0, GL::RGBA as i32, width, height, 0,
				GL::RGBA, GL::UNSIGNED_BYTE, None,
			).map_err(|e| format!("Failed to create texture: {:?}", e))?;

			gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
			gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
			gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
			gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

			gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&fb));
			gl.framebuffer_texture_2d(
				GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::TEXTURE_2D, Some(&tex), 0,
			);

			framebuffers.push(fb);
			textures.push(tex);
		}

		gl.bind_framebuffer(GL::FRAMEBUFFER, None);

		Ok(Self {
			framebuffers: [framebuffers.remove(0), framebuffers.remove(0)],
			textures: [textures.remove(0), textures.remove(0)],
			current: 0,
		})
	}

	fn resize(&self, gl: &GL, width: i32, height: i32) {
		for tex in &self.textures {
			gl.bind_texture(GL::TEXTURE_2D, Some(tex));
			let _ = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
				GL::TEXTURE_2D, 0, GL::RGBA as i32, width, height, 0,
				GL::RGBA, GL::UNSIGNED_BYTE, None,
			);
		}
	}

	fn swap(&mut self) {
		self.current = 1 - self.current;
	}

	fn read_texture(&self) -> &WebGlTexture {
		&self.textures[self.current]
	}

	fn write_framebuffer(&self) -> &WebGlFramebuffer {
		&self.framebuffers[1 - self.current]
	}
}

pub struct PostProcessStack {
	scene_framebuffer: WebGlFramebuffer,
	scene_texture: WebGlTexture,
	depth_renderbuffer: WebGlRenderbuffer,
	ping_pong: PingPongBuffer,
	quad_buffer: WebGlBuffer,
	effects: Vec<PostProcessEffect>,
	width: i32,
	height: i32,
	pub enabled: bool,
}

impl PostProcessStack {
	pub fn new(gl: &GL, width: i32, height: i32) -> Result<Self, String> {
		let scene_framebuffer = gl.create_framebuffer()
			.ok_or("Failed to create scene framebuffer")?;
		let scene_texture = gl.create_texture()
			.ok_or("Failed to create scene texture")?;

		gl.bind_texture(GL::TEXTURE_2D, Some(&scene_texture));
		gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
			GL::TEXTURE_2D, 0, GL::RGBA as i32, width, height, 0,
			GL::RGBA, GL::UNSIGNED_BYTE, None,
		).map_err(|e| format!("Failed to create scene texture: {:?}", e))?;

		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

		let depth_renderbuffer = gl.create_renderbuffer()
			.ok_or("Failed to create depth renderbuffer")?;
		gl.bind_renderbuffer(GL::RENDERBUFFER, Some(&depth_renderbuffer));
		gl.renderbuffer_storage(GL::RENDERBUFFER, GL::DEPTH_COMPONENT24, width, height);

		gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&scene_framebuffer));
		gl.framebuffer_texture_2d(
			GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::TEXTURE_2D, Some(&scene_texture), 0,
		);
		gl.framebuffer_renderbuffer(
			GL::FRAMEBUFFER, GL::DEPTH_ATTACHMENT, GL::RENDERBUFFER, Some(&depth_renderbuffer),
		);

		let status = gl.check_framebuffer_status(GL::FRAMEBUFFER);
		if status != GL::FRAMEBUFFER_COMPLETE {
			return Err(format!("Scene framebuffer incomplete: {}", status));
		}

		gl.bind_framebuffer(GL::FRAMEBUFFER, None);

		let ping_pong = PingPongBuffer::new(gl, width, height)?;

		let quad_vertices: [f32; 24] = [
			-1.0, 1.0, 0.0, 1.0,
			-1.0, -1.0, 0.0, 0.0,
			1.0, -1.0, 1.0, 0.0,
			-1.0, 1.0, 0.0, 1.0,
			1.0, -1.0, 1.0, 0.0,
			1.0, 1.0, 1.0, 1.0,
		];

		let quad_buffer = gl.create_buffer()
			.ok_or("Failed to create quad buffer")?;
		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&quad_buffer));

		let vert_array = unsafe {
			std::slice::from_raw_parts(
				quad_vertices.as_ptr() as *const u8,
				quad_vertices.len() * std::mem::size_of::<f32>(),
			)
		};
		gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, vert_array, GL::STATIC_DRAW);

		Ok(Self {
			scene_framebuffer,
			scene_texture,
			depth_renderbuffer,
			ping_pong,
			quad_buffer,
			effects: Vec::new(),
			width,
			height,
			enabled: true,
		})
	}

	pub fn resize(&mut self, gl: &GL, width: i32, height: i32) {
		self.width = width;
		self.height = height;

		gl.bind_texture(GL::TEXTURE_2D, Some(&self.scene_texture));
		let _ = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
			GL::TEXTURE_2D, 0, GL::RGBA as i32, width, height, 0,
			GL::RGBA, GL::UNSIGNED_BYTE, None,
		);

		gl.bind_renderbuffer(GL::RENDERBUFFER, Some(&self.depth_renderbuffer));
		gl.renderbuffer_storage(GL::RENDERBUFFER, GL::DEPTH_COMPONENT24, width, height);

		self.ping_pong.resize(gl, width, height);
	}

	pub fn push(&mut self, effect: PostProcessEffect) -> usize {
		let index = self.effects.len();
		self.effects.push(effect);
		index
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut PostProcessEffect> {
		self.effects.get_mut(index)
	}

	pub fn remove(&mut self, index: usize) -> Option<PostProcessEffect> {
		if index < self.effects.len() {
			Some(self.effects.remove(index))
		} else {
			None
		}
	}

	pub fn clear(&mut self) {
		self.effects.clear();
	}

	pub fn begin(&self, gl: &GL) {
		if !self.enabled {
			return;
		}

		gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&self.scene_framebuffer));
		gl.viewport(0, 0, self.width, self.height);
		gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
	}

	pub fn end(&mut self, gl: &GL, time: f32) {
		if !self.enabled {
			return;
		}

		gl.disable(GL::DEPTH_TEST);

		let enabled_effects: Vec<usize> = self.effects
			.iter()
			.enumerate()
			.filter(|(_, e)| e.enabled)
			.map(|(i, _)| i)
			.collect();

		if enabled_effects.is_empty() {
			self.blit_to_screen(gl, &self.scene_texture);
		} else {
			self.blit_texture(gl, &self.scene_texture, self.ping_pong.write_framebuffer());
			self.ping_pong.swap();

			for (i, &effect_idx) in enabled_effects.iter().enumerate() {
				let is_last = i == enabled_effects.len() - 1;
				
				if is_last {
					gl.bind_framebuffer(GL::FRAMEBUFFER, None);
				} else {
					gl.bind_framebuffer(GL::FRAMEBUFFER, Some(self.ping_pong.write_framebuffer()));
				}
				
				gl.viewport(0, 0, self.width, self.height);
				gl.clear(GL::COLOR_BUFFER_BIT);

				let effect = &self.effects[effect_idx];
				self.apply_effect(gl, effect, self.ping_pong.read_texture(), time);

				if !is_last {
					self.ping_pong.swap();
				}
			}
		}

		gl.enable(GL::DEPTH_TEST);
	}

	fn apply_effect(&self, gl: &GL, effect: &PostProcessEffect, input_texture: &WebGlTexture, time: f32) {
		let program = effect.program();
		gl.use_program(Some(program));

		gl.active_texture(GL::TEXTURE0);
		gl.bind_texture(GL::TEXTURE_2D, Some(input_texture));

		if let Some(loc) = gl.get_uniform_location(program, "screenTexture") {
			gl.uniform1i(Some(&loc), 0);
		}
		if let Some(loc) = gl.get_uniform_location(program, "time") {
			gl.uniform1f(Some(&loc), time);
		}
		if let Some(loc) = gl.get_uniform_location(program, "resolution") {
			gl.uniform2f(Some(&loc), self.width as f32, self.height as f32);
		}

		effect.apply_uniforms(gl);

		self.draw_quad(gl, program);
	}

	fn blit_texture(&self, gl: &GL, texture: &WebGlTexture, target_fb: &WebGlFramebuffer) {
		gl.bind_framebuffer(GL::FRAMEBUFFER, Some(target_fb));
		gl.viewport(0, 0, self.width, self.height);

		gl.active_texture(GL::TEXTURE0);
		gl.bind_texture(GL::TEXTURE_2D, Some(texture));

		gl.bind_framebuffer(GL::READ_FRAMEBUFFER, Some(&self.scene_framebuffer));
		gl.bind_framebuffer(GL::DRAW_FRAMEBUFFER, Some(target_fb));
		gl.blit_framebuffer(
			0, 0, self.width, self.height,
			0, 0, self.width, self.height,
			GL::COLOR_BUFFER_BIT,
			GL::NEAREST,
		);
	}

	fn blit_to_screen(&self, gl: &GL, texture: &WebGlTexture) {
		gl.bind_framebuffer(GL::READ_FRAMEBUFFER, Some(&self.scene_framebuffer));
		gl.bind_framebuffer(GL::DRAW_FRAMEBUFFER, None);
		gl.blit_framebuffer(
			0, 0, self.width, self.height,
			0, 0, self.width, self.height,
			GL::COLOR_BUFFER_BIT,
			GL::NEAREST,
		);
	}

	fn draw_quad(&self, gl: &GL, program: &WebGlProgram) {
		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.quad_buffer));

		let pos_loc = gl.get_attrib_location(program, "position");
		let uv_loc = gl.get_attrib_location(program, "uv");

		if pos_loc >= 0 {
			gl.enable_vertex_attrib_array(pos_loc as u32);
			gl.vertex_attrib_pointer_with_i32(pos_loc as u32, 2, GL::FLOAT, false, 16, 0);
		}
		if uv_loc >= 0 {
			gl.enable_vertex_attrib_array(uv_loc as u32);
			gl.vertex_attrib_pointer_with_i32(uv_loc as u32, 2, GL::FLOAT, false, 16, 8);
		}

		gl.draw_arrays(GL::TRIANGLES, 0, 6);
	}
}


pub mod presets {
	use super::*;
	use web_sys::WebGl2RenderingContext as GL;

	const GRAYSCALE_FRAG: &str = include_str!("../pp_shaders/grayscale.frag");
	const VIGNETTE_FRAG: &str = include_str!("../pp_shaders/vignette.frag");
	const CHROMATIC_FRAG: &str = include_str!("../pp_shaders/chromatic.frag");
	const BLUR_FRAG: &str = include_str!("../pp_shaders/blur.frag");
	const INVERT_FRAG: &str = include_str!("../pp_shaders/invert.frag");
	const PIXELATE_FRAG: &str = include_str!("../pp_shaders/pixelate.frag");
	const FILM_GRAIN_FRAG: &str = include_str!("../pp_shaders/film_grain.frag");

	pub fn grayscale(gl: &GL) -> PostProcessEffect {
		PostProcessEffectBuilder::new(gl, GRAYSCALE_FRAG).build()
	}

	pub fn vignette(gl: &GL, intensity: f32, smoothness: f32) -> PostProcessEffect {
		PostProcessEffectBuilder::new(gl, VIGNETTE_FRAG)
			.float("intensity", intensity)
			.float("smoothness", smoothness)
			.build()
	}

	pub fn chromatic_aberration(gl: &GL, strength: f32) -> PostProcessEffect {
		PostProcessEffectBuilder::new(gl, CHROMATIC_FRAG)
			.float("strength", strength)
			.build()
	}

	pub fn blur(gl: &GL, radius: i32) -> PostProcessEffect {
		PostProcessEffectBuilder::new(gl, BLUR_FRAG)
			.int("radius", radius)
			.build()
	}

	pub fn invert(gl: &GL) -> PostProcessEffect {
		PostProcessEffectBuilder::new(gl, INVERT_FRAG).build()
	}

	pub fn pixelate(gl: &GL, pixel_size: f32) -> PostProcessEffect {
		PostProcessEffectBuilder::new(gl, PIXELATE_FRAG)
			.float("pixelSize", pixel_size)
			.build()
	}

	pub fn film_grain(gl: &GL, intensity: f32) -> PostProcessEffect {
		PostProcessEffectBuilder::new(gl, FILM_GRAIN_FRAG)
			.float("intensity", intensity)
			.build()
	}
}