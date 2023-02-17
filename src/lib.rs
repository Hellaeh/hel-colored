mod colors;

use colors::*;

trait Colored {
	fn blue(&self) -> String;
	fn cyan(&self) -> String;
	fn green(&self) -> String;
	fn magenta(&self) -> String;
	fn orange(&self) -> String;
	fn red(&self) -> String;
	fn yellow(&self) -> String;

	fn rgb(&self, red: u8, green: u8, blue: u8) -> String;
}

fn wrap(color: &str, str: &str) -> String {
	format!("{color}{str}{END}")
}

impl Colored for &'static str {
	fn blue(&self) -> String {
		wrap(BLUE, self)
	}

	fn cyan(&self) -> String {
		wrap(CYAN, self)
	}

	fn green(&self) -> String {
		wrap(GREEN, self)
	}

	fn magenta(&self) -> String {
		wrap(MAGENTA, self)
	}

	fn orange(&self) -> String {
		wrap(ORANGE, self)
	}

	fn red(&self) -> String {
		wrap(RED, self)
	}

	fn yellow(&self) -> String {
		wrap(YELLOW, self)
	}

	fn rgb(&self, r: u8, g: u8, b: u8) -> String {
		wrap(&format!("\x1b[38;2;{r};{g};{b}m"), self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn primary() {
		assert_eq!("\x1b[38;2;225;50;50mHello\x1b[0m", "Hello".red());
		assert_eq!(
			"\x1b[38;2;255;255;255mHello\x1b[0m",
			"Hello".rgb(255, 255, 255)
		);
	}

	#[test]
	fn fail() {
		let test = "Test 12345";

		println!("{}", test.blue());
		println!("{}", test.cyan());
		println!("{}", test.green());
		println!("{}", test.magenta());
		println!("{}", test.orange());
		println!("{}", test.red());
		println!("{}", test.yellow());

		println!("{}", test.rgb(0xff, 0xff, 0xff));

		// panic!("Have to fail") //uncomment to see output
	}
}
