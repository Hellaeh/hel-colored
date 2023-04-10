#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BitFlag {
	Bold = 1 << 0,
	Dim = 1 << 1,
	Italic = 1 << 2,
	Underline = 1 << 3,
	Blink = 1 << 4,
	Invert = 1 << 5,
	Hide = 1 << 6,
	Strikethrough = 1 << 7,
}

pub struct BitFlagIter(u8);

impl std::ops::BitAnd<BitFlag> for u8 {
	type Output = u8;

	#[inline]
	fn bitand(self, rhs: BitFlag) -> Self::Output {
		self & rhs as u8
	}
}

impl std::ops::BitOrAssign<BitFlag> for u8 {
	#[inline]
	fn bitor_assign(&mut self, rhs: BitFlag) {
		*self |= rhs as u8
	}
}

impl PartialEq<BitFlag> for u8 {
	#[inline]
	fn eq(&self, other: &BitFlag) -> bool {
		*self == *other as u8
	}
}

impl BitFlag {
	#[inline]
	pub fn into_iter() -> BitFlagIter {
		const FIRST_BIT: u8 = 0b1;
		BitFlagIter(FIRST_BIT)
	}
}

impl Iterator for BitFlagIter {
	type Item = BitFlag;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 0 {
			return None;
		}

		let res = unsafe { std::mem::transmute(self.0) };

		self.0 <<= 1;

		Some(res)
	}
}

#[cfg(test)]
mod test {
	extern crate test;

	use test::Bencher;

	use super::*;

	#[test]
	fn check_bit_flag() {
		assert!(0b1111 & BitFlag::Bold == BitFlag::Bold)
	}

	#[test]
	fn bit_or_assign() {
		let mut bits = 0b01110000;

		bits |= BitFlag::Bold;

		assert!(bits & BitFlag::Bold == BitFlag::Bold);
	}

	#[test]
	fn iter() {
		let iter = BitFlag::into_iter();

		assert_eq!(iter.count(), 8);
	}

	struct TestBitFlagMatchIter(u8);
	impl Iterator for TestBitFlagMatchIter {
		type Item = BitFlag;

		#[inline]
		fn next(&mut self) -> Option<Self::Item> {
			self.0 += 1;

			match self.0 {
				1 => Some(BitFlag::Bold),
				2 => Some(BitFlag::Dim),
				3 => Some(BitFlag::Italic),
				4 => Some(BitFlag::Underline),
				5 => Some(BitFlag::Blink),
				6 => Some(BitFlag::Invert),
				7 => Some(BitFlag::Hide),
				8 => Some(BitFlag::Strikethrough),
				_ => None,
			}
		}
	}

	#[bench]
	fn bench_iter(b: &mut Bencher) {
		let main = match b
			.bench(|i| {
				i.iter(|| BitFlag::into_iter().count());

				Ok(())
			})
			.unwrap()
		{
			Some(sum) => sum,
			None => return,
		};

		let matching = b
			.bench(|i| {
				i.iter(|| TestBitFlagMatchIter(0).count());

				Ok(())
			})
			.unwrap()
			.unwrap();

		dbg!(main);
		assert!([matching]
			.iter()
			.all(|e| (main.median - e.median).abs() < 1.0))
	}
}
