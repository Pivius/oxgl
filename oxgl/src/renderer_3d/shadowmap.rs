//! Shadow Map Rendering
//!
//! Provides depth-based shadow mapping for directional, point, and spot lights.
//! Shadow maps are rendered from the light's perspective and sampled during
//! the main rendering pass to determine shadowed regions.
//!
//! ## Examples
//!
//! ```
//! use oxgl::renderer_3d::ShadowMap;
//! use glam::Vec3;
//!
//! let shadow_map = ShadowMap::new(&gl)?;
//!
//! // Configure for a directional light
//! shadow_map.update_directional(
//!		Vec3::new(-1.0, -1.0, -1.0),  // Light direction
//!		Vec3::ZERO,                    // Scene center
//!		10.0                           // Scene radius
//! );
//!
//! // Render shadow pass
//! shadow_map.bind(&gl);
//! // ... render objects with depth-only shader ...
//! shadow_map.unbind(&gl, canvas_width, canvas_height);
//! ```
//!

use glam::{Mat4, Vec3};
use web_sys::{
	WebGlFramebuffer, WebGlTexture,
	WebGl2RenderingContext as GL,
};

/// Default resolution of the shadow map texture.
///
/// Higher values produce sharper shadows but use more memory.
pub const SHADOW_MAP_SIZE: i32 = 1024;

/// A depth-based shadow map for shadow rendering.
///
/// Renders the scene from the light's perspective into a depth texture,
/// which is then sampled during the main pass to determine if fragments
/// are in shadow.
///
/// ## Usage
///
/// 1. Create the shadow map with [`new`](Self::new)
/// 2. Update light-space matrix with `update_*` methods
/// 3. Bind with [`bind`](Self::bind) and render depth pass
/// 4. Unbind with [`unbind`](Self::unbind)
/// 5. Bind texture with [`bind_texture`](Self::bind_texture) during main pass
///
pub struct ShadowMap {
	pub framebuffer: WebGlFramebuffer,
	pub depth_texture: WebGlTexture,
	pub light_space: Mat4,
	pub size: i32,
}

impl ShadowMap {
	/// Creates a new shadow map with the default resolution.
	///
	/// Allocates a framebuffer and depth texture for shadow rendering.
	/// The shadow map size is defined by [`SHADOW_MAP_SIZE`].
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - Framebuffer creation fails
	/// - Depth texture creation fails
	/// - Framebuffer is incomplete (driver/hardware limitation)
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::renderer_3d::ShadowMap;
	///
	/// let shadow_map = ShadowMap::new(&gl)?;
	/// println!("Shadow map size: {}x{}", shadow_map.size, shadow_map.size);
	/// ```
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

	/// Updates the light-space matrix for a directional light.
	///
	/// Directional lights use orthographic projection to simulate parallel
	/// light rays from an infinite distance. The projection bounds are
	/// determined by the scene radius.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // Sun shining from upper-right
	/// shadow_map.update_directional(
	///		Vec3::new(-1.0, -1.0, -0.5).normalize(),
	///		Vec3::ZERO,
	///		15.0
	/// );
	/// ```
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

	/// Updates the light-space matrix for a point or spot light.
	///
	/// Point and spot lights use perspective projection since they emanate
	/// from a single point. For point lights, this creates shadows for one
	/// face of a conceptual cube map.
	///
	/// # Examples
	///
	/// ```
	/// use glam::Vec3;
	///
	/// // Spot light aimed at origin
	/// shadow_map.update_point(
	///		Vec3::new(5.0, 5.0, 5.0),    // Position
	///		Vec3::ZERO,                  // Target
	///		std::f32::consts::FRAC_PI_4, // 45° FOV
	///		0.1,                         // Near
	///		25.0                         // Far (light range)
	/// );
	/// ```
	pub fn update_point(&mut self, position: Vec3, target: Vec3, fov: f32, near: f32, far: f32) {
		let view = Mat4::look_at_rh(position, target, Vec3::Y);
		let projection = Mat4::perspective_rh_gl(fov, 1.0, near, far);

		self.light_space = projection * view;
	}

	/// Binds the shadow map framebuffer for rendering.
	///
	/// After calling this, all draw calls will render to the shadow map's
	/// depth texture. The viewport is set to the shadow map size.
	///
	/// # Examples
	///
	/// ```
	/// shadow_map.bind(&gl);
	///
	/// gl.use_program(Some(&depth_shader));
	/// // Set light-space matrix uniform
	/// // Render all shadow-casting objects
	///
	/// shadow_map.unbind(&gl, canvas_width, canvas_height);
	/// ```
	pub fn bind(&self, gl: &GL) {
		gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&self.framebuffer));
		gl.viewport(0, 0, self.size, self.size);
		gl.clear(GL::DEPTH_BUFFER_BIT);
	}

	/// Unbinds the shadow map framebuffer.
	///
	/// Returns rendering to the default framebuffer (screen) and restores
	/// the viewport to the canvas size.
	///
	pub fn unbind(&self, gl: &GL, width: i32, height: i32) {
		gl.bind_framebuffer(GL::FRAMEBUFFER, None);
		gl.viewport(0, 0, width, height);
	}

	/// Binds the shadow depth texture for sampling.
	///
	/// Call this during the main rendering pass to make the shadow map
	/// available for shadow calculations in the fragment shader.
	///
	/// # Examples
	///
	/// ```
	/// // During main render pass
	/// shadow_map.bind_texture(&gl, 0);
	///
	/// // In shader setup
	/// gl.uniform1i(shadow_map_location, 0);
	/// ```
	pub fn bind_texture(&self, gl: &GL, unit: u32) {
		gl.active_texture(GL::TEXTURE0 + unit);
		gl.bind_texture(GL::TEXTURE_2D, Some(&self.depth_texture));
	}
}