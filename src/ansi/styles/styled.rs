use super::BitFlag;

pub trait Styled
where
	Self: Sized,
{
	type Output;

	#[doc(hidden)]
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
