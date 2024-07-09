use std::fmt::Display;

pub(crate) const BLUE: Color = Color(50, 50, 225);
pub(crate) const CYAN: Color = Color(100, 225, 225);
pub(crate) const GREEN: Color = Color(50, 225, 50);
pub(crate) const MAGENTA: Color = Color(225, 100, 225);
pub(crate) const ORANGE: Color = Color(255, 150, 50);
pub(crate) const RED: Color = Color(225, 50, 50);
pub(crate) const YELLOW: Color = Color(225, 225, 50);

/// 3x3(u8) + 2x1(";") = 11
pub(crate) const COLOR_STR_LENGTH: usize = 11;

/// RGB tuple struct
#[derive(Clone, Debug, PartialEq)]
pub struct Color(u8, u8, u8);

impl Color {
	/// Constructs a new `Color` with provided RGB arguments
	#[inline]
	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Self(r, g, b)
	}
}

impl Display for Color {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{};{};{}", self.0, self.1, self.2)
	}
}

impl From<(u8, u8, u8)> for Color {
	#[inline]
	fn from(tup: (u8, u8, u8)) -> Self {
		unsafe { std::mem::transmute(tup) }
	}
}

#[cfg(test)]
mod tests {
	use super::Color;

	#[test]
	fn from_impl() {
		let tup = (10, 20, 30);

		let color: Color = tup.into();

		assert_eq!(tup.0, color.0);
		assert_eq!(tup.1, color.1);
		assert_eq!(tup.2, color.2);
	}
}
