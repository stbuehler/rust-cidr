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
	pub const fn is_ipv4(&self) -> bool {
		match self {
			Self::V4(_) => true,
			Self::V6(_) => false,
		}
	}

	/// Whether representing an IPv6 network
	pub const fn is_ipv6(&self) -> bool {
		match self {
			Self::V4(_) => false,
			Self::V6(_) => true,
		}
	}

	// --- copy of trait api

	/// Create new host within a network from address and prefix length.
	/// If the network length exceeds the address length an error is
	/// returned.
	pub const fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkLengthTooLongError> {
		match addr {
			IpAddr::V4(a) => match Ipv4Inet::new(a, len) {
				Ok(inet) => Ok(Self::V4(inet)),
				Err(e) => Err(e),
			},
			IpAddr::V6(a) => match Ipv6Inet::new(a, len) {
				Ok(inet) => Ok(Self::V6(inet)),
				Err(e) => Err(e),
			},
		}
	}

	/// Create a network containing a single address as host and the
	/// network (network length = address length).
	pub const fn new_host(addr: IpAddr) -> Self {
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

	/// Returns next address in network or `None` if it was the last address in the network
	pub const fn next(self) -> Option<Self> {
		match self {
			Self::V4(c) => match c.next() {
				Some(c) => Some(Self::V4(c)),
				None => None,
			},
			Self::V6(c) => match c.next() {
				Some(c) => Some(Self::V6(c)),
				None => None,
			},
		}
	}

	/// decrements host part (without changing the network part);
	/// returns true on wrap around
	pub fn decrement(&mut self) -> bool {
		match self {
			Self::V4(mut c) => c.decrement(),
			Self::V6(mut c) => c.decrement(),
		}
	}

	/// Returns previous address in network or `None` if it was the first address in the network
	pub const fn previous(self) -> Option<Self> {
		match self {
			Self::V4(c) => match c.previous() {
				Some(c) => Some(Self::V4(c)),
				None => None,
			},
			Self::V6(c) => match c.previous() {
				Some(c) => Some(Self::V6(c)),
				None => None,
			},
		}
	}

	/// Find the nth host after the current one in the current network
	///
	/// Returned boolean indicates whether an overflow occured.
	pub const fn overflowing_add(self, step: u128) -> (Self, bool) {
		match self {
			Self::V4(c) => {
				let (c, overflow) = c.overflowing_add(step);
				(Self::V4(c), overflow)
			},
			Self::V6(c) => {
				let (c, overflow) = c.overflowing_add(step);
				(Self::V6(c), overflow)
			},
		}
	}

	/// Find the nth host before the current one in the current network
	///
	/// Returned boolean indicates whether an overflow occured.
	pub const fn overflowing_sub(self, step: u128) -> (Self, bool) {
		match self {
			Self::V4(c) => {
				let (c, overflow) = c.overflowing_sub(step);
				(Self::V4(c), overflow)
			},
			Self::V6(c) => {
				let (c, overflow) = c.overflowing_sub(step);
				(Self::V6(c), overflow)
			},
		}
	}

	/// network (i.e. drops the host information)
	pub const fn network(&self) -> IpCidr {
		match self {
			Self::V4(c) => IpCidr::V4(c.network()),
			Self::V6(c) => IpCidr::V6(c.network()),
		}
	}

	/// the host
	pub const fn address(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.address()),
			Self::V6(c) => IpAddr::V6(c.address()),
		}
	}

	/// first address in the network as plain address
	pub const fn first_address(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.first_address()),
			Self::V6(c) => IpAddr::V6(c.first_address()),
		}
	}

	/// first address in the network
	pub const fn first(&self) -> Self {
		match self {
			Self::V4(c) => Self::V4(c.first()),
			Self::V6(c) => Self::V6(c.first()),
		}
	}

	/// last address in the network as plain address
	pub const fn last_address(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.last_address()),
			Self::V6(c) => IpAddr::V6(c.last_address()),
		}
	}

	/// last address in the network
	pub const fn last(&self) -> Self {
		match self {
			Self::V4(c) => Self::V4(c.last()),
			Self::V6(c) => Self::V6(c.last()),
		}
	}

	/// length in bits of the shared prefix of the contained addresses
	pub const fn network_length(&self) -> u8 {
		match self {
			Self::V4(c) => c.network_length(),
			Self::V6(c) => c.network_length(),
		}
	}

	/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
	///
	/// [`Ipv4`]: Family::Ipv4
	/// [`Ipv6`]: Family::Ipv6
	pub const fn family(&self) -> Family {
		match self {
			Self::V4(_) => Family::Ipv4,
			Self::V6(_) => Family::Ipv6,
		}
	}

	/// whether network represents a single host address
	pub const fn is_host_address(&self) -> bool {
		match self {
			Self::V4(c) => c.is_host_address(),
			Self::V6(c) => c.is_host_address(),
		}
	}

	/// network mask: an pseudo address which has the first `network
	/// length` bits set to 1 and the remaining to 0.
	pub const fn mask(&self) -> IpAddr {
		match self {
			Self::V4(c) => IpAddr::V4(c.mask()),
			Self::V6(c) => IpAddr::V6(c.mask()),
		}
	}

	/// check whether an address is contained in the network
	pub const fn contains(&self, addr: &IpAddr) -> bool {
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

	fn next(self) -> Option<Self> {
		self.next()
	}

	fn decrement(&mut self) -> bool {
		self.decrement()
	}

	fn previous(self) -> Option<Self> {
		self.previous()
	}

	fn overflowing_add(self, step: u128) -> (Self, bool) {
		self.overflowing_add(step)
	}

	fn overflowing_sub(self, step: u128) -> (Self, bool) {
		self.overflowing_sub(step)
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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl core::ops::Add<u128> for IpInet {
	type Output = IpInet;

	fn add(self, step: u128) -> Self::Output {
		let (result, overflow) = self.overflowing_add(step);
		debug_assert!(!overflow, "{} + {} overflow", self, step);
		result
	}
}

impl core::ops::Sub<u128> for IpInet {
	type Output = IpInet;

	fn sub(self, step: u128) -> Self::Output {
		let (result, overflow) = self.overflowing_sub(step);
		debug_assert!(!overflow, "{} - {} overflow", self, step);
		result
	}
}
