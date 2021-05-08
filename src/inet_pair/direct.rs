use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};

use super::super::errors::*;
use super::super::family::Family;
use super::super::inet::*;
use super::super::internal_traits::*;
use super::super::num::NumberOfAddresses;
use super::super::traits::*;
use super::{Ipv4InetPair, Ipv6InetPair};

macro_rules! impl_inet_pair_for {
	($n:ident : inet $inet:ident : addr $addr:ty : native $native:ident : family $family:expr) => {
		impl HasAddressType for $n {
			type Address = $addr;
		}

		impl PrivInetPair for $n {
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

		impl InetPair for $n {
			fn new(
				first: Self::Address,
				second: Self::Address,
				len: u8,
			) -> Result<Self, InetTupleError> {
				if !first._prefix_match(second, len) {
					return Err(InetTupleError::NotInSharedNetwork);
				}
				Ok(Self { first, second, network_length: len })
			}

			fn first(&self) -> $inet {
				$inet { address: self.first, network_length: self.network_length }
			}

			fn second(&self) -> $inet {
				$inet { address: self.second, network_length: self.network_length }
			}

			fn network(&self) -> <Self::Address as Address>::Cidr {
				self.first().network()
			}

			fn network_length(&self) -> u8 {
				self.network_length
			}

			fn family(&self) -> Family {
				$family
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
				use std::fmt::Write;
				f.write_char('(')?;
				self.first().fmt(f)?;
				f.write_str(", ")?;
				self.second().fmt(f)?;
				f.write_char(')')?;
				Ok(())
			}
		}
	};
}

impl_inet_pair_for! {Ipv4InetPair : inet Ipv4Inet : addr Ipv4Addr : native u32  : family Family::Ipv4}
impl_inet_pair_for! {Ipv6InetPair : inet Ipv6Inet : addr Ipv6Addr : native u128 : family Family::Ipv6}
