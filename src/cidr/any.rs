use core::{
	fmt,
	str::FromStr,
};
use std::net::IpAddr;

use super::from_str::cidr_from_str;
use crate::{
	errors::*,
	Cidr,
	Family,
	IpCidr,
	IpInet,
	Ipv4Cidr,
	Ipv6Cidr,
};

/// Represents either an IPv4 or an IPv6 network or "any".
///
/// Allows for a bit string representation which treats "any" as the
/// empty string, IPv4 as starting with `false` and IPv6 as starting
/// with `true`. After the first bit the normal represenation for the
/// picked address-family follows.
///
/// Setting the first bit (using the `bitstring` API) always truncates
/// the bit string to length 1 (i.e. `/0` in the resulting family).
///
/// The [`Cidr`] trait cannot be implemented for this type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AnyIpCidr {
	/// "any" network containing all IPv4 and IPv6 addresses
	Any,
	/// IPv4 network
	V4(Ipv4Cidr),
	/// IPv6 network
	V6(Ipv6Cidr),
}

impl AnyIpCidr {
	/// Whether representing any address
	pub fn is_any(&self) -> bool {
		match self {
			Self::Any => true,
			_ => false,
		}
	}

	/// Whether representing an IPv4 network
	pub fn is_ipv4(&self) -> bool {
		match self {
			Self::V4(_) => true,
			_ => false,
		}
	}

	/// Whether representing an IPv6 network
	pub fn is_ipv6(&self) -> bool {
		match self {
			Self::V4(_) => false,
			_ => true,
		}
	}
}

// "Cidr" functions
impl AnyIpCidr {
	/// Create new network from address and prefix length.  If the
	/// network length exceeds the address length or the address is not
	/// the first address in the network ("host part not zero") an error
	/// is returned.
	pub fn new(addr: IpAddr, len: u8) -> Result<Self, NetworkParseError> {
		Ok(match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Cidr::new(a, len)?),
			IpAddr::V6(a) => Self::V6(Ipv6Cidr::new(a, len)?),
		})
	}

	/// Create a network containing a single address (network length =
	/// address length).
	pub fn new_host(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(a) => Self::V4(Ipv4Cidr::new_host(a)),
			IpAddr::V6(a) => Self::V6(Ipv6Cidr::new_host(a)),
		}
	}

	/// first address in the network as plain address
	///
	/// returns [`None`] for [`Any`]
	///
	/// [`Any`]: Self::Any
	pub fn first_address(&self) -> Option<IpAddr> {
		match self {
			Self::Any => None,
			Self::V4(c) => Some(IpAddr::V4(c.first_address())),
			Self::V6(c) => Some(IpAddr::V6(c.first_address())),
		}
	}

	/// first address in the network
	///
	/// returns [`None`] for [`Any`]
	///
	/// [`Any`]: Self::Any
	pub fn first(&self) -> Option<IpInet> {
		match self {
			Self::Any => None,
			Self::V4(c) => Some(IpInet::V4(c.first())),
			Self::V6(c) => Some(IpInet::V6(c.first())),
		}
	}

	/// last address in the network as plain address
	///
	/// returns [`None`] for [`Any`]
	///
	/// [`Any`]: Self::Any
	pub fn last_address(&self) -> Option<IpAddr> {
		match self {
			Self::Any => None,
			Self::V4(c) => Some(IpAddr::V4(c.last_address())),
			Self::V6(c) => Some(IpAddr::V6(c.last_address())),
		}
	}

	/// last address in the network
	///
	/// returns [`None`] for [`Any`]
	///
	/// [`Any`]: Self::Any
	pub fn last(&self) -> Option<IpInet> {
		match self {
			Self::Any => None,
			Self::V4(c) => Some(IpInet::V4(c.last())),
			Self::V6(c) => Some(IpInet::V6(c.last())),
		}
	}

	/// length in bits of the shared prefix of the contained addresses
	///
	/// returns [`None`] for [`Any`]
	///
	/// [`Any`]: Self::Any
	pub fn network_length(&self) -> Option<u8> {
		match self {
			Self::Any => None,
			Self::V4(c) => Some(c.network_length()),
			Self::V6(c) => Some(c.network_length()),
		}
	}

	/// IP family of the contained address ([`Ipv4`] or [`Ipv6`]).
	///
	/// returns [`None`] for [`Any`]
	///
	/// [`Any`]: Self::Any
	/// [`Ipv4`]: Family::Ipv4
	/// [`Ipv6`]: Family::Ipv6
	pub fn family(&self) -> Option<Family> {
		match self {
			Self::Any => None,
			Self::V4(_) => Some(Family::Ipv4),
			Self::V6(_) => Some(Family::Ipv6),
		}
	}

	/// whether network represents a single host address
	pub fn is_host_address(&self) -> bool {
		match self {
			Self::Any => false,
			Self::V4(c) => c.is_host_address(),
			Self::V6(c) => c.is_host_address(),
		}
	}

	/// network mask: an pseudo address which has the first `network
	/// length` bits set to 1 and the remaining to 0.
	///
	/// returns [`None`] for [`Any`]
	///
	/// [`Any`]: Self::Any
	pub fn mask(&self) -> Option<IpAddr> {
		match self {
			Self::Any => None,
			Self::V4(c) => Some(IpAddr::V4(c.mask())),
			Self::V6(c) => Some(IpAddr::V6(c.mask())),
		}
	}

	/// check whether an address is contained in the network
	pub fn contains(&self, addr: &IpAddr) -> bool {
		match self {
			Self::Any => true,
			Self::V4(c) => match addr {
				IpAddr::V4(a) => c.contains(a),
				IpAddr::V6(_) => false,
			},
			Self::V6(c) => match addr {
				IpAddr::V4(_) => false,
				IpAddr::V6(a) => c.contains(a),
			},
		}
	}
}

