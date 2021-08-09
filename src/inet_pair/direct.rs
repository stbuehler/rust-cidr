use core::fmt;
use std::net::{
	Ipv4Addr,
	Ipv6Addr,
};

use crate::{
	errors::*,
	internal_traits::{
		PrivInetPair,
		PrivUnspecAddress,
	},
	num::NumberOfAddresses,
	Family,
	InetIterator,
	InetPair,
	Ipv4Cidr,
	Ipv4Inet,
	Ipv4InetPair,
	Ipv6Cidr,
	Ipv6Inet,
	Ipv6InetPair,
};

macro_rules! impl_inet_pair_for {
	($n:ident : inet $inet:ident : cidr $cidr:ident : addr $addr:ty : native $native:ident : family $family:expr) => {
		impl $n {
			/// Create new pair from two addresses in the same network
			///
			/// Fails if the addresses are not in the same network.
			pub fn new(first: $inet, second: $inet) -> Result<Self, InetTupleError> {
				if first.network_length != second.network_length {
					return Err(InetTupleError::NotInSharedNetwork);
				}
				if !first
					.address
					._prefix_match(second.address, first.network_length)
				{
					return Err(InetTupleError::NotInSharedNetwork);
				}
				Ok(Self {
					first: first.address,
					second: second.address,
					network_length: first.network_length,
				})
			}

			/// Create new pair from two addresses and a common length
			///
			/// Fails if the network length is invalid for the addresses or the addresses are not in the same network.
			pub fn new_from_addresses(
				first: $addr,
				second: $addr,
				len: u8,
			) -> Result<Self, InetTupleError> {
				if !first._prefix_match(second, len) {
					return Err(InetTupleError::NotInSharedNetwork);
				}
				Ok(Self {
					first,
					second,
					network_length: len,
				})
			}

			/// First address
			pub fn first(&self) -> $inet {
				$inet {
					address: self.first,
					network_length: self.network_length,
				}
			}

			/// Second address
			pub fn second(&self) -> $inet {
				$inet {
					address: self.second,
					network_length: self.network_length,
				}
			}

			/// network (i.e. drops the host information)
			pub fn network(&self) -> $cidr {
				self.first().network()
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

			/// Iterate over `first..=second` (inclusive)
			pub fn iter(self) -> InetIterator<$addr> {
				InetIterator::_new(self)
			}
		}

		impl PrivInetPair for $n {}

		impl InetPair for $n {
			type Address = $addr;

			fn new(first: $inet, second: $inet) -> Result<Self, InetTupleError> {
				Self::new(first, second)
			}

			fn new_from_addresses(
				first: Self::Address,
				second: Self::Address,
				len: u8,
			) -> Result<Self, InetTupleError> {
				Self::new_from_addresses(first, second, len)
			}

			fn first(&self) -> $inet {
				self.first()
			}

			fn second(&self) -> $inet {
				self.second()
			}

			fn network(&self) -> $cidr {
				self.network()
			}

			fn network_length(&self) -> u8 {
				self.network_length()
			}

			fn family(&self) -> Family {
				self.family()
			}

			fn iter(self) -> InetIterator<$addr> {
				self.iter()
			}

			fn _covered_addresses(&self) -> NumberOfAddresses {
				let first = <$native>::from(self.first);
				let second = <$native>::from(self.second);
				NumberOfAddresses::count_from_distance((second - first) as u128)
			}

			fn _inc_first(&mut self) -> bool {
				if self.first < self.second {
					self.first = <$addr>::from(<$native>::from(self.first) + 1);
					true
				} else {
					false
				}
			}

			fn _dec_second(&mut self) -> bool {
				if self.first < self.second {
					self.second = <$addr>::from(<$native>::from(self.second) - 1);
					true
				} else {
					false
				}
			}
		}

		impl fmt::Debug for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				f.debug_struct(stringify!($n))
					.field("first", &self.first())
					.field("second", &self.second())
					.finish()
			}
		}

		impl fmt::Display for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				use core::fmt::Write;
				f.write_char('(')?;
				self.first().fmt(f)?;
				f.write_str(", ")?;
				self.second().fmt(f)?;
				f.write_char(')')?;
				Ok(())
			}
		}

		/// Iterate over all the addresses in the CIDR.
		impl IntoIterator for $n {
			type IntoIter = crate::InetIterator<$addr>;
			type Item = $inet;

			fn into_iter(self) -> Self::IntoIter {
				self.iter()
			}
		}
	};
}

impl_inet_pair_for! {Ipv4InetPair : inet Ipv4Inet : cidr Ipv4Cidr : addr Ipv4Addr : native u32  : family Family::Ipv4}
impl_inet_pair_for! {Ipv6InetPair : inet Ipv6Inet : cidr Ipv6Cidr : addr Ipv6Addr : native u128 : family Family::Ipv6}
