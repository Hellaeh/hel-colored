use std::slice::SliceIndex;

pub(crate) trait UnsafeBytes {
	fn fmt_str_from<I: SliceIndex<Self, Output = [u8]>>(
		&self,
		index: I,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result;
}

impl UnsafeBytes for [u8] {
	#[inline]
	fn fmt_str_from<I: SliceIndex<Self, Output = [u8]>>(
		&self,
		index: I,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		let bytes = unsafe { self.get_unchecked(index) };
		let str = unsafe { std::str::from_utf8_unchecked(bytes) };

		f.write_str(str)
	}
}
