use std::fmt::{Debug, Display};

use crate::{Colored, Styled};

use super::{colors::Color, consts::*, BitFlag, Style, COLOR_STR_LENGTH, STYLE_STR_LENGTH};

#[cfg(feature = "nested")]
use super::utils::UnsafeBytes;

/// A struct that owns `T`, also holds ANSI styles and colors
#[derive(PartialEq, Clone, Debug)]
pub struct ANSIString<T> {
	pub(crate) inner: T,
	pub(crate) fg: Option<Color>,
	pub(crate) bg: Option<Color>,
	pub(crate) style: Option<Style>,
}

impl<T> ANSIString<T> {
	pub(crate) fn new(str: T) -> Self {
		Self {
			inner: str,
			fg: None,
			bg: None,
			style: None,
		}
	}

	/// Returns ownership of `T`, while consuming `self`
	pub fn clear(self) -> T {
		self.inner
	}

	/// `to_string` method implemented directly on struct
	/// for performance reasons
	#[inline]
	pub fn to_string(self) -> String
	where
		T: AsRef<str>,
	{
		const NESTED: usize = if cfg!(feature = "nested") {
			30
		} else {
			SGR_RESET_SEQ_STR.len()
		};

		/// 2x11(color) + 15(style) + 20(for escape bytes and reset seq)
		const ANSI_STRING_PADDING: usize = 2 * COLOR_STR_LENGTH + STYLE_STR_LENGTH + NESTED;

		let mut buf = String::with_capacity(self.inner.as_ref().len() + ANSI_STRING_PADDING);

		let mut formatter = core::fmt::Formatter::new(&mut buf);
		std::fmt::Display::fmt(&self, &mut formatter)
			.expect("Display implementation returned an error unexpectedly");

		buf
	}

	/// Returns `true` if foreground (letters) of a string is colored
	#[inline]
	pub fn is_foreground_colored(&self) -> bool {
		self.fg.is_some()
	}

	/// Return `true` if background (bounding box) of a string is colored
	#[inline]
	pub fn is_background_colored(&self) -> bool {
		self.bg.is_some()
	}

	/// Returns `true` if both background and foreground of a string is colored
	#[inline]
	pub fn is_colored(&self) -> bool {
		self.is_foreground_colored() || self.is_background_colored()
	}

	/// Return `true` if string is styled, e.g. `bold` or `italic`
	#[inline]
	pub fn is_styled(&self) -> bool {
		self.style.is_some()
	}

	#[inline]
	pub(crate) fn get_or_init_style(&mut self) -> &mut Style {
		self.style.get_or_insert(Style::default())
	}

	#[inline]
	fn fmt_sgr(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// [0; to reset each nested part of string
		f.write_str("\x1b[0;")?;

		let mut wrote = false;
		if let Some(fg) = &self.fg {
			write!(f, "38;2;{fg}")?;
			wrote = true
		}

		if let Some(bg) = &self.bg {
			if wrote {
				f.write_str(";")?;
			}

			write!(f, "48;2;{bg}")?;

			wrote = true
		}

		if let Some(style) = &self.style {
			if wrote {
				f.write_str(";")?;
			}

			write!(f, "{style}")?;
		}

		f.write_str("m")?;

		Ok(())
	}

	#[inline]
	#[cfg(feature = "nested")]
	fn fmt_inner(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	where
		T: AsRef<str>,
	{
		let str = self.inner.as_ref();
		let bytes = str.as_bytes();

		let mut prev = 0;

		// TODO work directly with bytes
		for (i, _) in str.match_indices(SGR_RESET_SEQ_STR) {
			bytes.fmt_str_from(prev..i, f)?;

			ANSIString::fmt_sgr(self, f)?;

			const OFFSET: usize = SGR_RESET_SEQ_STR.len();
			prev = i + OFFSET;
		}

		if prev > 0 {
			bytes.fmt_str_from(prev.., f)
		} else {
			std::fmt::Display::fmt(str, f)
		}
	}

	#[inline(always)]
	#[cfg(not(feature = "nested"))]
	fn fmt_inner(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	where
		T: AsRef<str>,
	{
		std::fmt::Display::fmt(self.inner.as_ref(), f)
	}
}

impl<T: AsRef<str>> Display for ANSIString<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		ANSIString::fmt_sgr(self, f)?;
		ANSIString::fmt_inner(self, f)?;

		f.write_str(SGR_RESET_SEQ_STR)?;

		Ok(())
	}
}

impl<T: AsRef<str>> From<ANSIString<T>> for String {
	fn from(value: ANSIString<T>) -> Self {
		ANSIString::to_string(value)
	}
}

impl<T: AsRef<str>> Styled for T {
	type Output = ANSIString<T>;

	#[inline]
	fn set(self, style: BitFlag) -> Self::Output {
		Self::Output::new(self).set(style)
	}
}

impl<T> Styled for ANSIString<T> {
	type Output = Self;

	#[inline]
	fn set(mut self, style: BitFlag) -> Self::Output {
		self.get_or_init_style().enable(style);
		self
	}
}

impl<T> Styled for &mut ANSIString<T> {
	type Output = ();

	#[inline]
	fn set(self, style: BitFlag) -> Self::Output {
		self.get_or_init_style().enable(style)
	}
}

impl<T: AsRef<str>> Colored for T {
	type Output = ANSIString<T>;

	#[inline]
	fn colorize_fg(self, color: Color) -> Self::Output {
		Self::Output::new(self).colorize_fg(color)
	}
	#[inline]
	fn colorize_bg(self, color: Color) -> Self::Output {
		Self::Output::new(self).colorize_bg(color)
	}
}

impl<T> Colored for ANSIString<T> {
	type Output = Self;

	#[inline]
	fn colorize_fg(mut self, color: Color) -> Self::Output {
		self.fg = Some(color);
		self
	}
	#[inline]
	fn colorize_bg(mut self, color: Color) -> Self::Output {
		self.bg = Some(color);
		self
	}
}

impl<T> Colored for &mut ANSIString<T> {
	type Output = ();

	#[inline]
	fn colorize_fg(self, color: Color) -> Self::Output {
		self.fg = Some(color)
	}
	#[inline]
	fn colorize_bg(self, color: Color) -> Self::Output {
		self.bg = Some(color)
	}
}

#[cfg(test)]
mod test {
	extern crate test;

	use test::Bencher;

	use crate::Styled;

	// const SMALL_STR: &str = "Hello 12345";

	const BIG_STR: &str = "Lorem ipsum dolor sit amet, \
	consectetur adipiscing elit, sed do eiusmod tempor \
	incididunt ut labore et dolore magna aliqua. Ut enim \
	ad minim veniam, quis nostrud exercitation ullamco laboris \
	nisi ut aliquip ex ea commodo consequat. Duis aute irure \
	dolor in reprehenderit in voluptate velit esse cillum dolore \
	eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat \
	non proident, sunt in culpa qui officia deserunt mollit anim \
	id est laborum.";

	#[bench]
	fn custom_to_string(b: &mut Bencher) {
		b.iter(|| BIG_STR.bold().to_string())
	}

	#[bench]
	fn fmt_to_string(b: &mut Bencher) {
		b.iter(|| ToString::to_string(&BIG_STR.bold()))
	}

	#[bench]
	fn format(b: &mut Bencher) {
		b.iter(|| format!("{}", BIG_STR.bold()))
	}

	#[bench]
	fn string_from(b: &mut Bencher) {
		b.iter(|| String::from(BIG_STR.bold()))
	}
}
