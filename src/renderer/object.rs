use super::{camera::Camera, mesh::Mesh, scene::Transform, util::Color};

pub enum ObjectKind {
	Mesh(Mesh),
	Light(Light),
	Camera(Camera),
}

pub struct Light {
	pub color: Color,
	pub intensity: f32,
}

pub struct Object {
	pub kind: ObjectKind,
	pub transform: Transform,
}