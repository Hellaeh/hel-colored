use super::BitFlag;

const BOLD: &str = "1";
const DIM: &str = "2";
const ITALIC: &str = "3";
const UNDERLINE: &str = "4";
const BLINK: &str = "5";
const INVERT: &str = "7";
const HIDE: &str = "8";
const STRIKETHROUGH: &str = "9";

impl From<BitFlag> for &str {
	#[inline]
	fn from(value: BitFlag) -> Self {
		match value {
			BitFlag::Bold => BOLD,
			BitFlag::Dim => DIM,
			BitFlag::Italic => ITALIC,
			BitFlag::Underline => UNDERLINE,
			BitFlag::Blink => BLINK,
			BitFlag::Invert => INVERT,
			BitFlag::Hide => HIDE,
			BitFlag::Strikethrough => STRIKETHROUGH,
		}
	}
}
