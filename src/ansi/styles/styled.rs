use super::BitFlag;

macro_rules! make {
	($fn_name: ident; $method_args: expr; $doc: expr) => {
		// rust-analyzer wont pick up macro in doc attributes yet
		// https://github.com/rust-lang/rust-analyzer/issues/8092
		#[doc = $doc]
		///
		/// # Example
		/// ```
		/// use hel_colored::{ANSIString, Styled};
		///
		/// // Does not allocate until needed
		#[doc = concat!("let styled_str: ANSIString<&str> = \"Hello World!\".", stringify!($fn_name), "();")]
		/// println!("{styled_str}");
		/// ```
		#[inline]
		fn $fn_name(self) -> Self::Output {
			self.set($method_args)
		}
	};
}

/// A helper trait for [`ANSIString`] and [`ANSIStringBuilder`]
pub trait Styled: Sized {
	/// This is returned by all methods
	type Output;

	/// One should not use this method directly
	#[doc(hidden)]
	fn set(self, style: BitFlag) -> Self::Output;

	make!(bold; BitFlag::Bold; "Causes a string to be displayed as bold");
	make!(dim; BitFlag::Dim; "Causes a string to appear darker (not widely supported)");
	make!(italic; BitFlag::Italic; "Causes a string to be displayed as italic");
	make!(underline; BitFlag::Underline; "Causes a string to be displayed as underlined");
	make!(blink; BitFlag::Blink; "Causes a string to blink (not widely supported)");
	make!(invert; BitFlag::Invert; "Inverts colors of a string (not widely supported)");
	make!(hide; BitFlag::Hide; "Hides a string (not widely supported)");
	make!(strikethrough; BitFlag::Strikethrough; "Causes a string to be displayed as striked out");
}
