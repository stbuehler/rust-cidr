//! Various error types returned by function in this crate

use core::{
	fmt,
	num::ParseIntError,
};
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
use std::error::Error;
use std::net::AddrParseError;

use crate::Family;

/// Error returned when the network length was longer than the address
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NetworkLengthTooLongError(usize, Family);

impl NetworkLengthTooLongError {
	#[doc(hidden)]
	pub fn new(len: usize, family: Family) -> Self {
		NetworkLengthTooLongError(len, family)
	}
}

impl fmt::Debug for NetworkLengthTooLongError {
	fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
		write!(
			w,
			"Network length {} is too long for {:?} (maximum: {})",
			self.0,
			self.1,
			self.1.len()
		)
	}
}
impl fmt::Display for NetworkLengthTooLongError {
	fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, w)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl Error for NetworkLengthTooLongError {
	fn description(&self) -> &str {
		"network length too long"
	}
}

/// Error type returned when parsing IP networks
#[derive(Clone, PartialEq)]
pub enum NetworkParseError {
	/// The host part wasn't zero but should have been. The [`Cidr`] types
	/// require that you use the first address in the network (and the
	/// network length) to represent the address, but it wasn't the
	/// first address.
	///
	/// [`Cidr`]: crate::Cidr
	InvalidHostPart,
	/// Failed to parse the address
	AddrParseError(AddrParseError),
	/// Failed to parse the network length
	NetworkLengthParseError(ParseIntError),
	/// The network length was not valid (but was successfully parsed)
	NetworkLengthTooLongError(NetworkLengthTooLongError),
}
impl fmt::Debug for NetworkParseError {
	fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::InvalidHostPart => write!(w, "host part of address was not zero"),
			Self::AddrParseError(e) => {
				write!(w, "couldn't parse address in network: {}", e)
			},
			Self::NetworkLengthParseError(e) => {
				write!(w, "couldn't parse length in network: {}", e)
			},
			Self::NetworkLengthTooLongError(e) => {
				write!(w, "invalid length for network: {}", e)
			},
		}
	}
}
impl fmt::Display for NetworkParseError {
	fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, w)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl Error for NetworkParseError {
	fn description(&self) -> &str {
		"network parse error"
	}

	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::InvalidHostPart => None,
			Self::AddrParseError(e) => Some(e),
			Self::NetworkLengthParseError(e) => Some(e),
			Self::NetworkLengthTooLongError(e) => Some(e),
		}
	}
}

impl From<AddrParseError> for NetworkParseError {
	fn from(e: AddrParseError) -> Self {
		NetworkParseError::AddrParseError(e)
	}
}

impl From<ParseIntError> for NetworkParseError {
	fn from(e: ParseIntError) -> Self {
		NetworkParseError::NetworkLengthParseError(e)
	}
}

impl From<NetworkLengthTooLongError> for NetworkParseError {
	fn from(e: NetworkLengthTooLongError) -> Self {
		NetworkParseError::NetworkLengthTooLongError(e)
	}
}

/// Error type returned when creating [`Inet`] pair
///
/// [`Inet`]: crate::Inet
#[derive(Clone, PartialEq)]
pub enum InetTupleError {
	/// The given addresses are not in the same network
	NotInSharedNetwork,
	/// The network length was not valid (but was successfully parsed)
	NetworkLengthTooLongError(NetworkLengthTooLongError),
}

impl fmt::Debug for InetTupleError {
	fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NotInSharedNetwork => write!(w, "addresses not in shared network"),
			Self::NetworkLengthTooLongError(e) => {
				write!(w, "invalid length for network: {}", e)
			},
		}
	}
}

impl fmt::Display for InetTupleError {
	fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, w)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl Error for InetTupleError {
	fn description(&self) -> &str {
		"inet tuple error"
	}

	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::NotInSharedNetwork => None,
			Self::NetworkLengthTooLongError(e) => Some(e),
		}
	}
}

impl From<NetworkLengthTooLongError> for InetTupleError {
	fn from(e: NetworkLengthTooLongError) -> Self {
		InetTupleError::NetworkLengthTooLongError(e)
	}
}
