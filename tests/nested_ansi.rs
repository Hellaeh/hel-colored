use hel_colored::{Colored, Styled};

#[test]
fn single_nested_ansi_string_at_a_time() {
	let res = "blue text".blue().underline();
	let res = format!("green {res} wrapping").green().italic();

	let should_be = "\x1b[0;38;2;50;225;50;3mgreen \
	\x1b[0;38;2;50;50;225;4mblue text\x1b[0;38;2;50;225;50;3m \
	wrapping\x1b[0m";

	println!("{}", should_be);
	assert_eq!(should_be, res.to_string());

	let res = format!("Red {res} wrapping").red().strikethrough();

	let should_be = "\x1b[0;38;2;225;50;50;9mRed \
	\x1b[0;38;2;50;225;50;3mgreen \
	\x1b[0;38;2;50;50;225;4mblue text\x1b[0;38;2;50;225;50;3m \
	wrapping\x1b[0;38;2;225;50;50;9m \
	wrapping\x1b[0m";

	println!("{}", should_be);
	assert_eq!(should_be, res.to_string());
}

#[test]
fn multiple_nested_ansi_string_at_a_time() {
	let res = "blue text".blue();
	let res = format!("green {res} wrapping, another {res} wrapping").green();

	let should_be = "\u{1b}[0;38;2;50;225;50mgreen \
	\u{1b}[0;38;2;50;50;225mblue text\u{1b}[0;38;2;50;225;50m \
	wrapping, another \u{1b}[0;38;2;50;50;225mblue \
	text\u{1b}[0;38;2;50;225;50m wrapping\u{1b}[0m";

	println!("{}", should_be);
	assert_eq!(should_be, res.to_string());
}
