use std::error::Error;
use std::fmt;
use std::net::AddrParseError;
use std::num::ParseIntError;

use super::Family;

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

impl Error for NetworkLengthTooLongError {
	fn description(&self) -> &str {
		"network length too long"
	}
}

/// Error type returned when parsing IP networks
#[derive(Clone, PartialEq)]
pub enum NetworkParseError {
	/// The host part wasn't zero but should have been. The `Cidr` types
	/// require that you use the first address in the network (and the
	/// network length) to represent the address, but it wasn't the
	/// first address.
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
		match *self {
			NetworkParseError::InvalidHostPart => {
				write!(w, "host part of address was not zero")
			},
			NetworkParseError::AddrParseError(ref e) => {
				write!(w, "couldn't parse address in network: {}", e)
			},
			NetworkParseError::NetworkLengthParseError(ref e) => {
				write!(w, "couldn't parse length in network: {}", e)
			},
			NetworkParseError::NetworkLengthTooLongError(ref e) => {
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

impl Error for NetworkParseError {
	fn description(&self) -> &str {
		"network parse error"
	}

	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			NetworkParseError::InvalidHostPart => None,
			NetworkParseError::AddrParseError(ref e) => Some(e),
			NetworkParseError::NetworkLengthParseError(ref e) => Some(e),
			NetworkParseError::NetworkLengthTooLongError(ref e) => Some(e),
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
