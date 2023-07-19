#![feature(test)]
#![allow(clippy::assertions_on_constants)]

extern crate test;

use std::cmp::Ordering;

use test::{stats::Summary, Bencher};

const SHOULD_FAIL: bool = false;

fn bench<T, F: FnMut() -> T + Copy + 'static>(b: &mut Bencher, f: F) -> Summary {
	b.bench(move |i| {
		i.iter(f);

		Ok(())
	})
	.unwrap()
	.unwrap()
}

const SMALL: &str = "Hello 12345";
const BIG: &str = "Lorem ipsum dolor sit amet, \
	consectetur adipiscing elit, sed do eiusmod tempor \
	incididunt ut labore et dolore magna aliqua. Ut enim \
	ad minim veniam, quis nostrud exercitation ullamco laboris \
	nisi ut aliquip ex ea commodo consequat. Duis aute irure \
	dolor in reprehenderit in voluptate velit esse cillum dolore \
	eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat \
	non proident, sunt in culpa qui officia deserunt mollit anim \
	id est laborum.";

#[bench]
#[ignore = "For comparison only. Disabled by default"]
fn small_str(b: &mut Bencher) {
	let this = bench(b, || {
		use hel_colored::{Colored, Styled};

		SMALL
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.dim()
			.strikethrough()
			.hide()
			.invert()
	});

	let other = bench(b, || {
		use colored::Colorize;

		SMALL.bold()
	});

	dbg!(this, other);
	assert!(!SHOULD_FAIL);
	assert!(this.is_faster_than(other))
}

#[bench]
#[ignore = "For comparison only. Disabled by default"]
fn bigger_str(b: &mut Bencher) {
	let this = bench(b, || {
		use hel_colored::{Colored, Styled};

		BIG
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.strikethrough()
			.hide()
	});

	let other = bench(b, || {
		use colored::Colorize;

		BIG
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.strikethrough()
			.hidden()
	});

	dbg!(this, other);
	assert!(!SHOULD_FAIL);
	assert!(this.is_faster_than(other));
}

#[bench]
#[ignore = "For comparison only. Disabled by default"]
fn small_to_string(b: &mut Bencher) {
	let this = bench(b, || {
		use hel_colored::{Colored, Styled};

		SMALL
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.dim()
			.strikethrough()
			.hide()
			.invert()
			.to_string()
	});

	let other = bench(b, || {
		use colored::Colorize;

		SMALL.bold().to_string()
	});

	dbg!(this, other);
	assert!(!SHOULD_FAIL);
	assert!(this.is_faster_than(other))
}

#[bench]
#[ignore = "For comparison only. Disabled by default"]
fn bigger_to_string(b: &mut Bencher) {
	let this = bench(b, || {
		use hel_colored::{Colored, Styled};

		BIG
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.strikethrough()
			.hide()
			.to_string()
	});

	let other = bench(b, || {
		use colored::Colorize;

		BIG
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.strikethrough()
			.hidden()
			.to_string()
	});

	dbg!(this, other);
	assert!(!SHOULD_FAIL);
	assert!(this.is_faster_than(other));
}

#[bench]
#[ignore = "For comparison only. Disabled by default"]
#[cfg(feature = "nested")]
fn small_nested(b: &mut Bencher) {
	let this = bench(b, || {
		use hel_colored::{Colored, Styled};

		let res = SMALL
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.dim()
			.strikethrough()
			.hide()
			.invert();

		format!("Some text {res} wrapped").green().to_string()
	});

	let other = bench(b, || {
		use colored::Colorize;

		let res = SMALL.bold();

		format!("Some text {res} wrapped").green().to_string()
	});

	dbg!(this, other);
	assert!(!SHOULD_FAIL);
	assert!(this.is_faster_than(other))
}

#[bench]
#[ignore = "For comparison only. Disabled by default"]
#[cfg(feature = "nested")]
fn bigger_nested(b: &mut Bencher) {
	let this = bench(b, || {
		use hel_colored::{Colored, Styled};

		let res = BIG
			.blue()
			.bold()
			.underline()
			.on_green()
			.italic()
			.blink()
			.dim()
			.strikethrough()
			.hide()
			.invert();

		format!("Some text {res} wrapped").green().to_string()
	});

	let other = bench(b, || {
		use colored::Colorize;

		let res = BIG.bold();

		format!("Some text {res} wrapped").green().to_string()
	});

	dbg!(this, other);
	assert!(!SHOULD_FAIL);
	assert!(this.is_faster_than(other))
}

trait SummaryCmp {
	fn iter_zip(&self, other: Summary) -> Box<dyn Iterator<Item = (f64, f64)>>;

	fn cmp(&self, other: Summary) -> Ordering;
	fn diff_lower_than_fraction(&self, other: Summary, percent: f64) -> bool;

	fn is_faster_than(&self, other: Summary) -> bool {
		self.cmp(other) == Ordering::Less
	}
}

impl SummaryCmp for Summary {
	fn cmp(&self, other: Summary) -> Ordering {
		if self
			.iter_zip(other)
			.all(|(a, b)| a.total_cmp(&b) == Ordering::Less)
		{
			Ordering::Less
		} else {
			Ordering::Greater
		}
	}

	fn iter_zip(&self, other: Summary) -> Box<dyn Iterator<Item = (f64, f64)>> {
		Box::new(SummaryIter::new(*self).zip(SummaryIter::new(other)))
	}

	fn diff_lower_than_fraction(&self, other: Summary, fraction: f64) -> bool {
		self
			.iter_zip(other)
			.all(|(a, b)| (a - b).abs() < a * fraction)
	}
}

struct SummaryIter {
	inner: Summary,
	idx: usize,
}

impl SummaryIter {
	fn new(sum: Summary) -> Self {
		Self { inner: sum, idx: 0 }
	}
}

impl Iterator for SummaryIter {
	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		let res = match self.idx {
			0 => self.inner.median,
			1 => self.inner.max,
			2 => self.inner.min,
			_ => None?,
		};

		self.idx += 1;

		Some(res)
	}
}
