use super::color::*;

macro_rules! make {
	($fn_name: ident$(,)? $($fn_args: ident: $arg_type: ty)*$(,)?; $method: ident, $($method_args: expr),*; $doc: expr $(,$doc_arg: tt)*) => {
		#[doc = $doc]
		///
		/// # Example
		/// ```
		/// use hel_colored::{ANSIString, Colored};
		///
		/// // Does not allocate until needed
		#[doc = concat!("let colored_str: ANSIString<&str> = \"Hello World!\".", stringify!($fn_name), stringify!(($($doc_arg)*);))]
		/// println!("{colored_str}");
		/// ```
		#[inline]
		fn $fn_name(self, $($fn_args: $arg_type),*) -> Self::Output {
			self.$method($($method_args),*)
		}
	};

	($first_fn_name: ident, $second_fn_name: ident, $arg: expr) => {
		make!($first_fn_name; colorize_fg, $arg; make!(doc, "foreground", $first_fn_name));
		make!($second_fn_name; colorize_bg, $arg; make!(doc, "background", $first_fn_name));
	};

	(doc, $part: expr, $color: expr) => {
		concat!("Colors ", $part, " of a string in ", stringify!($color))
	};
}

/// A helper trait for [`ANSIString`] and [`ANSIStringBuilder`]
pub trait Colored: Sized {
	/// This is returned by all methods
	type Output;

	#[doc(hidden)]
	fn colorize_fg(self, color: Color) -> Self::Output;
	#[doc(hidden)]
	fn colorize_bg(self, color: Color) -> Self::Output;

	make!(blue, on_blue, BLUE);
	make!(cyan, on_cyan, CYAN);
	make!(green, on_green, GREEN);
	make!(magenta, on_magenta, MAGENTA);
	make!(orange, on_orange, ORANGE);
	make!(red, on_red, RED);
	make!(yellow, on_yellow, YELLOW);

	make!(rgb, rgb: (u8, u8, u8); colorize_fg, rgb.into(); make!(doc, "foreground", rgb), (255, 120, 120));
	make!(on_rgb, rgb: (u8, u8, u8); colorize_bg, rgb.into(); make!(doc, "background", rgb), (255, 120, 120));
}
