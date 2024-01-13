mod combined;
mod direct;

use std::net::{
	Ipv4Addr,
	Ipv6Addr,
};

/// [`InetPair`] type representing a pair of IPv4 hosts within a network
///
/// [`InetPair`]: crate::InetPair
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct Ipv4InetPair {
	pub(crate) first: Ipv4Addr,
	pub(crate) second: Ipv4Addr,
	pub(crate) network_length: u8,
}

/// [`InetPair`] type representing a pair of IPv6 hosts within a network
///
/// [`InetPair`]: crate::InetPair
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct Ipv6InetPair {
	pub(crate) first: Ipv6Addr,
	pub(crate) second: Ipv6Addr,
	pub(crate) network_length: u8,
}

/// [`InetPair`] type representing either a pair of IPv4 host or a pair of IPv6
/// hosts within a network
///
/// [`InetPair`]: crate::InetPair
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub enum IpInetPair {
	/// IPv4 host within network
	V4(Ipv4InetPair),
	/// IPv6 host within network
	V6(Ipv6InetPair),
}
