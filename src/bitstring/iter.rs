use super::FixedBitString;

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct FixedBitStringIterator<B: FixedBitString+Sized+Clone> {
	next: Option<B>,
	prefix: usize,
}

impl<B: FixedBitString+Sized+Clone> FixedBitStringIterator<B> {
	pub fn new(start: B, prefix: usize) -> Self {
		FixedBitStringIterator{
			next: Some(start),
			prefix: prefix,
		}
	}
}

impl<B: FixedBitString+Sized+Clone> Iterator for FixedBitStringIterator<B> {
	type Item = B;

	fn next(&mut self) -> Option<Self::Item> {
		let mut overflow = false;
		let result = match self.next {
			None => None,
			Some(ref mut next) => {
				let result = Some(next.clone());
				overflow = next.inc(self.prefix);
				result
			}
		};
		if overflow {
			self.next = None;
		}
		result
	}
}
