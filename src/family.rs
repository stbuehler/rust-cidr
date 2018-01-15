use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Represents the type of an IP address
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Family {
	/// IPv4
	Ipv4,
	/// IPv6
	Ipv6,
}
pub use Family::*;

impl Family {
	/// The length of an address in the given family
	pub fn len(&self) -> u8 {
		match *self {
			Ipv4 => 32,
			Ipv6 => 128,
		}
	}

	/// The "unspecified" address (all zero) of the given family
	pub fn unspecified_address(&self) -> IpAddr {
		match *self {
			Ipv4 => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
			Ipv6 => IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
		}
	}

	/// The "loopback" address (`127.0.0.1` or `::1`) of the given family
	pub fn loopback_address(&self) -> IpAddr {
		match *self {
			Ipv4 => IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
			Ipv6 => IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
		}
	}
}
