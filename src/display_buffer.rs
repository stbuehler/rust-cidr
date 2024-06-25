#![cfg(any(feature = "serde", test))]

// formatting Cidr/Inet to str without std/alloc

use core::fmt;
#[cfg(not(feature = "no_unsafe"))]
use core::mem::MaybeUninit;

// from core::net, adapted to work with stable rust and `no_unsafe`
// https://github.com/rust-lang/rust/blob/d8d5732456d375f7c4bdc2f6ad771989a5e0ae02/library/core/src/net/display_buffer.rs
// https://github.com/rust-lang/rust/blob/master/COPYRIGHT

struct DisplayBuffer<const SIZE: usize> {
	#[cfg(not(feature = "no_unsafe"))]
	buf: [MaybeUninit<u8>; SIZE],
	#[cfg(feature = "no_unsafe")]
	buf: [u8; SIZE],
	len: usize,
}

impl<const SIZE: usize> DisplayBuffer<SIZE> {
	#[inline]
	pub fn new() -> Self {
		#[cfg(not(feature = "no_unsafe"))]
		{
			let buf = unsafe { MaybeUninit::<[MaybeUninit<u8>; SIZE]>::uninit().assume_init() };
			Self { buf, len: 0 }
		}
		#[cfg(feature = "no_unsafe")]
		{
			Self {
				buf: [0; SIZE],
				len: 0,
			}
		}
	}
}

impl<const SIZE: usize> core::ops::Deref for DisplayBuffer<SIZE> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &Self::Target {
		// SAFETY: `buf` is only written to by the `fmt::Write::write_str` implementation
		// which writes a valid UTF-8 string to `buf` and correctly sets `len`.
		#[cfg(not(feature = "no_unsafe"))]
		unsafe {
			let written = &self.buf[..self.len];
			let s = &*(written as *const [_] as *const [u8]);
			core::str::from_utf8_unchecked(s)
		}
		#[cfg(feature = "no_unsafe")]
		{
			core::str::from_utf8(&self.buf[..self.len]).expect("proper string")
		}
	}
}

impl<const SIZE: usize> fmt::Write for DisplayBuffer<SIZE> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let bytes = s.as_bytes();

		if let Some(buf) = self.buf.get_mut(self.len..(self.len + bytes.len())) {
			#[cfg(not(feature = "no_unsafe"))]
			buf.copy_from_slice(unsafe { core::mem::transmute(bytes) });
			#[cfg(feature = "no_unsafe")]
			buf.copy_from_slice(bytes);
			self.len += bytes.len();
			Ok(())
		} else {
			Err(fmt::Error)
		}
	}
}

const LONGEST_IPV6_CIDR: &str = "V6(ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/128)";

pub fn format_cidr_fixed(args: fmt::Arguments<'_>) -> impl core::ops::Deref<Target = str> {
	let mut result = DisplayBuffer::<{ LONGEST_IPV6_CIDR.len() }>::new();
	fmt::write(&mut result, args).expect("format failed");
	result
}

macro_rules! format_cidr_fixed {
	($($tt:tt)*) => {
		&*crate::display_buffer::format_cidr_fixed(format_args!($($tt)*))
	};
}
