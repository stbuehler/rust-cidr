use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

use super::super::cidr::*;
use super::super::errors::*;
use super::super::family::Family;
use super::super::traits::*;
use super::from_str::inet_from_str;
use super::{IpInet, Ipv4Inet, Ipv6Inet};

impl IpInet {
	/// Whether representing an IPv4 network
	pub fn is_ipv4(&self) -> bool {
		match *self {
			IpInet::V4(_) => true,
			IpInet::V6(_) => false,
		}
	}

	/// Whether representing an IPv6 network
	pub fn is_ipv6(&self) -> bool {
		match *self {
			IpInet::V4(_) => false,
			IpInet::V6(_) => true,
		}
	}
}

impl Inet for IpInet {
	type Address = IpAddr;
	type Cidr = IpCidr;

	fn new(addr: Self::Address, len: u8) -> Result<Self, NetworkLengthTooLongError> {
		Ok(match addr {
			IpAddr::V4(a) => IpInet::V4(Ipv4Inet::new(a, len)?),
			IpAddr::V6(a) => IpInet::V6(Ipv6Inet::new(a, len)?),
		})
	}

	fn new_host(addr: Self::Address) -> Self {
		match addr {
			IpAddr::V4(a) => IpInet::V4(Ipv4Inet::new_host(a)),
			IpAddr::V6(a) => IpInet::V6(Ipv6Inet::new_host(a)),
		}
	}

	fn next(&mut self) -> bool {
		match *self {
			IpInet::V4(ref mut c) => c.next(),
			IpInet::V6(ref mut c) => c.next(),
		}
	}

	fn network(&self) -> Self::Cidr {
		match *self {
			IpInet::V4(ref c) => IpCidr::V4(c.network()),
			IpInet::V6(ref c) => IpCidr::V6(c.network()),
		}
	}

	fn address(&self) -> Self::Address {
		match *self {
			IpInet::V4(ref c) => IpAddr::V4(c.address()),
			IpInet::V6(ref c) => IpAddr::V6(c.address()),
		}
	}

	fn first_address(&self) -> Self::Address {
		match *self {
			IpInet::V4(ref c) => IpAddr::V4(c.first_address()),
			IpInet::V6(ref c) => IpAddr::V6(c.first_address()),
		}
	}

	fn first(&self) -> Self {
		match *self {
			IpInet::V4(ref c) => IpInet::V4(c.first()),
			IpInet::V6(ref c) => IpInet::V6(c.first()),
		}
	}

	fn last_address(&self) -> Self::Address {
		match *self {
			IpInet::V4(ref c) => IpAddr::V4(c.last_address()),
			IpInet::V6(ref c) => IpAddr::V6(c.last_address()),
		}
	}

	fn last(&self) -> Self {
		match *self {
			IpInet::V4(ref c) => IpInet::V4(c.last()),
			IpInet::V6(ref c) => IpInet::V6(c.last()),
		}
	}

	fn network_length(&self) -> u8 {
		match *self {
			IpInet::V4(ref c) => c.network_length(),
			IpInet::V6(ref c) => c.network_length(),
		}
	}

	fn family(&self) -> Family {
		match *self {
			IpInet::V4(_) => Family::Ipv4,
			IpInet::V6(_) => Family::Ipv6,
		}
	}

	fn mask(&self) -> Self::Address {
		match *self {
			IpInet::V4(ref c) => IpAddr::V4(c.mask()),
			IpInet::V6(ref c) => IpAddr::V6(c.mask()),
		}
	}

	fn contains(&self, addr: &Self::Address) -> bool {
		match *self {
			IpInet::V4(ref c) => match *addr {
				IpAddr::V4(ref a) => c.contains(a),
				IpAddr::V6(_) => false,
			},
			IpInet::V6(ref c) => match *addr {
				IpAddr::V4(_) => false,
				IpAddr::V6(ref a) => c.contains(a),
			},
		}
	}
}

impl fmt::Display for IpInet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			IpInet::V4(ref c) => fmt::Display::fmt(c, f),
			IpInet::V6(ref c) => fmt::Display::fmt(c, f),
		}
	}
}

impl FromStr for IpInet {
	type Err = NetworkParseError;
	fn from_str(s: &str) -> Result<IpInet, NetworkParseError> {
		inet_from_str(s)
	}
}

impl From<Ipv4Inet> for IpInet {
	fn from(c: Ipv4Inet) -> Self {
		IpInet::V4(c)
	}
}

impl From<Ipv6Inet> for IpInet {
	fn from(c: Ipv6Inet) -> Self {
		IpInet::V6(c)
	}
}
