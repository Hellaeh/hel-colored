use crate::ansi::string::ANSIString;

use super::color::*;

pub trait Colored
where
	Self: Sized,
{
	type Output;

	fn colorize_fg(self, color: Color) -> Self::Output;
	fn colorize_bg(self, color: Color) -> Self::Output;

	// foreground
	#[inline]
	fn blue(self) -> Self::Output {
		self.colorize_fg(BLUE)
	}
	#[inline]
	fn cyan(self) -> Self::Output {
		self.colorize_fg(CYAN)
	}
	#[inline]
	fn green(self) -> Self::Output {
		self.colorize_fg(GREEN)
	}
	#[inline]
	fn magenta(self) -> Self::Output {
		self.colorize_fg(MAGENTA)
	}
	#[inline]
	fn orange(self) -> Self::Output {
		self.colorize_fg(ORANGE)
	}
	#[inline]
	fn red(self) -> Self::Output {
		self.colorize_fg(RED)
	}
	#[inline]
	fn yellow(self) -> Self::Output {
		self.colorize_fg(YELLOW)
	}

	// background
	#[inline]
	fn on_blue(self) -> Self::Output {
		self.colorize_bg(BLUE)
	}
	#[inline]
	fn on_cyan(self) -> Self::Output {
		self.colorize_bg(CYAN)
	}
	#[inline]
	fn on_green(self) -> Self::Output {
		self.colorize_bg(GREEN)
	}
	#[inline]
	fn on_magenta(self) -> Self::Output {
		self.colorize_bg(MAGENTA)
	}
	#[inline]
	fn on_orange(self) -> Self::Output {
		self.colorize_bg(ORANGE)
	}
	#[inline]
	fn on_red(self) -> Self::Output {
		self.colorize_bg(RED)
	}
	#[inline]
	fn on_yellow(self) -> Self::Output {
		self.colorize_bg(YELLOW)
	}

	#[inline]
	fn rgb(self, rgb: (u8, u8, u8)) -> Self::Output {
		self.colorize_fg(rgb.into())
	}
	#[inline]
	fn on_rgb(self, rgb: (u8, u8, u8)) -> Self::Output {
		self.colorize_bg(rgb.into())
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
