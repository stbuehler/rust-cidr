#[cfg(feature = "bitstring")]
use bitstring::FixedBitString;

use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use super::from_str::inet_from_str;
use crate::{
	errors::*,
	internal_traits::{PrivAddress, PrivInet, PrivUnspecAddress},
	Family, HasAddressType, Inet, Ipv4Cidr, Ipv4Inet, Ipv6Cidr, Ipv6Inet,
};

macro_rules! impl_inet_for {
	($n:ident : cidr $cidr:ident : addr $addr:ident : pair $pair:ident : family $family:expr) => {
		#[cfg(feature = "bitstring")]
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
				self.network_length = std::cmp::min(self.network_length, len as u8);
			}

			fn append(&mut self, bit: bool) {
				self.address.set(self.network_length as usize, bit);
				self.network_length += 1;
			}

			fn null() -> Self {
				Self { address: FixedBitString::new_all_false(), network_length: 0 }
			}

			fn shared_prefix_len(&self, other: &Self) -> usize {
				let max_len = std::cmp::min(self.network_length, other.network_length) as usize;
				FixedBitString::shared_prefix_len(&self.address, &other.address, max_len)
			}
		}

		impl HasAddressType for $n {
			type Address = $addr;
		}

		impl PrivInet for $n {}

		impl Inet for $n {
			fn new(addr: $addr, len: u8) -> Result<Self, NetworkLengthTooLongError> {
				if len > $family.len() {
					Err(NetworkLengthTooLongError::new(len as usize, $family).into())
				} else {
					Ok(Self { address: addr, network_length: len })
				}
			}

			fn new_host(addr: $addr) -> Self {
				Self { address: addr, network_length: $family.len() }
			}

			fn next(&mut self) -> bool {
				let (address, overflow) = self.address._overflowing_next(self.network_length);
				self.address = address;
				overflow
			}

			fn network(&self) -> $cidr {
				$cidr { address: self.first_address(), network_length: self.network_length }
			}

			fn address(&self) -> $addr {
				self.address
			}

			fn first_address(&self) -> $addr {
				self.address._network_address(self.network_length)
			}

			fn first(&self) -> Self {
				Self { address: self.first_address(), network_length: self.network_length }
			}

			fn last_address(&self) -> $addr {
				self.address._last_address(self.network_length)
			}

			fn last(&self) -> Self {
				Self { address: self.last_address(), network_length: self.network_length }
			}

			fn network_length(&self) -> u8 {
				self.network_length
			}

			fn family(&self) -> Family {
				$family
			}

			fn mask(&self) -> $addr {
				$addr::_network_mask(self.network_length)
			}

			fn contains(&self, addr: &$addr) -> bool {
				self.address._prefix_match(*addr, self.network_length)
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
