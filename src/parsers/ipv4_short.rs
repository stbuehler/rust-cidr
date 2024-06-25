use core::str::FromStr;
use std::net::{
	IpAddr,
	Ipv4Addr,
};

use crate::{
	errors::NetworkParseError,
	AnyIpCidr,
	IpCidr,
	Ipv4Cidr,
};

// parse normal IPv4 addresses as host cidr, and short forms
// with a cidr that indicates how many octets were present.
fn _parse_short_ipv4_address_as_cidr(s: &str) -> Option<Ipv4Cidr> {
	let mut octets = [0u8; 4];
	let mut last = 0;
	for (ndx, os) in s.split('.').enumerate() {
		if ndx >= 4 {
			// too many octets
			return None;
		}
		// abort on invalid octet
		octets[ndx] = os.parse().ok()?;
		last = ndx;
	}
	let bits = (last as u8 + 1) * 8;
	Some(Ipv4Cidr::new(Ipv4Addr::from(octets), bits).expect("host bits are zero"))
}

/// Parse "short" IPv4 addresses as networks with octet-aligned length
///
/// * parse `"10"` as `10.0.0.0/8`
/// * parse `"192.168"` as `192.168.0.0/16`
/// * parse `"192.0.2"` as `192.0.2.0/24`
/// * parse `"127"` as `127.0.0.0/8`
/// * parse `"127.0.0.1"` as `127.0.0.1/32`
///
/// The returned prefix length indicates how many octets were present.
///
/// This is very different from [`inet_addr`][`super::inet_addr`] which would
/// interpret `"192.168"` as `192.0.0.168`!
///
/// This function doesn't accept normal CIDR notations, so you will probably
/// need to combine it with other functions.
pub fn parse_short_ipv4_address_as_cidr(s: &str) -> Result<Ipv4Cidr, NetworkParseError> {
	match _parse_short_ipv4_address_as_cidr(s) {
		Some(n) => Ok(n),
		None => {
			// address parser should fail here (to generate proper error result)
			// (but if it works the result should actually be correct)
			Ok(Ipv4Cidr::new_host(s.parse()?))
		},
	}
}

/// Parses normal IPv4 CIDR notations or short forms via [`parse_short_ipv4_address_as_cidr`]
pub fn parse_short_ipv4_cidr(s: &str) -> Result<Ipv4Cidr, NetworkParseError> {
	super::parse_cidr_full(s, FromStr::from_str, parse_short_ipv4_address_as_cidr)
}

/// Parses normal IP addresses as host addresses, and short IPv4 addresses via [`parse_short_ipv4_address_as_cidr`]
///
/// This function doesn't accept normal CIDR notations, so you will probably
/// need to combine it with other functions.
pub fn parse_short_ip_address_as_cidr(s: &str) -> Result<IpCidr, NetworkParseError> {
	match s.parse::<IpAddr>() {
		Ok(a) => Ok(IpCidr::new_host(a)),
		// only try short IPv4 as fallback
		Err(e) => match _parse_short_ipv4_address_as_cidr(s) {
			Some(n) => Ok(n.into()),
			None => Err(e.into()),
		},
	}
}

/// Parses normal IP CIDR notations or short IPv4 forms via [`parse_short_ipv4_address_as_cidr`]
pub fn parse_short_ip_cidr(s: &str) -> Result<IpCidr, NetworkParseError> {
	super::parse_cidr_full(s, FromStr::from_str, parse_short_ip_address_as_cidr)
}

/// Parses normal IP CIDR notations, `"any"` or short IPv4 forms via [`parse_short_ipv4_address_as_cidr`]
pub fn parse_short_any_ip_cidr(s: &str) -> Result<AnyIpCidr, NetworkParseError> {
	super::parse_any_cidr_full(s, FromStr::from_str, parse_short_ip_address_as_cidr)
}

#[cfg(test)]
mod tests {
	use super::{
		parse_short_ip_cidr,
		parse_short_ipv4_cidr,
	};
	use crate::{
		IpCidr,
		Ipv4Cidr,
	};

	fn test(s: &str, expect: &str) {
		// check against standard parser of canonical form
		let expect_v4 = expect.parse::<Ipv4Cidr>().unwrap();
		let expect = expect.parse::<IpCidr>().unwrap();

		assert_eq!(parse_short_ipv4_cidr(s).unwrap(), expect_v4);

		assert_eq!(parse_short_ip_cidr(s).unwrap(), expect);
	}

	#[test]
	fn invalid_short() {
		assert!(parse_short_ipv4_cidr("").is_err());
		assert!(parse_short_ip_cidr("").is_err());
	}

	#[test]
	fn short_10() {
		test("10.0.0.0/8", "10.0.0.0/8");
		test("10", "10.0.0.0/8");
		test("10.42.0.0/16", "10.42.0.0/16");
		test("10.42", "10.42.0.0/16");
		test("10.0.42.0/24", "10.0.42.0/24");
		test("10.0.42", "10.0.42.0/24");
		test("10.0.0.42/32", "10.0.0.42/32");
		test("10.0.0.42", "10.0.0.42/32");
	}

	#[test]
	fn short_192_168() {
		test("192.168.0.0/16", "192.168.0.0/16");
		test("192.168", "192.168.0.0/16");
		test("192.168.42.0/24", "192.168.42.0/24");
		test("192.168.42", "192.168.42.0/24");
		test("192.168.0.42/32", "192.168.0.42/32");
		test("192.168.0.42", "192.168.0.42/32");
	}

	#[test]
	fn short_192_0_2() {
		test("192.0.2.0/24", "192.0.2.0/24");
		test("192.0.2", "192.0.2.0/24");
		test("192.0.2.42/32", "192.0.2.42/32");
		test("192.0.2.42", "192.0.2.42/32");
	}
}
