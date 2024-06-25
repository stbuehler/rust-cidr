use core::str::FromStr;
use std::net::{
	AddrParseError,
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
};

pub trait ParseableAddress: Sized {
	fn address_from_str(s: &str) -> Result<Self, AddrParseError>;
}

fn special_ipv4_parser(s: &str) -> Option<Ipv4Addr> {
	Some(
		crate::parsers::parse_short_ipv4_address_as_cidr(s)
			.ok()?
			.first_address(),
	)
}

impl ParseableAddress for Ipv4Addr {
	fn address_from_str(s: &str) -> Result<Self, AddrParseError> {
		match FromStr::from_str(s) {
			Ok(addr) => Ok(addr),
			Err(err) => match special_ipv4_parser(s) {
				Some(addr) => Ok(addr),
				None => Err(err),
			},
		}
	}
}

impl ParseableAddress for Ipv6Addr {
	fn address_from_str(s: &str) -> Result<Self, AddrParseError> {
		FromStr::from_str(s)
	}
}

impl ParseableAddress for IpAddr {
	fn address_from_str(s: &str) -> Result<Self, AddrParseError> {
		match FromStr::from_str(s) {
			Ok(addr) => Ok(addr),
			Err(err) => match special_ipv4_parser(s) {
				Some(addr) => Ok(Self::V4(addr)),
				None => Err(err),
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::ParseableAddress;
	use std::net::{
		IpAddr,
		Ipv4Addr,
	};

	fn test_addr(s: &str, a: Ipv4Addr) {
		assert_eq!(
			Ipv4Addr::address_from_str(s).unwrap(),
			a,
			"{} didn't match {:?} (through Ipv4Addr)",
			s,
			a
		);

		assert_eq!(
			IpAddr::address_from_str(s).unwrap(),
			IpAddr::V4(a),
			"{} didn't match {:?} (through IpAddr)",
			s,
			a
		);
	}

	#[test]
	fn invalid_short() {
		assert!(IpAddr::address_from_str("").is_err());
		assert!(Ipv4Addr::address_from_str("").is_err());
	}

	#[test]
	fn short_10() {
		test_addr("10", Ipv4Addr::new(10, 0, 0, 0));
		test_addr("10.0", Ipv4Addr::new(10, 0, 0, 0));
		test_addr("10.0.0", Ipv4Addr::new(10, 0, 0, 0));
		test_addr("10.0.0.0", Ipv4Addr::new(10, 0, 0, 0));
	}

	#[test]
	fn short_192_168() {
		test_addr("192.168", Ipv4Addr::new(192, 168, 0, 0));
		test_addr("192.168.0", Ipv4Addr::new(192, 168, 0, 0));
		test_addr("192.168.0.0", Ipv4Addr::new(192, 168, 0, 0));
	}

	#[test]
	fn short_192_0_2() {
		test_addr("192.0.2", Ipv4Addr::new(192, 0, 2, 0));
		test_addr("192.0.2.0", Ipv4Addr::new(192, 0, 2, 0));
	}
}
