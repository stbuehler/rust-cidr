use std::net::{Ipv4Addr,Ipv6Addr};

use bigendian::BigEndianManipulation;
use super::FixedBitString;

fn with_ipv4_mut_slice<F, T>(addr: &mut Ipv4Addr, f: F) -> T
where
	F: FnOnce(&mut[u8]) -> T
{
	let mut o = addr.octets();
	let result = f(&mut o);
	*addr = Ipv4Addr::from(o);
	result
}

impl FixedBitString for Ipv4Addr {
	fn inc(&mut self, prefix: usize) -> bool {
		with_ipv4_mut_slice(self, |slice| {
			BigEndianManipulation::inc(slice, prefix)
		})
	}

	fn get(&self, ndx: usize) -> bool {
		BigEndianManipulation::get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv4_mut_slice(self, |slice| {
			BigEndianManipulation::set(slice, ndx, bit)
		})
	}

	fn on(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| {
			BigEndianManipulation::on(slice, ndx)
		})
	}

	fn off(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| {
			BigEndianManipulation::off(slice, ndx)
		})
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| {
			BigEndianManipulation::flip(slice, ndx)
		})
	}

	fn zeroesfrom(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| {
			BigEndianManipulation::zeroesfrom(slice, ndx)
		})
	}

	fn is_zeroesfrom(&self, ndx: usize) -> bool {
		BigEndianManipulation::is_zeroesfrom(&self.octets(), ndx)
	}

	fn onesfrom(&mut self, ndx: usize) {
		with_ipv4_mut_slice(self, |slice| {
			BigEndianManipulation::onesfrom(slice, ndx)
		})
	}

	fn is_onesfrom(&self, ndx: usize) -> bool {
		BigEndianManipulation::is_onesfrom(&self.octets(), ndx)
	}

	fn all_zeroes() -> Self {
		Ipv4Addr::new(0, 0, 0, 0)
	}

	fn all_ones() -> Self {
		Ipv4Addr::new(!0, !0, !0, !0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		BigEndianManipulation::contains(&self.octets(), prefix, &other.octets())
	}
}

fn with_ipv6_mut_slice<F, T>(addr: &mut Ipv6Addr, f: F) -> T
where
	F: FnOnce(&mut[u8]) -> T
{
	let mut o = addr.octets();
	let result = f(&mut o);
	*addr = Ipv6Addr::from(o);
	result
}

impl FixedBitString for Ipv6Addr {
	fn inc(&mut self, prefix: usize) -> bool {
		with_ipv6_mut_slice(self, |slice| {
			BigEndianManipulation::inc(slice, prefix)
		})
	}

	fn get(&self, ndx: usize) -> bool {
		BigEndianManipulation::get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		with_ipv6_mut_slice(self, |slice| {
			BigEndianManipulation::set(slice, ndx, bit)
		})
	}

	fn on(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| {
			BigEndianManipulation::on(slice, ndx)
		})
	}

	fn off(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| {
			BigEndianManipulation::off(slice, ndx)
		})
	}

	fn flip(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| {
			BigEndianManipulation::flip(slice, ndx)
		})
	}

	fn zeroesfrom(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| {
			BigEndianManipulation::zeroesfrom(slice, ndx)
		})
	}

	fn is_zeroesfrom(&self, ndx: usize) -> bool {
		BigEndianManipulation::is_zeroesfrom(&self.octets(), ndx)
	}

	fn onesfrom(&mut self, ndx: usize) {
		with_ipv6_mut_slice(self, |slice| {
			BigEndianManipulation::onesfrom(slice, ndx)
		})
	}

	fn is_onesfrom(&self, ndx: usize) -> bool {
		BigEndianManipulation::is_onesfrom(&self.octets(), ndx)
	}

	fn all_zeroes() -> Self {
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)
	}

	fn all_ones() -> Self {
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0)
	}

	fn contains(&self, prefix: usize, other: &Self) -> bool {
		BigEndianManipulation::contains(&self.octets(), prefix, &other.octets())
	}
}
