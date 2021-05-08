use super::traits::Inet;

/// Iterator type to iterate over a list of IP addresses in a network
pub struct InetIterator<I: Inet> {
	next: Option<I>,
}

impl<I: Inet> InetIterator<I> {
	#[doc(hidden)]
	pub fn new(start: I) -> Self {
		InetIterator { next: Some(start) }
	}
}

impl<I: Inet + Clone> Iterator for InetIterator<I> {
	type Item = I::Address;

	fn next(&mut self) -> Option<Self::Item> {
		let mut overflow = false;
		let result = match self.next {
			None => None,
			Some(ref mut next) => {
				let result = Some(next.address());
				overflow = next.next();
				result
			},
		};
		if overflow {
			self.next = None;
		}
		result
	}
}

#[cfg(test)]
mod tests {
	use super::super::*;
	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

	fn test_v4(s: &'static str, l: &[Ipv4Addr]) {
		assert_eq!(s.parse::<Ipv4Cidr>().unwrap().iter().collect::<Vec<_>>(), l);

		assert_eq!(
			s.parse::<IpCidr>().unwrap().iter().collect::<Vec<_>>(),
			l.iter().map(|e| IpAddr::V4(*e)).collect::<Vec<_>>()
		);
	}

	fn test_v6(s: &'static str, l: &[Ipv6Addr]) {
		assert_eq!(s.parse::<Ipv6Cidr>().unwrap().iter().collect::<Vec<_>>(), l);

		assert_eq!(
			s.parse::<IpCidr>().unwrap().iter().collect::<Vec<_>>(),
			l.iter().map(|e| IpAddr::V6(*e)).collect::<Vec<_>>()
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
