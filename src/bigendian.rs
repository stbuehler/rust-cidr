use std::mem::size_of;

// manipulate bitstrings in form of slices of u*
pub trait BigEndianManipulation: Sized {
	// bit count of single element
	fn elembits() -> usize {
		8 * size_of::<Self>()
	}

	// integer with single bit set. bit 0 is the highest bit (big endian)
	fn mask(ndx: usize) -> Self;

	// increment from right; don't touch first `prefix` bits; returns
	// true on overflow
	fn inc(slice: &mut [Self], prefix: usize) -> bool;

	// manipulate bit at certain ndx
	fn get(slice: &[Self], ndx: usize) -> bool;
	fn set(slice: &mut [Self], ndx: usize, bit: bool) {
		if bit {
			Self::on(slice, ndx)
		} else {
			Self::off(slice, ndx)
		}
	}
	fn on(slice: &mut [Self], ndx: usize);
	fn off(slice: &mut [Self], ndx: usize);
	fn flip(slice: &mut [Self], ndx: usize);

	// set all bits to zero from cetain (including) index
	fn zeroesfrom(slice: &mut [Self], ndx: usize);
	// whether `zeroesfrom` wouldn't change anything
	fn is_zeroesfrom(slice: &[Self], ndx: usize) -> bool;

	// set all bits to zero 
	fn onesfrom(slice: &mut [Self], ndx: usize);
	// whether `onesfrom` wouldn't change anything
	fn is_onesfrom(slice: &[Self], ndx: usize) -> bool;

	fn contains(slice: &[Self], prefix: usize, other: &[Self]) -> bool;
}

macro_rules! impl_big_endian_for {
	($t:ty) => (
		impl BigEndianManipulation for $t {
			fn mask(ndx: usize) -> Self {
				let bits = Self::elembits();
				let bit_ndx = bits - 1 - (ndx % bits);
				1 << bit_ndx
			}

			fn inc(slice: &mut [Self], prefix: usize) -> bool {
				let prev_bit = if prefix > 0 { Some(Self::get(slice, prefix - 1)) } else { None };
				let mut overflow = true;
				for i in (0..slice.len()).rev() {
					let (r, o) = slice[i].overflowing_add(1);
					slice[i] = r;
					overflow = o;
					if !overflow { break; }
				}

				match prev_bit {
					Some(prev_bit) => {
						if prev_bit != Self::get(slice, prefix - 1) {
							Self::flip(slice, prefix - 1);
							true
						} else {
							false
						}
					},
					None => overflow,
				}
			}

			fn get(slice: &[Self], ndx: usize) -> bool {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				0 != (slice[slice_ndx] & mask)
			}

			fn on(slice: &mut [Self], ndx: usize) {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				slice[slice_ndx] = slice[slice_ndx] | mask;
			}

			fn off(slice: &mut [Self], ndx: usize) {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				slice[slice_ndx] = slice[slice_ndx] & !mask;
			}

			fn flip(slice: &mut [Self], ndx: usize) {
				let mask = Self::mask(ndx);
				let slice_ndx = ndx / Self::elembits();
				slice[slice_ndx] = slice[slice_ndx] ^ mask;
			}

			fn zeroesfrom(slice: &mut [Self], ndx: usize) {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						slice[i] = 0;
					}
				}
				else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
					slice[slice_ndx] = slice[slice_ndx] & !mask;
					for i in slice_ndx+1..slice.len() {
						slice[i] = 0;
					}
				}
			}

			fn is_zeroesfrom(slice: &[Self], ndx: usize) -> bool {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						if 0 != slice[i] { return false; }
					}
				}
				else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
					if 0 != slice[slice_ndx] & mask { return false; }
					for i in slice_ndx+1..slice.len() {
						if 0 != slice[i] { return false; }
					}
				}
				true
			}

			fn onesfrom(slice: &mut [Self], ndx: usize) {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						slice[i] = !0;
					}
				}
				else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
					slice[slice_ndx] = slice[slice_ndx] | mask;
					for i in slice_ndx+1..slice.len() {
						slice[i] = !0;
					}
				}
			}

			fn is_onesfrom(slice: &[Self], ndx: usize) -> bool {
				let slice_ndx = ndx / Self::elembits();
				if 0 == ndx % Self::elembits() {
					for i in slice_ndx..slice.len() {
						if slice[i] != !0 { return false; }
					}
				}
				else if slice_ndx < slice.len() {
					let mask = Self::mask(ndx - 1) - 1;
					if slice[slice_ndx] | !mask != !0 { return false; }
					for i in slice_ndx+1..slice.len() {
						if slice[i] != !0 { return false; }
					}
				}
				true
			}

			fn contains(slice: &[Self], prefix: usize, other: &[Self]) -> bool {
				let slice_ndx = prefix / Self::elembits();
				for i in 0..slice_ndx {
					if slice[i] != other[i] { return false; }
				}
				if 0 == prefix % Self::elembits() {
					return true;
				}
				let mask = !(Self::mask(prefix - 1) - 1);
				0 == mask & (slice[slice_ndx] ^ other[slice_ndx])
			}
		}
	)
}

impl_big_endian_for!{u8}
// impl_big_endian_for!{u16}
// impl_big_endian_for!{u32}
// impl_big_endian_for!{u64}
