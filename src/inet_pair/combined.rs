use std::fmt;
use std::net::IpAddr;

use super::super::cidr::*;
use super::super::errors::*;
use super::super::family::Family;
use super::super::inet::*;
use super::super::internal_traits::*;
use super::super::num::NumberOfAddresses;
use super::super::traits::*;
use super::{IpInetPair, Ipv4InetPair, Ipv6InetPair};

impl IpInetPair {
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

impl HasAddressType for IpInetPair {
	type Address = IpAddr;
}

impl PrivInetPair for IpInetPair {
	fn _covered_addresses(&self) -> NumberOfAddresses {
		match self {
			Self::V4(p) => p._covered_addresses(),
			Self::V6(p) => p._covered_addresses(),
		}
	}

	fn _inc_first(&mut self) -> bool {
		match self {
			Self::V4(p) => p._inc_first(),
			Self::V6(p) => p._inc_first(),
		}
	}

	fn _dec_second(&mut self) -> bool {
		match self {
			Self::V4(p) => p._dec_second(),
			Self::V6(p) => p._dec_second(),
		}
	}
}

impl InetPair for IpInetPair {
	fn new(first: Self::Address, second: Self::Address, len: u8) -> Result<Self, InetTupleError> {
		match (first, second) {
			(IpAddr::V4(first), IpAddr::V4(second)) => {
				Ok(Self::V4(Ipv4InetPair::new(first, second, len)?))
			},
			(IpAddr::V6(first), IpAddr::V6(second)) => {
				Ok(Self::V6(Ipv6InetPair::new(first, second, len)?))
			},
			_ => Err(InetTupleError::NotInSharedNetwork),
		}
	}

	fn first(&self) -> IpInet {
		match self {
			Self::V4(p) => IpInet::V4(p.first()),
			Self::V6(p) => IpInet::V6(p.first()),
		}
	}

	fn second(&self) -> IpInet {
		match self {
			Self::V4(p) => IpInet::V4(p.second()),
			Self::V6(p) => IpInet::V6(p.second()),
		}
	}

	fn network(&self) -> IpCidr {
		match self {
			Self::V4(p) => IpCidr::V4(p.network()),
			Self::V6(p) => IpCidr::V6(p.network()),
		}
	}

	fn network_length(&self) -> u8 {
		match self {
			Self::V4(p) => p.network_length(),
			Self::V6(p) => p.network_length(),
		}
	}

	fn family(&self) -> Family {
		match *self {
			Self::V4(_) => Family::Ipv4,
			Self::V6(_) => Family::Ipv6,
		}
	}
}

impl fmt::Display for IpInetPair {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::V4(ref c) => fmt::Display::fmt(c, f),
			Self::V6(ref c) => fmt::Display::fmt(c, f),
		}
	}
}

impl From<Ipv4InetPair> for IpInetPair {
	fn from(c: Ipv4InetPair) -> Self {
		Self::V4(c)
	}
}

impl From<Ipv6InetPair> for IpInetPair {
	fn from(c: Ipv6InetPair) -> Self {
		Self::V6(c)
	}
}
