use bitstring::*;
use std::cmp::{min, Ordering};
use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use super::{Ipv4Cidr, Ipv6Cidr};
use super::from_str::cidr_from_str;
use super::super::errors::*;
use super::super::family::Family;
use super::super::inet::*;
use super::super::traits::*;

macro_rules! impl_cidr_for {
	($n:ident : inet $inet:ident : addr $addr:ty : family $family:expr) => (
		impl BitString for $n {
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
				if len > 255 { return; }
				self.address.set_false_from(len);
				self.network_length = min(self.network_length, len as u8);
			}

			fn append(&mut self, bit: bool) {
				self.address.set(self.network_length as usize, bit);
				self.network_length += 1;
			}

			fn null() -> Self {
				$n{
					address: FixedBitString::new_all_false(),
					network_length: 0,
				}
			}

			fn shared_prefix_len(&self, other: &Self) -> usize {
				let max_len = min(self.network_length, other.network_length) as usize;
				FixedBitString::shared_prefix_len(&self.address, &other.address, max_len)
			}
		}

		impl Cidr for $n {
			type Address = $addr;
			type Inet = $inet;

			fn new(addr: Self::Address, len: u8) -> Result<Self, NetworkParseError> {
				if len > $family.len() {
					Err(NetworkLengthTooLongError::new(len as usize, $family).into())
				} else if !addr.is_false_from(len as usize) {
					Err(NetworkParseError::InvalidHostPart)
				} else {
					Ok($n{
						address: addr,
						network_length: len,
					})
				}
			}

			fn new_host(addr: Self::Address) -> Self {
				$n{
					address: addr,
					network_length: $family.len(),
				}
			}

			fn first_address(&self) -> Self::Address {
				self.address.clone()
			}

			fn first(&self) -> Self::Inet {
				$inet::new(self.first_address(), self.network_length).unwrap()
			}

			fn last_address(&self) -> Self::Address {
				let mut a = self.address.clone();
				a.set_true_from(self.network_length as usize);
				a
			}

			fn last(&self) -> Self::Inet {
				$inet::new(self.last_address(), self.network_length).unwrap()
			}

			fn network_length(&self) -> u8 {
				self.network_length
			}

			fn family(&self) -> Family {
				$family
			}

			fn mask(&self) -> Self::Address {
				let mut a = Self::Address::new_all_true();
				a.set_false_from(self.network_length as usize);
				a
			}

			fn contains(&self, addr: &Self::Address) -> bool {
				self.address.contains(self.network_length as usize, addr)
			}
		}

		impl fmt::Debug for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "{:?}/{}", self.address, self.network_length)
			}
		}

		impl fmt::Display for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "{}", self.address)?;
				if self.network_length != $family.len() {
					write!(f, "/{}", self.network_length)?;
				}
				Ok(())
			}
		}

		impl PartialOrd<$n> for $n {
			fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
				Some(self.lexicographic_cmp(other))
			}
		}

		impl Ord for $n {
			fn cmp(&self, other: &Self) -> Ordering {
				self.lexicographic_cmp(other)
			}
		}

		impl FromStr for $n {
			type Err = NetworkParseError;
			fn from_str(s: &str) -> Result<$n, NetworkParseError> {
				cidr_from_str(s)
			}
		}
	)
}

impl_cidr_for!{Ipv4Cidr : inet Ipv4Inet : addr Ipv4Addr : family Family::Ipv4}
impl_cidr_for!{Ipv6Cidr : inet Ipv6Inet : addr Ipv6Addr : family Family::Ipv6}
