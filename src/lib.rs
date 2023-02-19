mod colors;

use colors::*;

pub trait Colored {
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

impl<T: AsRef<str>> Colored for T {
	fn blue(&self) -> String {
		wrap(BLUE, self.as_ref())
	}

	fn cyan(&self) -> String {
		wrap(CYAN, self.as_ref())
	}

	fn green(&self) -> String {
		wrap(GREEN, self.as_ref())
	}

	fn magenta(&self) -> String {
		wrap(MAGENTA, self.as_ref())
	}

	fn orange(&self) -> String {
		wrap(ORANGE, self.as_ref())
	}

	fn red(&self) -> String {
		wrap(RED, self.as_ref())
	}

	fn yellow(&self) -> String {
		wrap(YELLOW, self.as_ref())
	}

	fn rgb(&self, r: u8, g: u8, b: u8) -> String {
		wrap(&format!("\x1b[38;2;{r};{g};{b}m"), self.as_ref())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn primary() {
		let test = "Hello 12345";

		assert_eq!("\x1b[38;2;225;50;50mHello 12345\x1b[0m", test.red());
		assert_eq!(
			"\x1b[38;2;255;255;255mHello 12345\x1b[0m",
			test.rgb(255, 255, 255)
		);

		assert_eq!(
			test.to_owned().red(),
			"\x1b[38;2;225;50;50mHello 12345\x1b[0m"
		)
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
