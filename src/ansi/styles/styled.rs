use crate::ansi::string::ANSIString;

use super::BitFlag;

pub trait Styled
where
	Self: Sized,
{
	type Output;

	fn set(self, style: BitFlag) -> Self::Output;

	#[inline]
	fn bold(self) -> Self::Output {
		self.set(BitFlag::Bold)
	}
	#[inline]
	fn dim(self) -> Self::Output {
		self.set(BitFlag::Dim)
	}
	#[inline]
	fn italic(self) -> Self::Output {
		self.set(BitFlag::Italic)
	}
	#[inline]
	fn underline(self) -> Self::Output {
		self.set(BitFlag::Underline)
	}
	#[inline]
	fn blink(self) -> Self::Output {
		self.set(BitFlag::Blink)
	}
	#[inline]
	fn invert(self) -> Self::Output {
		self.set(BitFlag::Invert)
	}
	#[inline]
	fn hide(self) -> Self::Output {
		self.set(BitFlag::Hide)
	}
	#[inline]
	fn strikethrough(self) -> Self::Output {
		self.set(BitFlag::Strikethrough)
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
