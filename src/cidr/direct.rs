#[cfg(feature = "bitstring")]
use bitstring::FixedBitString;

use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use super::from_str::cidr_from_str;
use crate::{
	errors::*,
	internal_traits::{PrivAddress, PrivCidr, PrivUnspecAddress},
	Cidr, Family, Ipv4Cidr, Ipv4Inet, Ipv4InetPair, Ipv6Cidr, Ipv6Inet, Ipv6InetPair,
};

macro_rules! impl_cidr_for {
	($n:ident : inet $inet:ident : addr $addr:ident : pair $pair:ident : family $family:expr) => {
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

		impl PrivCidr for $n {}

		impl Cidr for $n {
			type Address = $addr;

			fn new(addr: $addr, len: u8) -> Result<Self, NetworkParseError> {
				if len > $family.len() {
					Err(NetworkLengthTooLongError::new(len as usize, $family).into())
				} else if !addr._has_zero_host_part(len) {
					Err(NetworkParseError::InvalidHostPart)
				} else {
					Ok(Self { address: addr, network_length: len })
				}
			}

			fn new_host(addr: $addr) -> Self {
				Self { address: addr, network_length: $family.len() }
			}

			fn first_address(&self) -> $addr {
				self.address
			}

			fn first(&self) -> $inet {
				$inet { address: self.first_address(), network_length: self.network_length }
			}

			fn last_address(&self) -> $addr {
				self.address._last_address(self.network_length)
			}

			fn last(&self) -> $inet {
				$inet { address: self.last_address(), network_length: self.network_length }
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

			fn _range_pair(&self) -> $pair {
				$pair {
					first: self.first_address(),
					second: self.last_address(),
					network_length: self.network_length,
				}
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

		impl PartialOrd<$n> for $n {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}

		impl Ord for $n {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				self.address
					.cmp(&other.address)
					.then(self.network_length.cmp(&other.network_length))
			}
		}

		impl FromStr for $n {
			type Err = NetworkParseError;
			fn from_str(s: &str) -> Result<$n, NetworkParseError> {
				cidr_from_str(s)
			}
		}

		/// Iterate over all the addresses in the CIDR.
		impl IntoIterator for $n {
			type IntoIter = $crate::InetIterator<$addr>;
			type Item = $addr;

			fn into_iter(self) -> Self::IntoIter {
				$crate::InetIterator::_new(self._range_pair())
			}
		}

		/// Iterate over all the addresses in the CIDR.
		impl<'a> IntoIterator for &'a $n {
			type IntoIter = $crate::InetIterator<$addr>;
			type Item = $addr;

			fn into_iter(self) -> Self::IntoIter {
				$crate::InetIterator::_new(self._range_pair())
			}
		}
	};
}

impl_cidr_for! {Ipv4Cidr : inet Ipv4Inet : addr Ipv4Addr : pair Ipv4InetPair : family Family::Ipv4}
impl_cidr_for! {Ipv6Cidr : inet Ipv6Inet : addr Ipv6Addr : pair Ipv6InetPair : family Family::Ipv6}

#[cfg(test)]
mod tests {
	use std::net::Ipv4Addr;

	use crate::{Cidr, Ipv4Cidr};

	#[test]
	fn v4_ref_into_iter() {
		let cidr = Ipv4Cidr::new(Ipv4Addr::new(1, 2, 3, 0), 30).unwrap();
		assert_eq!(
			vec![
				Ipv4Addr::new(1, 2, 3, 0),
				Ipv4Addr::new(1, 2, 3, 1),
				Ipv4Addr::new(1, 2, 3, 2),
				Ipv4Addr::new(1, 2, 3, 3),
			],
			(&cidr).into_iter().collect::<Vec<_>>()
		);
	}

	#[test]
	fn v4_owned_into_iter() {
		let cidr = Ipv4Cidr::new(Ipv4Addr::new(1, 2, 3, 0), 30).unwrap();
		assert_eq!(
			vec![
				Ipv4Addr::new(1, 2, 3, 0),
				Ipv4Addr::new(1, 2, 3, 1),
				Ipv4Addr::new(1, 2, 3, 2),
				Ipv4Addr::new(1, 2, 3, 3),
			],
			cidr.into_iter().collect::<Vec<_>>()
		);
	}
}
