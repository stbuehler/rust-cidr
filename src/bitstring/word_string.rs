use std::cmp::min;

use super::{BitString,FixedBitString};

#[derive(Clone,Debug)]
pub struct BitWordString<W: FixedBitString+Sized+Clone> {
	pub bitwords : W,
	pub len : usize,
}

impl<W: FixedBitString+Clone> BitWordString<W> {
	pub fn contains(&self, word: &W) -> bool {
		self.bitwords.contains(self.len, word)
	}
}

impl<W: FixedBitString+Clone> BitString for BitWordString<W> {
	fn get(&self, ndx: usize) -> bool {
		self.bitwords.get(ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		assert!(ndx < self.len);
		self.bitwords.set(ndx, bit);
	}

	fn flip(&mut self, ndx: usize) {
		assert!(ndx < self.len);
		self.bitwords.flip(ndx);
	}

	fn len(&self) -> usize {
		self.len
	}

	fn clip(&mut self, len: usize) {
		self.bitwords.zeroesfrom(len);
		self.len = min(self.len, len);
	}

	fn append(&mut self, bit: bool) {
		self.bitwords.set(self.len, bit);
		self.len += 1;
	}

	fn null() -> Self {
		BitWordString{
			bitwords: W::all_zeroes(),
			len: 0,
		}
	}

	fn shared_prefix_len(&self, other: &Self) -> usize {
		let max_len = min(self.len, other.len);
		W::shared_prefix_len(&self.bitwords, &other.bitwords, max_len)
	}
}
