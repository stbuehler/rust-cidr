use core::{
	fmt,
	str::FromStr,
};
use std::net::IpAddr;

use super::from_str::inet_from_str;
use crate::{
	errors::*,
	internal_traits::PrivInet,
	Family,
	Inet,
	IpCidr,
	IpInet,
	Ipv4Inet,
	Ipv6Inet,
};

impl IpInet {
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

	// --- copy of trait api

	/// Create new host within a network from address and prefix length.
	/// If the network length exceeds the address length an error is
	/// returned.
	pub fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkLengthTooLongError> {
		Ok(match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Inet::new(a, len)?),
			IpAddr::V6(a) => Self::V6(Ipv6Inet::new(a, len)?),
		})
	}

	/// Create a network containing a single address as host and the
	/// network (network length = address length).
	pub fn new_host(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Inet::new_host(a)),
			IpAddr::V6(a) => Self::V6(Ipv6Inet::new_host(a)),
		}
	}

	/// increments host part (without changing the network part);
	/// returns true on wrap around
	pub fn increment(&mut self) -> bool {
		match self {
			Self::V4(mut c) => c.increment(),
			Self::V6(mut c) => c.increment(),
		}
	}

	/// network (i.e. drops the host information)
	pub fn network(&self) -> IpCidr {
		match self {
			Self::V4(c) => IpCidr::V4(c.network()),
			Self::V6(c) => IpCidr::V6(c.network()),
		}
	}

	/// the host
	pub fn address(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.address()),
			Self::V6(c) => IpAddr::V6(c.address()),
		}
	}

	/// first address in the network as plain address
	pub fn first_address(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.first_address()),
			Self::V6(c) => IpAddr::V6(c.first_address()),
		}
	}

	/// first address in the network
	pub fn first(&self) -> Self {
		match self {
			Self::V4(c) => Self::V4(c.first()),
			Self::V6(c) => Self::V6(c.first()),
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
	pub fn last(&self) -> Self {
		match self {
			Self::V4(c) => Self::V4(c.last()),
			Self::V6(c) => Self::V6(c.last()),
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

impl PrivInet for IpInet {}

impl Inet for IpInet {
	type Address = IpAddr;

	fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkLengthTooLongError> {
		Self::new(addr, len)
	}

	fn new_host(addr: IpAddr) -> Self {
		Self::new_host(addr)
	}

	fn increment(&mut self) -> bool {
		self.increment()
	}

	fn network(&self) -> IpCidr {
		self.network()
	}

	fn address(&self) -> IpAddr {
		self.address()
	}

	fn first_address(&self) -> IpAddr {
		self.first_address()
	}

	fn first(&self) -> Self {
		self.first()
	}

	fn last_address(&self) -> IpAddr {
		self.last_address()
	}

	fn last(&self) -> Self {
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
}

impl fmt::Display for IpInet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::V4(c) => fmt::Display::fmt(c, f),
			Self::V6(c) => fmt::Display::fmt(c, f),
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
