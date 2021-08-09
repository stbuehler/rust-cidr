mod combined;
mod direct;
mod from_str;
mod serde;

#[cfg(test)]
mod tests;

use std::net::{
	Ipv4Addr,
	Ipv6Addr,
};

/// [`Inet`] type representing an IPv4 host within a network
///
/// Derived ordering, i.e. first sort by address, then by network
/// length.
///
/// [`Inet`]: crate::Inet
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ipv4Inet {
	pub(crate) address: Ipv4Addr,
	pub(crate) network_length: u8,
}

/// [`Inet`] type representing an IPv6 host within a network
///
/// Derived ordering, i.e. first sort by address, then by network
/// length.
///
/// [`Inet`]: crate::Inet
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ipv6Inet {
	pub(crate) address: Ipv6Addr,
	pub(crate) network_length: u8,
}

/// [`Inet`] type representing either an IPv4 or an IPv6 host within a
/// network
///
/// [`Inet`]: crate::Inet
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum IpInet {
	/// IPv4 host within network
	V4(Ipv4Inet),
	/// IPv6 host within network
	V6(Ipv6Inet),
}
