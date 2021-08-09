use core::{
	fmt,
	str::FromStr,
};
use std::net::IpAddr;

use super::from_str::cidr_from_str;
use crate::{
	errors::*,
	internal_traits::PrivCidr,
	Cidr,
	Family,
	InetIterator,
	IpCidr,
	IpInet,
	IpInetPair,
	Ipv4Cidr,
	Ipv6Cidr,
};

impl IpCidr {
	/// Whether representing an IPv4 network
	pub fn is_ipv4(&self) -> bool {
		match self {
			Self::V4(_) => true,
			Self::V6(_) => false,
		}
	}

	/// Whether representing an IPv6 network
	pub fn is_ipv6(&self) -> bool {
		match self {
			Self::V4(_) => false,
			Self::V6(_) => true,
		}
	}

	// --- copy of trait API

	/// Create new network from address and prefix length.  If the
	/// network length exceeds the address length or the address is not
	/// the first address in the network ("host part not zero") an
	/// error is returned.
	pub fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkParseError> {
		Ok(match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Cidr::new(a, len)?),
			IpAddr::V6(a) => Self::V6(Ipv6Cidr::new(a, len)?),
		})
	}

	/// Create a network containing a single address (network length =
	/// address length).
	pub fn new_host(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Cidr::new_host(a)),
			IpAddr::V6(a) => Self::V6(Ipv6Cidr::new_host(a)),
		}
	}

	/// Iterate over all addresses in the range.  With IPv6 addresses
	/// this can produce really long iterations (up to 2<sup>128</sup>
	/// addresses).
	pub fn iter(&self) -> InetIterator<IpAddr> {
		self._range_pair().iter()
	}

	/// first address in the network as plain address
	pub fn first_address(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.first_address()),
			Self::V6(c) => IpAddr::V6(c.first_address()),
		}
	}

	/// first address in the network
	pub fn first(&self) -> IpInet {
		match self {
			Self::V4(c) => IpInet::V4(c.first()),
			Self::V6(c) => IpInet::V6(c.first()),
		}
	}

	/// last address in the network as plain address
	pub fn last_address(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.last_address()),
			Self::V6(c) => IpAddr::V6(c.last_address()),
		}
	}

	/// last address in the network
	pub fn last(&self) -> IpInet {
		match self {
			Self::V4(c) => IpInet::V4(c.last()),
			Self::V6(c) => IpInet::V6(c.last()),
		}
	}

	/// length in bits of the shared prefix of the contained addresses
	pub fn network_length(&self) -> u8 {
		match self {
			Self::V4(c) => c.network_length(),
			Self::V6(c) => c.network_length(),
		}
	}

	/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
	///
	/// [`Ipv4`]: Family::Ipv4
	/// [`Ipv6`]: Family::Ipv6
	pub fn family(&self) -> Family {
		match self {
			Self::V4(_) => Family::Ipv4,
			Self::V6(_) => Family::Ipv6,
		}
	}

	/// whether network represents a single host address
	pub fn is_host_address(&self) -> bool {
		match self {
			Self::V4(c) => c.is_host_address(),
			Self::V6(c) => c.is_host_address(),
		}
	}

	/// network mask: an pseudo address which has the first `network
	/// length` bits set to 1 and the remaining to 0.
	pub fn mask(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.mask()),
			Self::V6(c) => IpAddr::V6(c.mask()),
		}
	}

	/// check whether an address is contained in the network
	pub fn contains(&self, addr: &IpAddr) -> bool {
		match self {
			Self::V4(c) => match addr {
				IpAddr::V4(a) => c.contains(a),
				IpAddr::V6(_) => false,
			},
			Self::V6(c) => match addr {
				IpAddr::V4(_) => false,
				IpAddr::V6(a) => c.contains(a),
			},
		}
	}
}

impl PrivCidr for IpCidr {}

impl Cidr for IpCidr {
	type Address = IpAddr;

	fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkParseError> {
		Self::new(addr, len)
	}

	fn new_host(addr: IpAddr) -> Self {
		Self::new_host(addr)
	}

	fn iter(&self) -> InetIterator<IpAddr> {
		self.iter()
	}

	fn first_address(&self) -> IpAddr {
		self.first_address()
	}

	fn first(&self) -> IpInet {
		self.first()
	}

	fn last_address(&self) -> IpAddr {
		self.last_address()
	}

	fn last(&self) -> IpInet {
		self.last()
	}

	fn network_length(&self) -> u8 {
		self.network_length()
	}

	fn family(&self) -> Family {
		self.family()
	}

	fn is_host_address(&self) -> bool {
		self.is_host_address()
	}

	fn mask(&self) -> IpAddr {
		self.mask()
	}

	fn contains(&self, addr: &IpAddr) -> bool {
		self.contains(addr)
	}

	fn _range_pair(&self) -> IpInetPair {
		match self {
			Self::V4(c) => IpInetPair::V4(c._range_pair()),
			Self::V6(c) => IpInetPair::V6(c._range_pair()),
		}
	}
}

impl fmt::Display for IpCidr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::V4(c) => fmt::Display::fmt(c, f),
			Self::V6(c) => fmt::Display::fmt(c, f),
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
	type Item = IpInet;

	fn into_iter(self) -> Self::IntoIter {
		InetIterator::_new(self._range_pair())
	}
}
