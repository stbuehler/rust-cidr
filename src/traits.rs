use crate::{
	errors::{
		InetTupleError,
		NetworkLengthTooLongError,
		NetworkParseError,
	},
	internal_traits::{
		PrivCidr,
		PrivInet,
		PrivInetPair,
		PrivUnspecAddress,
	},
	num::NumberOfAddresses,
	Family,
	InetIterator,
};
use core::{
	fmt::Debug,
	hash::Hash,
};

/// Maps IP address type to other types based on this address type
///
/// Implemented for [`IPv4Addr`], [`IPv6Addr`] and [`IpAddr`].
///
/// [`Ipv4Addr`]: std::net::Ipv4Addr
/// [`Ipv6Addr`]: std::net::Ipv6Addr
/// [`IpAddr`]: std::net::IpAddr
pub trait Address: Copy + Debug + Ord + Hash + PrivUnspecAddress {
	/// Corresponding [`Inet`] type (representing an address + a network
	/// containing it)
	type Inet: Inet<Address = Self>;

	/// Corresponding [`Cidr`] type (representing only a network, not a specific
	/// address within)
	type Cidr: Cidr<Address = Self>;

	/// Corresponding [`InetPair`] type (representing two addresses in the same network)
	type InetPair: InetPair<Address = Self>;
}

/// Types implementing [`Cidr`] represent IP networks.  An IP network in
/// this case is a set of IP addresses which share a common prefix (when
/// viewed as a bitstring).  The length of this prefix is called
/// `network_length`.
///
/// In the standard representation the network is identified by the
/// first address and the network length, separated by a '/'.
///
/// The parsers will expect the input in the same format, i.e. only the
/// first address of the network is accepted.
///
/// The first network length bits in an address representing the network
/// are the network part, the remaining bits are the host part.
/// Requiring an address to be the first in a network is equivalent to
/// requiring the host part being zero.
pub trait Cidr: Copy + Debug + Ord + Hash + PrivCidr {
	/// Type for the underlying address ([`IpAddr`], [`Ipv4Addr`] or
	/// [`Ipv6Addr`]).
	///
	/// [`Ipv4Addr`]: std::net::Ipv4Addr
	/// [`Ipv6Addr`]: std::net::Ipv6Addr
	/// [`IpAddr`]: std::net::IpAddr
	type Address: Address<Cidr = Self>;

	/// Create new network from address and prefix length.  If the
	/// network length exceeds the address length or the address is not
	/// the first address in the network ("host part not zero") an
	/// error is returned.
	fn new(addr: Self::Address, len: u8) -> Result<Self, NetworkParseError>;

	/// Create a network containing a single address (network length =
	/// address length).
	fn new_host(addr: Self::Address) -> Self;

	/// Iterate over all addresses in the range.  With IPv6 addresses
	/// this can produce really long iterations (up to 2<sup>128</sup>
	/// addresses).
	fn iter(&self) -> InetIterator<Self::Address>;

	/// first address in the network as plain address
	fn first_address(&self) -> Self::Address;
	/// first address in the network
	fn first(&self) -> <Self::Address as Address>::Inet;
	/// last address in the network as plain address
	fn last_address(&self) -> Self::Address;
	/// last address in the network
	fn last(&self) -> <Self::Address as Address>::Inet;
	/// length in bits of the shared prefix of the contained addresses
	fn network_length(&self) -> u8;
	/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
	///
	/// [`Ipv4`]: Family::Ipv4
	/// [`Ipv6`]: Family::Ipv6
	fn family(&self) -> Family;

	/// whether network represents a single host address
	fn is_host_address(&self) -> bool;

	/// network mask: an pseudo address which has the first `network
	/// length` bits set to 1 and the remaining to 0.
	fn mask(&self) -> Self::Address;

	/// check whether an address is contained in the network
	fn contains(&self, addr: &Self::Address) -> bool;

	#[doc(hidden)]
	fn _range_pair(&self) -> <Self::Address as Address>::InetPair;
}

