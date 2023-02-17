macro_rules! rgb {
	($r: literal, $g: literal, $b: literal) => {
		concat!(
			"\x1b[38;2;",
			stringify!($r),
			";",
			stringify!($g),
			";",
			stringify!($b),
			"m"
		)
	};
}

pub static BLUE: &str = rgb!(50, 50, 225);
pub static CYAN: &str = rgb!(100, 225, 225);
pub static GREEN: &str = rgb!(50, 225, 50);
pub static MAGENTA: &str = rgb!(225, 100, 225);
pub static ORANGE: &str = rgb!(225, 100, 100);
pub static RED: &str = rgb!(225, 50, 50);
pub static YELLOW: &str = rgb!(225, 225, 50);

pub static END: &str = "\x1b[0m";
