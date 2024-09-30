use crate::{
	internal_traits::PrivInetPair,
	num::NumberOfAddresses,
	Address,
	Inet,
	InetPair,
};

/// Iterator type to iterate over a list of IP addresses within a network
#[derive(Clone, Copy, Debug)]
pub struct InetIterator<A: Address> {
	state: Option<A::InetPair>,
}

impl<A: Address> InetIterator<A> {
	pub(crate) const fn _new(range_pair: A::InetPair) -> Self {
		Self {
			state: Some(range_pair),
		}
	}

	/// Iterate only over addresses (without network prefix length)
	pub const fn addresses(self) -> InetAddressIterator<A> {
		InetAddressIterator { inner: self }
	}
}

impl<A: Address> Iterator for InetIterator<A> {
	type Item = A::Inet;

	fn next(&mut self) -> Option<Self::Item> {
		let state = self.state.as_mut().take()?;
		let res = state.first();
		if !state._inc_first() {
			self.state = None;
		}
		Some(res)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let state = match &self.state {
			None => return (0, Some(0)),
			Some(state) => state,
		};
		match state._covered_addresses() {
			NumberOfAddresses::MaxIpv6Addresses => (0, None),
			NumberOfAddresses::Count(c) => {
				if c > (usize::MAX as u128) {
					(usize::MAX, None)
				} else {
					let c = c as usize;
					(c, Some(c))
				}
			},
		}
	}

	fn count(self) -> usize
	where
		Self: Sized,
	{
		self.size_hint().1.expect("iterator count overflow")
	}
}

impl<A: Address> core::iter::DoubleEndedIterator for InetIterator<A> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let state = self.state.as_mut().take()?;
		let res = state.second();
		if !state._dec_second() {
			self.state = None;
		}
		Some(res)
	}
}

impl<A: Address> core::iter::FusedIterator for InetIterator<A> {}

/// Iterator type to iterate over a list of IP addresses in a network
#[derive(Clone, Copy, Debug)]
pub struct InetAddressIterator<A: Address> {
	inner: InetIterator<A>,
}

impl<A: Address> Iterator for InetAddressIterator<A> {
	type Item = A;

	fn next(&mut self) -> Option<Self::Item> {
		Some(self.inner.next()?.address())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}

	fn count(self) -> usize
	where
		Self: Sized,
	{
		self.inner.count()
	}
}

impl<A: Address> core::iter::DoubleEndedIterator for InetAddressIterator<A> {
	fn next_back(&mut self) -> Option<Self::Item> {
		Some(self.inner.next_back()?.address())
	}
}

impl<A: Address> core::iter::FusedIterator for InetAddressIterator<A> {}

#[cfg(test)]
mod tests {
	use crate::{
		IpCidr,
		Ipv4Cidr,
		Ipv6Cidr,
	};
	use std::net::{
		IpAddr,
		Ipv4Addr,
		Ipv6Addr,
	};

	fn check_list_iter<T: PartialEq + core::fmt::Debug>(
		data: impl AsRef<[T]>,
		iter: impl IntoIterator<Item = T>,
	) {
		let mut iter = iter.into_iter();
		for elem in data.as_ref() {
			assert_eq!(Some(elem), iter.next().as_ref());
		}
		assert_eq!(None, iter.next());
	}

	fn check_iter_iter<T1, T2>(
		iter1: impl IntoIterator<Item = T1>,
		iter2: impl IntoIterator<Item = T2>,
	) where
		T1: core::fmt::Debug + PartialEq<T2>,
		T2: core::fmt::Debug,
	{
		let mut iter2 = iter2.into_iter();
		for elem in iter1 {
			assert_eq!(elem, iter2.next().expect("second iterator too short"));
		}
		assert!(iter2.next().is_none(), "second iterator too many elements");
	}

	fn test_v4(s: &'static str, l: &[Ipv4Addr]) {
		check_list_iter(l, s.parse::<Ipv4Cidr>().unwrap().iter().addresses());

		check_iter_iter(
			s.parse::<IpCidr>().unwrap().iter().addresses(),
			l.iter().map(|e: &Ipv4Addr| IpAddr::V4(*e)),
		);

		check_iter_iter(
			s.parse::<Ipv4Cidr>().unwrap().iter().addresses().rev(),
			l.iter().cloned().rev(),
		);

		check_iter_iter(
			s.parse::<IpCidr>().unwrap().iter().addresses().rev(),
			l.iter().map(|e| IpAddr::V4(*e)).rev(),
		);
	}

	fn test_v6(s: &'static str, l: &[Ipv6Addr]) {
		check_list_iter(l, s.parse::<Ipv6Cidr>().unwrap().iter().addresses());

		check_iter_iter(
			s.parse::<IpCidr>().unwrap().iter().addresses(),
			l.iter().map(|e: &Ipv6Addr| IpAddr::V6(*e)),
		);

		check_iter_iter(
			s.parse::<Ipv6Cidr>().unwrap().iter().addresses().rev(),
			l.iter().cloned().rev(),
		);

		check_iter_iter(
			s.parse::<IpCidr>().unwrap().iter().addresses().rev(),
			l.iter().map(|e| IpAddr::V6(*e)).rev(),
		);
	}

	#[test]
	fn range_v4_28bit() {
		test_v4(
			"192.168.4.48/28",
			&[
				Ipv4Addr::new(192, 168, 4, 48),
				Ipv4Addr::new(192, 168, 4, 49),
				Ipv4Addr::new(192, 168, 4, 50),
				Ipv4Addr::new(192, 168, 4, 51),
				Ipv4Addr::new(192, 168, 4, 52),
				Ipv4Addr::new(192, 168, 4, 53),
				Ipv4Addr::new(192, 168, 4, 54),
				Ipv4Addr::new(192, 168, 4, 55),
				Ipv4Addr::new(192, 168, 4, 56),
				Ipv4Addr::new(192, 168, 4, 57),
				Ipv4Addr::new(192, 168, 4, 58),
				Ipv4Addr::new(192, 168, 4, 59),
				Ipv4Addr::new(192, 168, 4, 60),
				Ipv4Addr::new(192, 168, 4, 61),
				Ipv4Addr::new(192, 168, 4, 62),
				Ipv4Addr::new(192, 168, 4, 63),
			],
		);
	}

	#[test]
	fn range_v6_28bit() {
		test_v6(
			"::192.168.4.48/124",
			&[
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 48),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 49),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 50),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 51),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 52),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 53),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 54),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 55),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 56),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 57),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 58),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 59),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 60),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 61),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 62),
				Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 63),
			],
		);
	}
}
