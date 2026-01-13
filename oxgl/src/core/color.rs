/// # Color
/// RGBA color representation
pub enum Color {
	Rgba(u8, u8, u8, u8),
}

impl Color {
	pub fn to_hex_string(&self) -> String {
		match *self {
			Color::Rgba(r, g, b, a) => {
				format!(
					"#{:02X}{:02X}{:02X}{:02X}",
					r, g, b, a
				)
			}
		}
	}

	pub fn from_hex_string(hex: &str) -> Option<Self> {
		let hex = hex.trim_start_matches('#');
		if hex.len() != 8 {
			return None;
		}
		let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
		let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
		let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
		let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
		Some(Color::Rgba(r, g, b, a))
	}

	pub fn to_hsv(&self) -> (f32, f32, f32, u8) {
		let Color::Rgba(r, g, b, a) = *self;
		let r = r as f32 / 255.0;
		let g = g as f32 / 255.0;
		let b = b as f32 / 255.0;

		let max = r.max(g).max(b);
		let min = r.min(g).min(b);
		let delta = max - min;

		let h = if delta == 0.0 {
			0.0
		} else if max == r {
			60.0 * (((g - b) / delta) % 6.0)
		} else if max == g {
			60.0 * (((b - r) / delta) + 2.0)
		} else {
			60.0 * (((r - g) / delta) + 4.0)
		};

		let s = if max == 0.0 { 0.0 } else { delta / max };
		let v = max;

		(h, s, v, a)
	}

	pub fn from_hsv(h: f32, s: f32, v: f32, a: u8) -> Self {
		let c = v * s;
		let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
		let m = v - c;

		let (r, g, b) = if h < 60.0 {
			(c, x, 0.0)
		} else if h < 120.0 {
			(x, c, 0.0)
		} else if h < 180.0 {
			(0.0, c, x)
		} else if h < 240.0 {
			(0.0, x, c)
		} else if h < 300.0 {
			(x, 0.0, c)
		} else {
			(c, 0.0, x)
		};

		let r = ((r + m) * 255.0) as u8;
		let g = ((g + m) * 255.0) as u8;
		let b = ((b + m) * 255.0) as u8;

		Color::Rgba(r, g, b, a)
	}

	pub fn lighten(&self, amount: f32) -> Self {
		let (h, s, v, a) = self.to_hsv();
		Self::from_hsv(h, s, (v + amount).clamp(0.0, 1.0), a)
	}

	pub fn darken(&self, amount: f32) -> Self {
		self.lighten(-amount)
	}

	pub fn saturate(&self, amount: f32) -> Self {
		let (h, s, v, a) = self.to_hsv();
		Self::from_hsv(h, (s + amount).clamp(0.0, 1.0), v, a)
	}

	pub fn desaturate(&self, amount: f32) -> Self {
		self.saturate(-amount)
	}
}