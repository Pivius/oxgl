use glam::{Mat4, Quat, Vec2, Vec3};

/// # Transformable Trait
/// A trait for objects that can be transformed in space.
/// 
/// ## Type Parameters
/// - `V`: The vector type for position.
/// - `M`: The matrix type for transformation.
/// 
/// ## Methods
/// - `position(&self) -> V`: Get the position of the object.
/// - `set_position(&mut self, pos: V)`: Set the position of the object.
/// - `to_matrix(&self) -> M`: Convert the transform to a transformation matrix.
pub trait Transformable<V, M> {
	fn position(&self) -> V;
	fn set_position(&mut self, pos: V);
	fn to_matrix(&self) -> M;
}

/// # 3D Transform
/// A 3D transform struct with position, rotation, and scale.
/// 
/// # Fields
/// - `position`: The position of the object in 3D space.
/// - `rotation`: The rotation of the object as a quaternion.
/// - `scale`: The scale of the object in 3D space.
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
	fn to_matrix(&self) -> Mat4 {
		Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
	}
}

/// # 2D Transform
/// A 2D transform struct with position, rotation, and scale.
/// 
/// ## Fields
/// - `position`: The position of the object in 2D space.
/// - `rotation`: The rotation of the object in radians.
/// - `scale`: The scale of the object in 2D space. 
#[derive(Clone, Debug, Default)]
pub struct Transform2D {
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
}

impl Transform2D {
	pub fn new() -> Self {
		Self {
			position: Vec2::ZERO,
			rotation: 0.0,
			scale: Vec2::ONE,
		}
	}
}