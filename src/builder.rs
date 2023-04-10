use crate::{
	ansi::{BitFlag, Color, Style},
	ANSIString, Colored, Styled,
};

#[derive(Debug, Default, Clone)]
pub struct ANSIStringBuilder {
	fg: Option<Color>,
	bg: Option<Color>,
	style: Option<Style>,
}

impl ANSIStringBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn clear(&mut self) {
		self.fg = None;
		self.bg = None;
		self.style = None;
	}

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

	fn set(mut self, style: BitFlag) -> Self::Output {
		self.style.get_or_insert(Style::default()).enable(style);

		self
	}
}
