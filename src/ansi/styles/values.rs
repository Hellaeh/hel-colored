use super::BitFlag;

impl From<BitFlag> for char {
	#[inline]
	fn from(value: BitFlag) -> Self {
		match value {
			BitFlag::Bold => '1',
			BitFlag::Dim => '2',
			BitFlag::Italic => '3',
			BitFlag::Underline => '4',
			BitFlag::Blink => '5',
			BitFlag::Invert => '7',
			BitFlag::Hide => '8',
			BitFlag::Strikethrough => '9',
		}
	}
}
