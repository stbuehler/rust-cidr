use std::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
};

/// Represents the type of an IP address
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Family {
	/// IPv4
	Ipv4,
	/// IPv6
	Ipv6,
}

impl Family {
	/// The length of an address (as bitstring) in the given family
	#[allow(clippy::len_without_is_empty)]
	pub fn len(&self) -> u8 {
		match self {
			Self::Ipv4 => 32,
			Self::Ipv6 => 128,
		}
	}

	/// The "unspecified" address (all zero) of the given family
	pub fn unspecified_address(&self) -> IpAddr {
		match self {
			Self::Ipv4 => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
			Self::Ipv6 => IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
		}
	}

	/// The "loopback" address (`127.0.0.1` or `::1`) of the given family
	pub fn loopback_address(&self) -> IpAddr {
		match self {
			Self::Ipv4 => IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
			Self::Ipv6 => IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
		}
	}
}
