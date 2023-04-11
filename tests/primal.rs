use hel_colored::{ANSIString, ANSIStringBuilder, Colored, Styled};

macro_rules! TEST_STRING {
	() => {
		"Hello 12345"
	};
}

macro_rules! wrap {
	($codes: expr) => {
		concat!("\x1b[0;", $codes, "m", TEST_STRING!(), "\x1b[0m")
	};
}

#[test]
#[ignore = "run \"cargo t -- --ignored\" to see output"]
fn output() {
	let test = TEST_STRING!();
	let sep = "-".repeat(test.len());

	println!("{}", test.bold());
	println!("{}", test.dim());
	println!("{}", test.italic());
	println!("{}", test.underline());
	println!("{}", test.blink());
	println!("{}", test.invert());
	println!("{} {}", test.hide(), "<--- Should be hidden".green());
	println!("{}", test.strikethrough());

	println!("{sep}");

	println!("{}", test.blue());
	println!("{}", test.cyan());
	println!("{}", test.green());
	println!("{}", test.magenta());
	println!("{}", test.orange());
	println!("{}", test.red());
	println!("{}", test.yellow());
	println!("{}", test.rgb((0xff, 0xff, 0xff)));

	println!("{sep}");

	println!("{}", test.on_blue());
	println!("{}", test.on_cyan());
	println!("{}", test.on_green());
	println!("{}", test.on_magenta());
	println!("{}", test.on_orange());
	println!("{}", test.on_red());
	println!("{}", test.on_yellow());
	println!("{}", test.on_rgb((0xff, 0xff, 0xff)));

	panic!("Have to fail")
}

#[test]
fn primary() {
	let test = TEST_STRING!();

	assert_eq!(wrap!("38;2;225;50;50"), test.red().to_string());
	assert_eq!(
		wrap!("38;2;255;255;255"),
		test.rgb((255, 255, 255)).to_string()
	);
	assert_eq!(wrap!("38;2;225;50;50"), test.to_owned().red().to_string());
	assert_eq!(wrap!("1"), test.bold().to_string());
	assert_eq!(
		wrap!("38;2;225;50;50;1;3;4"),
		test.underline().bold().italic().red().to_string()
	);

	assert!(test.bold().is_styled());
	assert!(!test.bold().is_colored());
	assert!(!test.red().is_styled());
	assert!(test.red().is_colored());
}

#[test]
fn format() {
	let test = TEST_STRING!();

	let s1 = format!("{:.1}", test.bold());
	let s2 = test.chars().next().unwrap().to_string().bold().to_string();
	assert_eq!(s1, s2);

	let len = test.len();
	let s1 = format!("{:w$}", test.red(), w = len);
	assert!(s1.len() > len * 2);
}

#[test]
fn should_work_for_refs() {
	fn make_blue_for_mut<T: AsRef<str>>(s: &mut ANSIString<T>) {
		s.blue();
		let s = s.to_string();
		assert_eq!(s, TEST_STRING!().blue().to_string())
	}

	fn make_green_for_ref<T: AsRef<str> + std::clone::Clone>(s: &ANSIString<T>) {
		let s = s.clone().green();

		assert_eq!(s.to_string(), TEST_STRING!().green().to_string())
	}

	let mut s = TEST_STRING!().red();

	make_blue_for_mut(&mut s);
	make_green_for_ref(&s);

	assert_eq!(s.to_string(), TEST_STRING!().blue().to_string())
}

#[test]
fn using_builder() {
	let builder = ANSIStringBuilder::new().bold().underline().on_orange();

	let test = TEST_STRING!();
	let res = builder.build(test).to_string();

	println!("{}", res);
	assert!(test.len() < res.len());

	let test = "1234556778";
	let res = builder.build(test).to_string();

	println!("{}", res);
	assert!(test.len() < res.len());

	let builder: ANSIStringBuilder = ANSIStringBuilder::new().bold().underline().orange();
	let str1 = builder.build("Hello");
	let str2 = builder.build("World!");
	println!("{str1}, {str2}");
}
