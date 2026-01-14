use glam::{Mat4, Vec3};
use web_sys::{
	WebGlFramebuffer, WebGlTexture,
	WebGl2RenderingContext as GL,
};

pub const SHADOW_MAP_SIZE: i32 = 1024;

pub struct ShadowMap {
	pub framebuffer: WebGlFramebuffer,
	pub depth_texture: WebGlTexture,
	pub light_space: Mat4,
	pub size: i32,
}

impl ShadowMap {
	pub fn new(gl: &GL) -> Result<Self, String> {
		let size = SHADOW_MAP_SIZE;

		let framebuffer = gl
			.create_framebuffer()
			.ok_or("Failed to create shadow framebuffer")?;

		let depth_texture = gl
			.create_texture()
			.ok_or("Failed to create shadow texture")?;

		gl.bind_texture(GL::TEXTURE_2D, Some(&depth_texture));
		
		gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
			GL::TEXTURE_2D,
			0,
			GL::DEPTH_COMPONENT24 as i32,
			size,
			size,
			0,
			GL::DEPTH_COMPONENT,
			GL::UNSIGNED_INT,
			None,
		).map_err(|e| format!("Failed to create depth texture: {:?}", e))?;

		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
		gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

		gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&framebuffer));
		gl.framebuffer_texture_2d(
			GL::FRAMEBUFFER,
			GL::DEPTH_ATTACHMENT,
			GL::TEXTURE_2D,
			Some(&depth_texture),
			0,
		);

		gl.draw_buffers(&js_sys::Array::new());

		let status = gl.check_framebuffer_status(GL::FRAMEBUFFER);

		if status != GL::FRAMEBUFFER_COMPLETE {
			let error_msg = match status {
				GL::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => "INCOMPLETE_ATTACHMENT",
				GL::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => "MISSING_ATTACHMENT",
				GL::FRAMEBUFFER_INCOMPLETE_DIMENSIONS => "INCOMPLETE_DIMENSIONS",
				GL::FRAMEBUFFER_UNSUPPORTED => "UNSUPPORTED",
				_ => "UNKNOWN",
			};
			return Err(format!("Shadow framebuffer incomplete: {} ({})", error_msg, status));
		}

		gl.bind_framebuffer(GL::FRAMEBUFFER, None);
		gl.bind_texture(GL::TEXTURE_2D, None);

		Ok(Self {
			framebuffer,
			depth_texture,
			light_space: Mat4::IDENTITY,
			size,
		})
	}

	pub fn update_directional(&mut self, direction: Vec3, scene_center: Vec3, scene_radius: f32) {
		let light_pos = scene_center - direction.normalize() * scene_radius * 2.0;
		
		let view = Mat4::look_at_rh(light_pos, scene_center, Vec3::Y);
		let projection = Mat4::orthographic_rh_gl(
			-scene_radius, scene_radius,
			-scene_radius, scene_radius,
			0.1, scene_radius * 4.0,
		);

		self.light_space = projection * view;
	}

	pub fn update_point(&mut self, position: Vec3, target: Vec3, fov: f32, near: f32, far: f32) {
		let view = Mat4::look_at_rh(position, target, Vec3::Y);
		let projection = Mat4::perspective_rh_gl(fov, 1.0, near, far);

		self.light_space = projection * view;
	}

	pub fn bind(&self, gl: &GL) {
		gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&self.framebuffer));
		gl.viewport(0, 0, self.size, self.size);
		gl.clear(GL::DEPTH_BUFFER_BIT);
	}

	pub fn unbind(&self, gl: &GL, width: i32, height: i32) {
		gl.bind_framebuffer(GL::FRAMEBUFFER, None);
		gl.viewport(0, 0, width, height);
	}

	pub fn bind_texture(&self, gl: &GL, unit: u32) {
		gl.active_texture(GL::TEXTURE0 + unit);
		gl.bind_texture(GL::TEXTURE_2D, Some(&self.depth_texture));
	}
}