pub use self::any::AnyIpCidr;

mod any;
mod combined;
mod direct;
mod from_str;
mod serde;

#[cfg(feature = "bitstring")]
#[cfg(test)]
mod bitstring_tests;
#[cfg(test)]
mod tests;

use std::net::{
	Ipv4Addr,
	Ipv6Addr,
};

/// [`Cidr`] type representing an IPv4 network
///
/// Ordering based on lexicographic bitstring representation.
///
/// [`Cidr`]: crate::Cidr
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Cidr {
	pub(crate) address: Ipv4Addr,
	pub(crate) network_length: u8,
}

/// [`Cidr`] type representing an IPv6 network
///
/// Ordering based on lexicographic bitstring representation.
///
/// [`Cidr`]: crate::Cidr
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Cidr {
	pub(crate) address: Ipv6Addr,
	pub(crate) network_length: u8,
}

/// [`Cidr`] type representing either an IPv4 or an IPv6 network
///
/// [`Cidr`]: crate::Cidr
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum IpCidr {
	/// IPv4 network
	V4(Ipv4Cidr),
	/// IPv6 network
	V6(Ipv6Cidr),
}
