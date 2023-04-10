use std::fmt::Display;

pub(crate) const BLUE: Color = Color(50, 50, 225);
pub(crate) const CYAN: Color = Color(100, 225, 225);
pub(crate) const GREEN: Color = Color(50, 225, 50);
pub(crate) const MAGENTA: Color = Color(225, 100, 225);
pub(crate) const ORANGE: Color = Color(225, 100, 100);
pub(crate) const RED: Color = Color(225, 50, 50);
pub(crate) const YELLOW: Color = Color(225, 225, 50);

/// 3x3(u8) + 2x1(";") = 11
pub(crate) const COLOR_STR_LENGTH: usize = 11;

#[derive(Clone, Debug, PartialEq)]
pub struct Color(u8, u8, u8);

impl Color {
	#[inline]
	fn to_string(&self) -> String {
		let mut buf = String::with_capacity(COLOR_STR_LENGTH);

		let mut formatter = core::fmt::Formatter::new(&mut buf);
		std::fmt::Display::fmt(self, &mut formatter)
			.expect("a Display implementation returned an error unexpectedly");

		buf
	}
}

impl Display for Color {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{};{};{}", self.0, self.1, self.2)
	}
}

impl From<(u8, u8, u8)> for Color {
	#[inline]
	fn from((red, green, blue): (u8, u8, u8)) -> Self {
		Self(red, green, blue)
	}
}

#[cfg(test)]
mod test {
	extern crate test;

	use test::Bencher;

	use super::Color;

	#[bench]
	fn to_string(b: &mut Bencher) {
		b.iter(|| Color(125, 125, 125).to_string())
	}
}
