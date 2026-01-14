//! Camera Types
//!
//! Provides perspective camera implementation for 3D rendering.
//!

use glam::{Mat4, Vec3};

/// A perspective camera for 3D scene viewing.
///
/// Generates view and projection matrices for rendering.
///
/// ## Examples
///
/// ```ignore
/// let camera = Camera::new(16.0 / 9.0)
/// 	.with_position(Vec3::new(0.0, 1.0, 5.0))
/// 	.with_target(Vec3::ZERO);
///
/// let view_matrix = camera.view_matrix();
/// let proj_matrix = camera.projection_matrix();
/// ```
#[derive(Debug, Clone)]
pub struct Camera {
	pub position: Vec3,
	pub target: Vec3,
	pub up: Vec3,
	pub fov_y: f32,
	pub aspect: f32,
	pub near: f32,
	pub far: f32,
}

impl Camera {
	pub fn new(aspect: f32) -> Self {
		Self {
			position: Vec3::new(0.0, 0.0, 3.0),
			target: Vec3::ZERO,
			up: Vec3::Y,
			fov_y: std::f32::consts::FRAC_PI_4,
			aspect,
			near: 0.1,
			far: 100.0,
		}
	}

	pub fn with_position(mut self, pos: Vec3) -> Self {
		self.position = pos;
		self
	}

	pub fn with_target(mut self, target: Vec3) -> Self {
		self.target = target;
		self
	}

	/// Returns the view matrix (world to camera space).
	pub fn view_matrix(&self) -> Mat4 {
		Mat4::look_at_rh(self.position, self.target, self.up)
	}

	/// Returns the projection matrix (camera to clip space).
	pub fn projection_matrix(&self) -> Mat4 {
		Mat4::perspective_rh_gl(self.fov_y, self.aspect, self.near, self.far)
	}
}