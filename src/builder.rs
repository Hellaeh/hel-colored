use crate::{
	ansi::{BitFlag, Color, Style},
	ANSIString, Colored, Styled,
};

/// A helper factory struct for an easy way to produce
/// many [`ANSIString`] with same styles and colors
#[derive(Debug, Default, Clone)]
pub struct ANSIStringBuilder {
	fg: Option<Color>,
	bg: Option<Color>,
	style: Option<Style>,
}

impl ANSIStringBuilder {
	/// Returns new instance of [`ANSIStringBuilder`]
	pub fn new() -> Self {
		Self::default()
	}

	/// Clear all styles and colors from the builder
	pub fn clear(&mut self) {
		self.fg = None;
		self.bg = None;
		self.style = None;
	}

	/// Builds a new [`ANSIString`] instance without consuming builder
	pub fn build<T: AsRef<str>>(&self, s: T) -> ANSIString<T> {
		let Self { style, fg, bg } = self.clone();

		ANSIString {
			inner: s,
			fg,
			bg,
			style,
		}
	}
}

impl Colored for ANSIStringBuilder {
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

impl Styled for ANSIStringBuilder {
	type Output = Self;

	#[inline]
	fn set(mut self, style: BitFlag) -> Self::Output {
		self.style.get_or_insert(Style::default()).enable(style);

		self
	}
}