impl fmt::Display for AnyIpCidr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any => write!(f, "any"),
			Self::V4(c) => fmt::Display::fmt(c, f),
			Self::V6(c) => fmt::Display::fmt(c, f),
		}
	}
}

impl From<AnyIpCidr> for Option<IpCidr> {
	fn from(value: AnyIpCidr) -> Option<IpCidr> {
		match value {
			AnyIpCidr::Any => None,
			AnyIpCidr::V4(c) => Some(IpCidr::V4(c)),
			AnyIpCidr::V6(c) => Some(IpCidr::V6(c)),
		}
	}
}

impl From<Option<IpCidr>> for AnyIpCidr {
	fn from(a: Option<IpCidr>) -> Self {
		match a {
			None => Self::Any,
			Some(IpCidr::V4(c)) => Self::V4(c),
			Some(IpCidr::V6(c)) => Self::V6(c),
		}
	}
}

impl FromStr for AnyIpCidr {
	type Err = NetworkParseError;

	fn from_str(s: &str) -> Result<Self, NetworkParseError> {
		if s == "any" {
			Ok(Self::Any)
		} else {
			cidr_from_str::<IpCidr>(s).map(Self::from)
		}
	}
}

impl From<IpCidr> for AnyIpCidr {
	fn from(c: IpCidr) -> Self {
		match c {
			IpCidr::V4(c) => Self::V4(c),
			IpCidr::V6(c) => Self::V6(c),
		}
	}
}

impl From<Ipv4Cidr> for AnyIpCidr {
	fn from(c: Ipv4Cidr) -> Self {
		Self::V4(c)
	}
}

impl From<Ipv6Cidr> for AnyIpCidr {
	fn from(c: Ipv6Cidr) -> Self {
		Self::V6(c)
	}
}

#[cfg(feature = "bitstring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "bitstring")))]
impl bitstring::BitString for AnyIpCidr {
	fn get(&self, ndx: usize) -> bool {
		assert!(!self.is_any());
		if 0 == ndx {
			self.is_ipv6()
		} else {
			match self {
				Self::Any => unreachable!(),
				Self::V4(c) => c.get(ndx - 1),
				Self::V6(c) => c.get(ndx - 1),
			}
		}
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		assert!(!self.is_any());
		if 0 == ndx {
			if bit {
				*self = Self::V6(Ipv6Cidr::null());
			} else {
				*self = Self::V4(Ipv4Cidr::null());
			}
		} else {
			match self {
				Self::Any => unreachable!(),
				Self::V4(ref mut c) => c.set(ndx - 1, bit),
				Self::V6(ref mut c) => c.set(ndx - 1, bit),
			}
		}
	}

	fn flip(&mut self, ndx: usize) {
		assert!(!self.is_any());
		if 0 == ndx {
			if self.is_ipv6() {
				*self = Self::V4(Ipv4Cidr::null())
			} else {
				*self = Self::V6(Ipv6Cidr::null())
			}
		} else {
			match self {
				Self::Any => unreachable!(),
				Self::V4(ref mut c) => c.flip(ndx - 1),
				Self::V6(ref mut c) => c.flip(ndx - 1),
			}
		}
	}

	fn len(&self) -> usize {
		match self {
			Self::Any => 0,
			Self::V4(c) => c.len() + 1,
			Self::V6(c) => c.len() + 1,
		}
	}

	fn clip(&mut self, len: usize) {
		// max length is 129 (len(IPv6) + 1)
		if len > 128 {
			return;
		}
		if 0 == len {
			*self = Self::Any;
		} else {
			match self {
				Self::Any => (),
				Self::V4(ref mut c) => c.clip(len - 1),
				Self::V6(ref mut c) => c.clip(len - 1),
			}
		}
	}

	fn append(&mut self, bit: bool) {
		match self {
			Self::Any => {
				if bit {
					*self = Self::V6(Ipv6Cidr::null());
				} else {
					*self = Self::V4(Ipv4Cidr::null());
				}
			},
			Self::V4(ref mut c) => c.append(bit),
			Self::V6(ref mut c) => c.append(bit),
		}
	}

	fn null() -> Self {
		Self::Any
	}

	fn shared_prefix_len(&self, other: &Self) -> usize {
		match (self, other) {
			(Self::V4(a), Self::V4(b)) => 1 + a.shared_prefix_len(b),
			(Self::V6(a), Self::V6(b)) => 1 + a.shared_prefix_len(b),
			_ => 0,
		}
	}
}
