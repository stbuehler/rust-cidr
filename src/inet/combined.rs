use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

use super::from_str::inet_from_str;
use crate::{
	errors::*, internal_traits::PrivInet, Family, Inet, IpCidr, IpInet, Ipv4Inet, Ipv6Inet,
};

impl IpInet {
	/// Whether representing an IPv4 network
	pub fn is_ipv4(&self) -> bool {
		match *self {
			Self::V4(_) => true,
			Self::V6(_) => false,
		}
	}

	/// Whether representing an IPv6 network
	pub fn is_ipv6(&self) -> bool {
		match *self {
			Self::V4(_) => false,
			Self::V6(_) => true,
		}
	}
}

impl PrivInet for IpInet {}

impl Inet for IpInet {
	type Address = IpAddr;

	fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkLengthTooLongError> {
		Ok(match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Inet::new(a, len)?),
			IpAddr::V6(a) => Self::V6(Ipv6Inet::new(a, len)?),
		})
	}

	fn new_host(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Inet::new_host(a)),
			IpAddr::V6(a) => Self::V6(Ipv6Inet::new_host(a)),
		}
	}

	fn next(&mut self) -> bool {
		match *self {
			Self::V4(ref mut c) => c.next(),
			Self::V6(ref mut c) => c.next(),
		}
	}

	fn network(&self) -> IpCidr {
		match *self {
			Self::V4(ref c) => IpCidr::V4(c.network()),
			Self::V6(ref c) => IpCidr::V6(c.network()),
		}
	}

	fn address(&self) -> IpAddr {
		match *self {
			Self::V4(ref c) => IpAddr::V4(c.address()),
			Self::V6(ref c) => IpAddr::V6(c.address()),
		}
	}

	fn first_address(&self) -> IpAddr {
		match *self {
			Self::V4(ref c) => IpAddr::V4(c.first_address()),
			Self::V6(ref c) => IpAddr::V6(c.first_address()),
		}
	}

	fn first(&self) -> Self {
		match *self {
			Self::V4(ref c) => Self::V4(c.first()),
			Self::V6(ref c) => Self::V6(c.first()),
		}
	}

	fn last_address(&self) -> IpAddr {
		match *self {
			Self::V4(ref c) => IpAddr::V4(c.last_address()),
			Self::V6(ref c) => IpAddr::V6(c.last_address()),
		}
	}

	fn last(&self) -> Self {
		match *self {
			Self::V4(ref c) => Self::V4(c.last()),
			Self::V6(ref c) => Self::V6(c.last()),
		}
	}

	fn network_length(&self) -> u8 {
		match *self {
			Self::V4(ref c) => c.network_length(),
			Self::V6(ref c) => c.network_length(),
		}
	}

	fn family(&self) -> Family {
		match *self {
			Self::V4(_) => Family::Ipv4,
			Self::V6(_) => Family::Ipv6,
		}
	}

	fn mask(&self) -> IpAddr {
		match *self {
			Self::V4(ref c) => IpAddr::V4(c.mask()),
			Self::V6(ref c) => IpAddr::V6(c.mask()),
		}
	}

	fn contains(&self, addr: &IpAddr) -> bool {
		match *self {
			Self::V4(ref c) => match *addr {
				IpAddr::V4(ref a) => c.contains(a),
				IpAddr::V6(_) => false,
			},
			Self::V6(ref c) => match *addr {
				IpAddr::V4(_) => false,
				IpAddr::V6(ref a) => c.contains(a),
			},
		}
	}
}

impl fmt::Display for IpInet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::V4(ref c) => fmt::Display::fmt(c, f),
			Self::V6(ref c) => fmt::Display::fmt(c, f),
		}
	}
}

impl FromStr for IpInet {
	type Err = NetworkParseError;
	fn from_str(s: &str) -> Result<Self, NetworkParseError> {
		inet_from_str(s)
	}
}

impl From<Ipv4Inet> for IpInet {
	fn from(c: Ipv4Inet) -> Self {
		Self::V4(c)
	}
}

impl From<Ipv6Inet> for IpInet {
	fn from(c: Ipv6Inet) -> Self {
		Self::V6(c)
	}
}
