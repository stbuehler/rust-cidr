use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

use super::from_str::cidr_from_str;
use crate::{
	errors::*, internal_traits::PrivCidr, Cidr, Family, InetIterator, IpCidr, IpInet, IpInetPair,
	Ipv4Cidr, Ipv6Cidr,
};

impl IpCidr {
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

impl PrivCidr for IpCidr {}

impl Cidr for IpCidr {
	type Address = IpAddr;

	fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkParseError> {
		Ok(match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Cidr::new(a, len)?),
			IpAddr::V6(a) => Self::V6(Ipv6Cidr::new(a, len)?),
		})
	}

	fn new_host(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Cidr::new_host(a)),
			IpAddr::V6(a) => Self::V6(Ipv6Cidr::new_host(a)),
		}
	}

	fn first_address(&self) -> IpAddr {
		match *self {
			Self::V4(ref c) => IpAddr::V4(c.first_address()),
			Self::V6(ref c) => IpAddr::V6(c.first_address()),
		}
	}

	fn first(&self) -> IpInet {
		match *self {
			Self::V4(ref c) => IpInet::V4(c.first()),
			Self::V6(ref c) => IpInet::V6(c.first()),
		}
	}

	fn last_address(&self) -> IpAddr {
		match *self {
			Self::V4(ref c) => IpAddr::V4(c.last_address()),
			Self::V6(ref c) => IpAddr::V6(c.last_address()),
		}
	}

	fn last(&self) -> IpInet {
		match *self {
			Self::V4(ref c) => IpInet::V4(c.last()),
			Self::V6(ref c) => IpInet::V6(c.last()),
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

	fn _range_pair(&self) -> IpInetPair {
		match *self {
			Self::V4(c) => IpInetPair::V4(c._range_pair()),
			Self::V6(c) => IpInetPair::V6(c._range_pair()),
		}
	}
}

impl fmt::Display for IpCidr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::V4(ref c) => fmt::Display::fmt(c, f),
			Self::V6(ref c) => fmt::Display::fmt(c, f),
		}
	}
}

impl FromStr for IpCidr {
	type Err = NetworkParseError;
	fn from_str(s: &str) -> Result<Self, NetworkParseError> {
		cidr_from_str(s)
	}
}

impl From<Ipv4Cidr> for IpCidr {
	fn from(c: Ipv4Cidr) -> Self {
		Self::V4(c)
	}
}

impl From<Ipv6Cidr> for IpCidr {
	fn from(c: Ipv6Cidr) -> Self {
		Self::V6(c)
	}
}

impl IntoIterator for IpCidr {
	type IntoIter = InetIterator<IpAddr>;
	type Item = IpAddr;

	fn into_iter(self) -> Self::IntoIter {
		InetIterator::_new(self._range_pair())
	}
}

impl<'a> IntoIterator for &'a IpCidr {
	type IntoIter = InetIterator<IpAddr>;
	type Item = IpAddr;

	fn into_iter(self) -> Self::IntoIter {
		InetIterator::_new(self._range_pair())
	}
}
