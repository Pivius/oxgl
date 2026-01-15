//! Color Utilities
//!
//! Provides color conversion and manipulation utilities for working with colors
//! in various formats including RGBA, RGB, HSVA, and HSV.
//!
//! ## Examples
//!
//! ```ignore
//! use oxgl::core::Color;
//!
//! // Create from hex
//! let color = Color::from_hex("#FF5500FF").unwrap();
//!
//! // Convert between formats
//! let hsv = color.to_hsva();
//! let back_to_rgba = hsv.to_rgba();
//!
//! // Manipulate colors
//! let lighter = color.lighten(0.2);
//! let saturated = color.saturate(0.3);
//! ```
//!

use glam::{Vec3, Vec4};

/// Color representation in various formats.
///
/// All conversions normalize through RGBA internally for consistency.
///
/// ## Variants
///
/// - `Rgba` - Red, Green, Blue, Alpha (0-255 each)
/// - `Rgb` - Red, Green, Blue (0-255 each), alpha assumed 255
/// - `Hsva` - Hue (0-360), Saturation (0-1), Value (0-1), Alpha (0-255)
/// - `Hsv` - Hue (0-360), Saturation (0-1), Value (0-1), alpha assumed 255
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
	/// RGBA color with components in range 0-255.
	Rgba(u8, u8, u8, u8),
	/// RGB color with components in range 0-255.
	Rgb(u8, u8, u8),
	/// HSVA color: Hue (0-360), Saturation (0-1), Value (0-1), Alpha (0-255).
	Hsva(f32, f32, f32, u8),
	/// HSV color: Hue (0-360), Saturation (0-1), Value (0-1).
	Hsv(f32, f32, f32),
}