/// Types implementing Inet represent IP hosts within networks.
///
/// In addition to the network represented by the corresponding [`Cidr`]
/// type, an [`Inet`] type also stores a single host address which is part
/// of the network.
///
/// The host address is not really stored as separate data, but is
/// stored together with the network address.
///
/// The representation of a [`Inet`] type is similar to that of the
/// corresponding [`Cidr`] type, but uses the host address instead of the
/// first address of the network.
pub trait Inet: Copy + Debug + Ord + Hash + PrivInet {
	/// Type for the underlying address ([`IpAddr`], [`Ipv4Addr`] or
	/// [`Ipv6Addr`]).
	///
	/// [`Ipv4Addr`]: std::net::Ipv4Addr
	/// [`Ipv6Addr`]: std::net::Ipv6Addr
	/// [`IpAddr`]: std::net::IpAddr
	type Address: Address<Inet = Self>;

	/// Create new host within a network from address and prefix length.
	/// If the network length exceeds the address length an error is
	/// returned.
	fn new(addr: Self::Address, len: u8) -> Result<Self, NetworkLengthTooLongError>;

	/// Create a network containing a single address as host and the
	/// network (network length = address length).
	fn new_host(addr: Self::Address) -> Self;

	/// increments host part (without changing the network part);
	/// returns true on wrap around
	fn increment(&mut self) -> bool;

	/// network (i.e. drops the host information)
	fn network(&self) -> <Self::Address as Address>::Cidr;

	/// the host
	fn address(&self) -> Self::Address;

	/// first address in the network as plain address
	fn first_address(&self) -> Self::Address;
	/// first address in the network
	fn first(&self) -> Self;
	/// last address in the network as plain address
	fn last_address(&self) -> Self::Address;
	/// last address in the network
	fn last(&self) -> Self;
	/// length in bits of the shared prefix of the contained addresses
	fn network_length(&self) -> u8;
	/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
	///
	/// [`Ipv4`]: Family::Ipv4
	/// [`Ipv6`]: Family::Ipv6
	fn family(&self) -> Family;

	/// whether network represents a single host address
	fn is_host_address(&self) -> bool;

	/// network mask: an pseudo address which has the first `network
	/// length` bits set to 1 and the remaining to 0.
	fn mask(&self) -> Self::Address;

	/// check whether an address is contained in the network
	fn contains(&self, addr: &Self::Address) -> bool;
}

/// Pair of two addresses in the same network
pub trait InetPair: Copy + Debug + Eq + Hash + PrivInetPair {
	/// Type for the underlying address ([`IpAddr`], [`Ipv4Addr`] or
	/// [`Ipv6Addr`]).
	///
	/// [`Ipv4Addr`]: std::net::Ipv4Addr
	/// [`Ipv6Addr`]: std::net::Ipv6Addr
	/// [`IpAddr`]: std::net::IpAddr
	type Address: Address<InetPair = Self>;

	/// Create new pair from two addresses in the same network
	///
	/// Fails if the addresses are not in the same network.
	fn new(
		first: <Self::Address as Address>::Inet,
		second: <Self::Address as Address>::Inet,
	) -> Result<Self, InetTupleError>;

	/// Create new pair from two addresses and a common length
	///
	/// Fails if the network length is invalid for the addresses or the addresses are not in the same network.
	fn new_from_addresses(
		first: Self::Address,
		second: Self::Address,
		len: u8,
	) -> Result<Self, InetTupleError>;

	/// First address
	fn first(&self) -> <Self::Address as Address>::Inet;

	/// Second address
	fn second(&self) -> <Self::Address as Address>::Inet;

	/// network (i.e. drops the host information)
	fn network(&self) -> <Self::Address as Address>::Cidr;

	/// length in bits of the shared prefix of the contained addresses
	fn network_length(&self) -> u8;

	/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
	///
	/// [`Ipv4`]: Family::Ipv4
	/// [`Ipv6`]: Family::Ipv6
	fn family(&self) -> Family;

	/// Iterate over `first..=second` (inclusive)
	fn iter(self) -> InetIterator<Self::Address>;

	#[doc(hidden)]
	fn _covered_addresses(&self) -> NumberOfAddresses;

	#[doc(hidden)]
	fn _inc_first(&mut self) -> bool;

	#[doc(hidden)]
	fn _dec_second(&mut self) -> bool;
}
