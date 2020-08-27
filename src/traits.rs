use super::errors::*;
use super::family::Family;
use super::inet_iterator::*;

/// Types implementing Cidr represent IP networks.  An IP network in
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
pub trait Cidr: Sized {
	/// Type for the underlying address (`IpAddr`, `Ipv4Addr` or
	/// `Ipv6Addr`).
	type Address;

	/// Corresponding `Inet` type (representing an address + a network
	/// containing it)
	type Inet: Inet<Address = Self::Address>;

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
	fn iter(&self) -> InetIterator<Self::Inet> {
		InetIterator::new(self.first())
	}

	/// first address in the network as plain address
	fn first_address(&self) -> Self::Address;
	/// first address in the network
	fn first(&self) -> Self::Inet;
	/// last address in the network as plain address
	fn last_address(&self) -> Self::Address;
	/// last address in the network
	fn last(&self) -> Self::Inet;
	/// length in bits of the shared prefix of the contained addresses
	fn network_length(&self) -> u8;
	/// IP family of the contained address (`Ipv4` or `Ipv6`).
	fn family(&self) -> Family;

	/// whether network represents a single host address
	fn is_host_address(&self) -> bool {
		self.network_length() == self.family().len()
	}

	/// network mask: an pseudo address which has the first `network
	/// length` bits set to 1 and the remaining to 0.
	fn mask(&self) -> Self::Address;

	/// check whether an address is contained in the network
	fn contains(&self, addr: &Self::Address) -> bool;
}

/// Types implementing Inet represent IP hosts within networks.
///
/// In addition to a network represented by the corresponding `Cidr`
/// type, a `Inet` type also stores a single host address which is part
/// of the network.
///
/// The host address is not really stored as separate data, but is
/// stored together with the network address.
///
/// The representation of a `Inet` type is similar to that of the
/// corresponding `Cidr` type, but shows the host address instead of the
/// first address of the network.
pub trait Inet: Sized {
	/// Type for the underlying address (`IpAddr`, `Ipv4Addr` or
	/// `Ipv6Addr`).
	type Address;

	/// Corresponding `Cidr` type (representing only the network)
	type Cidr: Cidr<Address = Self::Address>;

	/// Create new host within a network from address and prefix length.
	/// If the network length exceeds the address length an error is
	/// returned.
	fn new(addr: Self::Address, len: u8) -> Result<Self, NetworkLengthTooLongError>;

	/// Create a network containing a single address as host and the
	/// network (network length = address length).
	fn new_host(addr: Self::Address) -> Self;

	/// increments host part (without changing the network part);
	/// returns true on wrap around
	fn next(&mut self) -> bool;

	/// network (i.e. drops the host information)
	fn network(&self) -> Self::Cidr;

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
	/// IP family of the contained address (`Ipv4` or `Ipv6`).
	fn family(&self) -> Family;

	/// whether network represents a single host address
	fn is_host_address(&self) -> bool {
		self.network_length() == self.family().len()
	}

	/// network mask: an pseudo address which has the first `network
	/// length` bits set to 1 and the remaining to 0.
	fn mask(&self) -> Self::Address;

	/// check whether an address is contained in the network
	fn contains(&self, addr: &Self::Address) -> bool;
}
