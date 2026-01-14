//! 3D Transform Types
//!
//! Provides position, rotation, and scale transformations for 3D objects.
//!

use glam::{Mat4, Quat, Vec2, Vec3};

/// Trait for types that can be transformed in 3D space.
pub trait Transformable<V, M> {
	fn position(&self) -> V;
	fn set_position(&mut self, pos: V);
	/// Converts the transform to a 4x4 transformation matrix.
	fn to_matrix(&self) -> M;
}

/// A 3D transformation consisting of position, rotation, and scale.
///
/// Uses a builder pattern for easy construction with method chaining.
///
/// ## Examples
///
/// ```ignore
/// use oxgl::core::Transform3D;
/// use glam::{Vec3, Quat};
///
/// let transform = Transform3D::new()
///		 .with_position(Vec3::new(1.0, 2.0, 3.0))
///		 .with_rotation(Quat::from_rotation_y(std::f32::consts::PI))
///		 .with_scale(Vec3::splat(2.0));
/// ```
#[derive(Clone, Debug, Default)]
pub struct Transform3D {
	pub position: Vec3,
	pub rotation: Quat,
	pub scale: Vec3,
}

impl Transform3D {
	pub fn new() -> Self {
		Self {
			position: Vec3::ZERO,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
		}
	}

	pub fn with_position(mut self, pos: Vec3) -> Self {
		self.position = pos;
		self
	}

	pub fn with_rotation(mut self, rot: Quat) -> Self {
		self.rotation = rot;
		self
	}

	pub fn with_scale(mut self, scale: Vec3) -> Self {
		self.scale = scale;
		self
	}
}

impl Transformable<Vec3, Mat4> for Transform3D {
	fn position(&self) -> Vec3 { self.position }
	fn set_position(&mut self, pos: Vec3) { self.position = pos; }
	/// Converts to a 4x4 matrix in TRS (translate * rotate * scale) order.
	fn to_matrix(&self) -> Mat4 {
		Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
	}
}