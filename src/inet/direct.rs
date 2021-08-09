#[cfg(feature = "bitstring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "bitstring")))]
use bitstring::FixedBitString;

use core::{
	fmt,
	str::FromStr,
};
use std::net::{
	Ipv4Addr,
	Ipv6Addr,
};

use super::from_str::inet_from_str;
use crate::{
	errors::*,
	internal_traits::{
		PrivAddress,
		PrivInet,
		PrivUnspecAddress,
	},
	Family,
	Inet,
	Ipv4Cidr,
	Ipv4Inet,
	Ipv6Cidr,
	Ipv6Inet,
};

macro_rules! impl_inet_for {
	($n:ident : cidr $cidr:ident : addr $addr:ident : pair $pair:ident : family $family:expr) => {
		#[cfg(feature = "bitstring")]
		#[cfg_attr(doc_cfg, doc(cfg(feature = "bitstring")))]
		impl bitstring::BitString for $n {
			fn get(&self, ndx: usize) -> bool {
				self.address.get(ndx)
			}

			fn set(&mut self, ndx: usize, bit: bool) {
				assert!(ndx < self.network_length as usize);
				self.address.set(ndx, bit);
			}

			fn flip(&mut self, ndx: usize) {
				assert!(ndx < self.network_length as usize);
				self.address.flip(ndx);
			}

			fn len(&self) -> usize {
				self.network_length as usize
			}

			fn clip(&mut self, len: usize) {
				if len > 255 {
					return;
				}
				self.address.set_false_from(len);
				self.network_length = core::cmp::min(self.network_length, len as u8);
			}

			fn append(&mut self, bit: bool) {
				self.address.set(self.network_length as usize, bit);
				self.network_length += 1;
			}

			fn null() -> Self {
				Self {
					address: FixedBitString::new_all_false(),
					network_length: 0,
				}
			}

			fn shared_prefix_len(&self, other: &Self) -> usize {
				let max_len = core::cmp::min(self.network_length, other.network_length) as usize;
				FixedBitString::shared_prefix_len(&self.address, &other.address, max_len)
			}
		}

		impl $n {
			/// Create new host within a network from address and prefix length.
			/// If the network length exceeds the address length an error is
			/// returned.
			pub fn new(addr: $addr, len: u8) -> Result<Self, NetworkLengthTooLongError> {
				if len > $family.len() {
					Err(NetworkLengthTooLongError::new(len as usize, $family).into())
				} else {
					Ok(Self {
						address: addr,
						network_length: len,
					})
				}
			}

			/// Create a network containing a single address as host and the
			/// network (network length = address length).
			pub fn new_host(addr: $addr) -> Self {
				Self {
					address: addr,
					network_length: $family.len(),
				}
			}

			/// increments host part (without changing the network part);
			/// returns true on wrap around
			pub fn increment(&mut self) -> bool {
				let (address, overflow) = self.address._overflowing_next(self.network_length);
				self.address = address;
				overflow
			}

			/// network (i.e. drops the host information)
			pub fn network(&self) -> $cidr {
				$cidr {
					address: self.first_address(),
					network_length: self.network_length,
				}
			}

			/// the host
			pub fn address(&self) -> $addr {
				self.address
			}

			/// first address in the network as plain address
			pub fn first_address(&self) -> $addr {
				self.address._network_address(self.network_length)
			}

			/// first address in the network
			pub fn first(&self) -> Self {
				Self {
					address: self.first_address(),
					network_length: self.network_length,
				}
			}

			/// last address in the network as plain address
			pub fn last_address(&self) -> $addr {
				self.address._last_address(self.network_length)
			}

			/// last address in the network
			pub fn last(&self) -> Self {
				Self {
					address: self.last_address(),
					network_length: self.network_length,
				}
			}

			/// length in bits of the shared prefix of the contained addresses
			pub fn network_length(&self) -> u8 {
				self.network_length
			}

			/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
			///
			/// [`Ipv4`]: Family::Ipv4
			/// [`Ipv6`]: Family::Ipv6
			pub fn family(&self) -> Family {
				$family
			}

			/// whether network represents a single host address
			pub fn is_host_address(&self) -> bool {
				self.network_length() == self.family().len()
			}

			/// network mask: an pseudo address which has the first `network
			/// length` bits set to 1 and the remaining to 0.
			pub fn mask(&self) -> $addr {
				$addr::_network_mask(self.network_length)
			}

			/// check whether an address is contained in the network
			pub fn contains(&self, addr: &$addr) -> bool {
				self.address._prefix_match(*addr, self.network_length)
			}
		}

		impl PrivInet for $n {}

		impl Inet for $n {
			type Address = $addr;

			fn new(addr: $addr, len: u8) -> Result<Self, NetworkLengthTooLongError> {
				Self::new(addr, len)
			}

			fn new_host(addr: $addr) -> Self {
				Self::new_host(addr)
			}

			fn increment(&mut self) -> bool {
				self.increment()
			}

			fn network(&self) -> $cidr {
				self.network()
			}

			fn address(&self) -> $addr {
				self.address()
			}

			fn first_address(&self) -> $addr {
				self.first_address()
			}

			fn first(&self) -> Self {
				self.first()
			}

			fn last_address(&self) -> $addr {
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

			fn mask(&self) -> $addr {
				self.mask()
			}

			fn contains(&self, addr: &$addr) -> bool {
				self.contains(addr)
			}
		}

		impl fmt::Debug for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "{:?}/{}", self.address, self.network_length)
			}
		}

		impl fmt::Display for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				if f.alternate() || !self.is_host_address() {
					write!(f, "{}/{}", self.address, self.network_length)?;
				} else {
					write!(f, "{}", self.address)?;
				}
				Ok(())
			}
		}

		impl FromStr for $n {
			type Err = NetworkParseError;

			fn from_str(s: &str) -> Result<Self, NetworkParseError> {
				inet_from_str(s)
			}
		}
	};
}

impl_inet_for! {Ipv4Inet : cidr Ipv4Cidr : addr Ipv4Addr : pair Ipv4InetPair : family Family::Ipv4}
impl_inet_for! {Ipv6Inet : cidr Ipv6Cidr : addr Ipv6Addr : pair Ipv6InetPair : family Family::Ipv6}
