pub enum Primitive {
	Quad,
	Triangle,

}

impl Primitive {
	pub fn vertices(&self) -> Vec<f32> {
		match self {
			Primitive::Quad => vec![
				-0.5,  0.5, 0.0,
				-0.5, -0.5, 0.0,
				0.5, -0.5, 0.0,
				-0.5,  0.5, 0.0,
				0.5, -0.5, 0.0,
				0.5,  0.5, 0.0,
			],
			Primitive::Triangle => vec![
				0.0,  0.5, 0.0,
				-0.5, -0.5, 0.0,
				0.5, -0.5, 0.0,
			],
		}
	}
}