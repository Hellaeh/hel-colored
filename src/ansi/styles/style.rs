use std::fmt::Display;

use super::BitFlag;

/// 8x1 + 7x1(";") = 15
pub(crate) const STYLE_STR_LENGTH: usize = 15;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Style(u8);

impl Style {
	#[inline]
	pub(crate) fn enable(&mut self, style: BitFlag) {
		self.0 |= style
	}

	#[inline]
	fn styles(&self) -> impl Iterator<Item = &str> {
		BitFlag::into_iter()
			.filter(|&bit| self.0 & bit == bit)
			.map(|value| value.into())
	}
}

impl Display for Style {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut styles = self.styles();

		if let Some(style) = styles.next() {
			f.write_str(style)?;
		}

		for style in styles {
			f.write_str(";")?;
			f.write_str(style)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	extern crate test;

	use test::Bencher;

	use super::Style;

	#[bench]
	fn styles_iter(b: &mut Bencher) {
		b.iter(|| Style(255).styles().count())
	}
}
