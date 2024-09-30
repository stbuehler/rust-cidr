#[cfg(feature = "bitstring")]
use bitstring::FixedBitString;

use core::{
	fmt,
	str::FromStr,
};
use std::net::{
	Ipv4Addr,
	Ipv6Addr,
};

use super::from_str::cidr_from_str;
use crate::{
	errors::*,
	internal_traits::{
		PrivCidr,
		PrivUnspecAddress,
	},
	Cidr,
	Family,
	InetIterator,
	Ipv4Cidr,
	Ipv4Inet,
	Ipv4InetPair,
	Ipv6Cidr,
	Ipv6Inet,
	Ipv6InetPair,
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
			/// Create new network from address and prefix length.  If the
			/// network length exceeds the address length or the address is not
			/// the first address in the network ("host part not zero") an
			/// error is returned.
			pub const fn new(addr: $addr, len: u8) -> Result<Self, NetworkParseError> {
				if len > $family.len() {
					Err(NetworkParseError::NetworkLengthTooLongError(
						NetworkLengthTooLongError::new(len as usize, $family),
					))
				} else if !<$addr as PrivUnspecAddress>::_Tools::_has_zero_host_part(addr, len) {
					Err(NetworkParseError::InvalidHostPart)
				} else {
					Ok(Self {
						address: addr,
						network_length: len,
					})
				}
			}

			/// Create a network containing a single address (network length =
			/// address length).
			pub const fn new_host(addr: $addr) -> Self {
				Self {
					address: addr,
					network_length: $family.len(),
				}
			}

			/// Iterate over all addresses in the range.  With IPv6 addresses
			/// this can produce really long iterations (up to 2<sup>128</sup>
			/// addresses).
			pub const fn iter(&self) -> InetIterator<$addr> {
				self._range_pair().iter()
			}

			/// first address in the network as plain address
			pub const fn first_address(&self) -> $addr {
				self.address
			}

			/// first address in the network
			pub const fn first(&self) -> $inet {
				$inet {
					address: self.first_address(),
					network_length: self.network_length,
				}
			}

			/// last address in the network as plain address
			pub const fn last_address(&self) -> $addr {
				<$addr as PrivUnspecAddress>::_Tools::_last_address(
					self.address,
					self.network_length,
				)
			}

			/// last address in the network
			pub const fn last(&self) -> $inet {
				$inet {
					address: self.last_address(),
					network_length: self.network_length,
				}
			}

			/// length in bits of the shared prefix of the contained addresses
			pub const fn network_length(&self) -> u8 {
				self.network_length
			}

			/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
			///
			/// [`Ipv4`]: Family::Ipv4
			/// [`Ipv6`]: Family::Ipv6
			pub const fn family(&self) -> Family {
				$family
			}

			/// whether network represents a single host address
			pub const fn is_host_address(&self) -> bool {
				self.network_length() == self.family().len()
			}

			/// network mask: an pseudo address which has the first `network
			/// length` bits set to 1 and the remaining to 0.
			pub const fn mask(&self) -> $addr {
				<$addr as PrivUnspecAddress>::_Tools::_network_mask(self.network_length)
			}

			/// check whether an address is contained in the network
			pub const fn contains(&self, addr: &$addr) -> bool {
				<$addr as PrivUnspecAddress>::_Tools::_prefix_match(
					self.address,
					*addr,
					self.network_length,
				)
			}

			pub(crate) const fn _range_pair(&self) -> $pair {
				$pair {
					first: self.first_address(),
					second: self.last_address(),
					network_length: self.network_length,
				}
			}
		}

		impl PrivCidr for $n {}

		impl Cidr for $n {
			type Address = $addr;

			fn new(addr: $addr, len: u8) -> Result<Self, NetworkParseError> {
				Self::new(addr, len)
			}

			fn new_host(addr: $addr) -> Self {
				Self::new_host(addr)
			}

			fn iter(&self) -> InetIterator<$addr> {
				self.iter()
			}

			fn first_address(&self) -> $addr {
				self.first_address()
			}

			fn first(&self) -> $inet {
				self.first()
			}

			fn last_address(&self) -> $addr {
				self.last_address()
			}

			fn last(&self) -> $inet {
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
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "{:?}/{}", self.address, self.network_length)
			}
		}

		impl fmt::Display for $n {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				if f.alternate() || !self.is_host_address() {
					write!(f, "{}/{}", self.address, self.network_length)?;
				} else {
					write!(f, "{}", self.address)?;
				}
				Ok(())
			}
		}

		impl PartialOrd<$n> for $n {
			fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}

		impl Ord for $n {
			fn cmp(&self, other: &Self) -> core::cmp::Ordering {
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

		impl From<$addr> for $n {
			fn from(address: $addr) -> Self {
				Self::new_host(address)
			}
		}

		/// Iterate over all the addresses in the CIDR.
		impl IntoIterator for $n {
			type IntoIter = $crate::InetIterator<$addr>;
			type Item = $inet;

			fn into_iter(self) -> Self::IntoIter {
				self._range_pair().iter()
			}
		}
	};
}

impl_cidr_for! {Ipv4Cidr : inet Ipv4Inet : addr Ipv4Addr : pair Ipv4InetPair : family Family::Ipv4}
impl_cidr_for! {Ipv6Cidr : inet Ipv6Inet : addr Ipv6Addr : pair Ipv6InetPair : family Family::Ipv6}

#[cfg(test)]
mod tests {
	use std::net::Ipv4Addr;

	use crate::Ipv4Cidr;

	fn check_list_iter<T: PartialEq + core::fmt::Debug>(
		data: impl AsRef<[T]>,
		iter: impl IntoIterator<Item = T>,
	) {
		let mut iter = iter.into_iter();
		for elem in data.as_ref() {
			assert_eq!(Some(elem), iter.next().as_ref());
		}
		assert_eq!(None, iter.next());
	}

	#[test]
	fn v4_ref_into_iter() {
		let cidr = Ipv4Cidr::new(Ipv4Addr::new(1, 2, 3, 0), 30).unwrap();
		check_list_iter(
			[
				Ipv4Addr::new(1, 2, 3, 0),
				Ipv4Addr::new(1, 2, 3, 1),
				Ipv4Addr::new(1, 2, 3, 2),
				Ipv4Addr::new(1, 2, 3, 3),
			],
			cidr.into_iter().addresses(),
		);
	}

	#[test]
	fn v4_owned_into_iter() {
		let cidr = Ipv4Cidr::new(Ipv4Addr::new(1, 2, 3, 0), 30).unwrap();
		check_list_iter(
			[
				Ipv4Addr::new(1, 2, 3, 0),
				Ipv4Addr::new(1, 2, 3, 1),
				Ipv4Addr::new(1, 2, 3, 2),
				Ipv4Addr::new(1, 2, 3, 3),
			],
			cidr.into_iter().addresses(),
		);
	}
}
