// might become a separate crate someday, for now hidden - and might
// therefore contain unused code
#![allow(dead_code)]

pub use self::iter::*;
pub use self::word_string::*;

use std::cmp::{min,Ordering};

mod address;
mod iter;
mod word_string;

pub trait FixedBitString: Sized+Clone {
	// increment from end; don't touch first `prefix` bits; returns
	// true on overflow
	fn inc(&mut self, prefix: usize) -> bool;

	fn iter(&self, prefix: usize) -> FixedBitStringIterator<Self> {
		FixedBitStringIterator::new(self.clone(), prefix)
	}

	// manipulate single bit
	fn get(&self, ndx: usize) -> bool;
	fn set(&mut self, ndx: usize, bit: bool);
	fn on(&mut self, ndx: usize);
	fn off(&mut self, ndx: usize);
	fn flip(&mut self, ndx: usize);

	fn zeroesfrom(&mut self, ndx: usize);
	fn is_zeroesfrom(&self, ndx: usize) -> bool;

	fn onesfrom(&mut self, ndx: usize);
	fn is_onesfrom(&self, ndx: usize) -> bool;

	fn all_zeroes() -> Self;
	fn all_ones() -> Self;

	fn contains(&self, prefix: usize, other: &Self) -> bool;
}

pub trait BitString: Sized+Clone {
	fn get(&self, ndx: usize) -> bool;
	fn set(&mut self, ndx: usize, bit: bool);
	fn flip(&mut self, ndx: usize);
	fn len(&self) -> usize;
	fn clip(&mut self, len: usize);
	fn append(&mut self, bit: bool);
	fn null() -> Self;

	fn shared_prefix_len(&self, other: &Self) -> usize {
		let max_len = min(self.len(), other.len());
		for i in 0..max_len {
			if self.get(i) != other.get(i) {
				return i
			}
		}
		max_len
	}

	fn shared_prefix(&self, other: &Self) -> Self {
		let mut a = self.clone();
		a.clip(self.shared_prefix_len(other));
		a
	}

	// a < b iff a != b and a is a prefix of b
	fn bitstring_partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let spl = self.shared_prefix_len(other);
		if spl == self.len() {
			if spl == other.len() {
				Some(Ordering::Equal)
			} else {
				Some(Ordering::Less)
			}
		} else if spl == other.len() {
			Some(Ordering::Greater)
		} else {
			None
		}
	}
}
