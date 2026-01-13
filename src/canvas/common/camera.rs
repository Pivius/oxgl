use glam::{Mat4, Vec3};

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

	pub fn view_matrix(&self) -> Mat4 {
		Mat4::look_at_rh(self.position, self.target, self.up)
	}

	pub fn projection_matrix(&self) -> Mat4 {
		Mat4::perspective_rh_gl(self.fov_y, self.aspect, self.near, self.far)
	}
}