impl Color {
	pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Color::Rgba(r, g, b, a)
	}

	pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
		Color::Rgb(r, g, b)
	}

	pub const fn hsva(h: f32, s: f32, v: f32, a: u8) -> Self {
		Color::Hsva(h, s, v, a)
	}

	pub const fn hsv(h: f32, s: f32, v: f32) -> Self {
		Color::Hsv(h, s, v)
	}

	/// Converts any color variant to RGBA.
	///
	/// This is the canonical conversion, all other conversions go through RGBA.
	///

	pub fn to_rgba(&self) -> Self {
		match *self {
			Color::Rgba(r, g, b, a) => Color::Rgba(r, g, b, a),
			Color::Rgb(r, g, b) => Color::Rgba(r, g, b, 255),
			Color::Hsva(h, s, v, a) => Self::hsv_to_rgba(h, s, v, a),
			Color::Hsv(h, s, v) => Self::hsv_to_rgba(h, s, v, 255),
		}
	}

	pub fn to_rgb(&self) -> Self {
		let Color::Rgba(r, g, b, _) = self.to_rgba() else { unreachable!() };
		Color::Rgb(r, g, b)
	}

	pub fn to_rgba_tuple(&self) -> (u8, u8, u8, u8) {
		let Color::Rgba(r, g, b, a) = self.to_rgba() else { unreachable!() };
		(r, g, b, a)
	}

	pub fn to_hsva(&self) -> Self {
		match *self {
			Color::Hsva(h, s, v, a) => Color::Hsva(h, s, v, a),
			Color::Hsv(h, s, v) => Color::Hsva(h, s, v, 255),
			_ => {
				let (r, g, b, a) = self.to_rgba_tuple();
				Self::rgba_to_hsva(r, g, b, a)
			}
		}
	}

	pub fn to_hsv(&self) -> Self {
		let Color::Hsva(h, s, v, _) = self.to_hsva() else { unreachable!() };
		Color::Hsv(h, s, v)
	}

	pub fn to_hsva_tuple(&self) -> (f32, f32, f32, u8) {
		let Color::Hsva(h, s, v, a) = self.to_hsva() else { unreachable!() };
		(h, s, v, a)
	}

	/// Converts the color to a hex string.
	///
	/// Always outputs in `#RRGGBBAA` format regardless of input.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::core::Color;
	///
	/// let red = Color::rgb(255, 0, 0);
	/// assert_eq!(red.to_hex(), "#FF0000FF");
	///
	/// let semi_transparent = Color::rgba(0, 255, 0, 128);
	/// assert_eq!(semi_transparent.to_hex(), "#00FF0080");
	/// ```
	pub fn to_hex(&self) -> String {
		let (r, g, b, a) = self.to_rgba_tuple();
		format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
	}

	/// Converts the color to a hex string without alpha.
	pub fn to_hex_rgb(&self) -> String {
		let (r, g, b, _) = self.to_rgba_tuple();
		format!("#{:02X}{:02X}{:02X}", r, g, b)
	}

	/// Creates a color from a hex string.
	///
	/// Supports the following formats:
	/// - `#RRGGBBAA` (8 chars)
	/// - `#RRGGBB` (6 chars, alpha defaults to 255)
	/// - `#RGBA` (4 chars, each char doubled)
	/// - `#RGB` (3 chars, each char doubled, alpha defaults to 255)
	///
	/// The leading `#` is optional.
	///
	/// # Errors
	///
	/// Returns `None` if the hex string is invalid or has an unsupported length.
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::core::Color;
	///
	/// let color = Color::from_hex("#FF5500").unwrap();
	/// let with_alpha = Color::from_hex("#FF550080").unwrap();
	/// let short = Color::from_hex("#F50").unwrap(); // Same as #FF5500
	/// ```
	pub fn from_hex(hex: &str) -> Option<Self> {
		let hex = hex.trim_start_matches('#');

		match hex.len() {
			// #RRGGBBAA
			8 => {
				let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
				let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
				let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
				let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
				Some(Color::Rgba(r, g, b, a))
			}
			// #RRGGBB
			6 => {
				let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
				let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
				let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
				Some(Color::Rgba(r, g, b, 255))
			}
			// #RGBA
			4 => {
				let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
				let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
				let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
				let a = u8::from_str_radix(&hex[3..4], 16).ok()? * 17;
				Some(Color::Rgba(r, g, b, a))
			}
			// #RGB
			3 => {
				let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
				let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
				let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
				Some(Color::Rgba(r, g, b, 255))
			}
			_ => None,
		}
	}

	/// Converts to a [`Vec3`] with normalized RGB values (0.0-1.0).
	pub fn to_vec3(&self) -> Vec3 {
		let (r, g, b, _) = self.to_rgba_tuple();
		Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
	}

	/// Converts to a [`Vec4`] with normalized RGBA values (0.0-1.0).
	pub fn to_vec4(&self) -> Vec4 {
		let (r, g, b, a) = self.to_rgba_tuple();
		Vec4::new(
			r as f32 / 255.0,
			g as f32 / 255.0,
			b as f32 / 255.0,
			a as f32 / 255.0,
		)
	}

	/// Creates a color from a [`Vec3`] with normalized RGB values (0.0-1.0).
	pub fn from_vec3(v: Vec3) -> Self {
		Color::Rgba(
			(v.x.clamp(0.0, 1.0) * 255.0) as u8,
			(v.y.clamp(0.0, 1.0) * 255.0) as u8,
			(v.z.clamp(0.0, 1.0) * 255.0) as u8,
			255,
		)
	}

	/// Creates a color from a [`Vec4`] with normalized RGBA values (0.0-1.0).
	pub fn from_vec4(v: Vec4) -> Self {
		Color::Rgba(
			(v.x.clamp(0.0, 1.0) * 255.0) as u8,
			(v.y.clamp(0.0, 1.0) * 255.0) as u8,
			(v.z.clamp(0.0, 1.0) * 255.0) as u8,
			(v.w.clamp(0.0, 1.0) * 255.0) as u8,
		)
	}

	pub fn lighten(&self, amount: f32) -> Self {
		let (h, s, v, a) = self.to_hsva_tuple();
		Self::hsv_to_rgba(h, s, (v + amount).clamp(0.0, 1.0), a)
	}

	pub fn darken(&self, amount: f32) -> Self {
		self.lighten(-amount)
	}

	pub fn saturate(&self, amount: f32) -> Self {
		let (h, s, v, a) = self.to_hsva_tuple();
		Self::hsv_to_rgba(h, (s + amount).clamp(0.0, 1.0), v, a)
	}

	pub fn desaturate(&self, amount: f32) -> Self {
		self.saturate(-amount)
	}

	/// Rotates the hue by the specified degrees.
	///
	/// # Arguments
	///
	/// * `degrees` - Degrees to rotate (can be negative)
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::core::Color;
	///
	/// let red = Color::rgb(255, 0, 0);
	/// let green = red.rotate_hue(120.0);  // Shift to green
	/// let blue = red.rotate_hue(240.0);   // Shift to blue
	/// ```
	pub fn rotate_hue(&self, degrees: f32) -> Self {
		let (h, s, v, a) = self.to_hsva_tuple();
		let new_h = (h + degrees).rem_euclid(360.0);
		Self::hsv_to_rgba(new_h, s, v, a)
	}

	pub fn complement(&self) -> Self {
		self.rotate_hue(180.0)
	}

	pub fn with_alpha(&self, alpha: u8) -> Self {
		let (r, g, b, _) = self.to_rgba_tuple();
		Color::Rgba(r, g, b, alpha)
	}

	/// Linearly interpolates between two colors.
	///
	/// # Arguments
	///
	/// * `other` - Target
	/// * `t` - Factor
	///
	/// # Examples
	///
	/// ```
	/// use oxgl::core::Color;
	///
	/// let red = Color::rgb(255, 0, 0);
	/// let blue = Color::rgb(0, 0, 255);
	/// let purple = red.lerp(&blue, 0.5);
	/// ```
	pub fn lerp(&self, other: &Self, t: f32) -> Self {
		let t = t.clamp(0.0, 1.0);
		let (r1, g1, b1, a1) = self.to_rgba_tuple();
		let (r2, g2, b2, a2) = other.to_rgba_tuple();

		Color::Rgba(
			((r1 as f32) + (r2 as f32 - r1 as f32) * t) as u8,
			((g1 as f32) + (g2 as f32 - g1 as f32) * t) as u8,
			((b1 as f32) + (b2 as f32 - b1 as f32) * t) as u8,
			((a1 as f32) + (a2 as f32 - a1 as f32) * t) as u8,
		)
	}

	fn hsv_to_rgba(h: f32, s: f32, v: f32, a: u8) -> Self {
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

		Color::Rgba(
			((r + m) * 255.0) as u8,
			((g + m) * 255.0) as u8,
			((b + m) * 255.0) as u8,
			a,
		)
	}

	fn rgba_to_hsva(r: u8, g: u8, b: u8, a: u8) -> Self {
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

		let h = if h < 0.0 { h + 360.0 } else { h };
		let s = if max == 0.0 { 0.0 } else { delta / max };
		let v = max;

		Color::Hsva(h, s, v, a)
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// Common Color Constants
// ─────────────────────────────────────────────────────────────────────────────

impl Color {
	pub const WHITE: Color = Color::Rgba(255, 255, 255, 255);
	pub const BLACK: Color = Color::Rgba(0, 0, 0, 255);
	pub const RED: Color = Color::Rgba(255, 0, 0, 255);
	pub const GREEN: Color = Color::Rgba(0, 255, 0, 255);
	pub const BLUE: Color = Color::Rgba(0, 0, 255, 255);
	pub const YELLOW: Color = Color::Rgba(255, 255, 0, 255);
	pub const CYAN: Color = Color::Rgba(0, 255, 255, 255);
	pub const MAGENTA: Color = Color::Rgba(255, 0, 255, 255);
	pub const TRANSPARENT: Color = Color::Rgba(0, 0, 0, 0);
}