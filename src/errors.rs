//! Various error types returned by function in this crate

use core::{
	fmt,
	num::ParseIntError,
};
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
use std::{
	error::Error,
	net::AddrParseError,
};

use crate::{
	Address,
	AnyIpCidr,
	Family,
	GenericCidr,
	Inet,
	IpCidr,
	IpInet,
	Ipv4Cidr,
	Ipv6Cidr,
};

/// Error returned when the network length was longer than the address
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NetworkLengthTooLongError(usize, Family);

impl NetworkLengthTooLongError {
	pub(crate) const fn new(len: usize, family: Family) -> Self {
		NetworkLengthTooLongError(len, family)
	}
}

impl fmt::Debug for NetworkLengthTooLongError {
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
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
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Error type returned when parsing CIDR networks
#[derive(Clone, PartialEq)]
pub enum CidrParseError<C>
where
	C: GenericCidr,
{
	/// The host part wasn't zero but should have been. The [`Cidr`] types
	/// require that you use the first address in the network (and the
	/// network length) to represent the address, but it wasn't the
	/// first address.
	///
	/// [`Cidr`]: crate::Cidr
	InvalidHostPart {
		/// Address with non-zero host part in network.
		address: <C::Address as Address>::Inet,
	},
	/// More generic network parse error
	NetworkParseError(NetworkParseError),
}

impl<C> CidrParseError<C>
where
	C: GenericCidr,
	<C::Address as Address>::Cidr: Into<C>,
{
	/// Extract cidr when host part was non-zero, return other error otherwise
	///
	/// ```rust
	/// # fn main() -> Result<(), cidr::errors::NetworkParseError> {
	/// let prefix = "192.168.1.1/24".parse::<cidr::AnyIpCidr>().or_else(|e| e.ignore_invalid_host_part())?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn ignore_invalid_host_part(self) -> Result<C, NetworkParseError> {
		match self {
			Self::InvalidHostPart { address } => Ok(address.network().into()),
			Self::NetworkParseError(e) => Err(e),
		}
	}
}

impl<C> CidrParseError<C>
where
	C: GenericCidr,
{
	fn upcast<T>(self) -> CidrParseError<T>
	where
		T: GenericCidr,
		<C::Address as Address>::Inet: Into<<T::Address as Address>::Inet>,
	{
		match self {
			Self::InvalidHostPart { address } => CidrParseError::InvalidHostPart {
				address: address.into(),
			},
			Self::NetworkParseError(e) => CidrParseError::NetworkParseError(e),
		}
	}
}

impl CidrParseError<Ipv4Cidr> {
	pub(crate) const fn const_v4_into(self) -> CidrParseError<IpCidr> {
		match self {
			Self::InvalidHostPart { address } => CidrParseError::InvalidHostPart {
				address: IpInet::V4(address),
			},
			Self::NetworkParseError(e) => CidrParseError::NetworkParseError(e),
		}
	}

	pub(crate) const fn const_v4_into_any(self) -> CidrParseError<AnyIpCidr> {
		match self {
			Self::InvalidHostPart { address } => CidrParseError::InvalidHostPart {
				address: IpInet::V4(address),
			},
			Self::NetworkParseError(e) => CidrParseError::NetworkParseError(e),
		}
	}
}

impl CidrParseError<Ipv6Cidr> {
	pub(crate) const fn const_v6_into(self) -> CidrParseError<IpCidr> {
		match self {
			Self::InvalidHostPart { address } => CidrParseError::InvalidHostPart {
				address: IpInet::V6(address),
			},
			Self::NetworkParseError(e) => CidrParseError::NetworkParseError(e),
		}
	}

	pub(crate) const fn const_v6_into_any(self) -> CidrParseError<AnyIpCidr> {
		match self {
			Self::InvalidHostPart { address } => CidrParseError::InvalidHostPart {
				address: IpInet::V6(address),
			},
			Self::NetworkParseError(e) => CidrParseError::NetworkParseError(e),
		}
	}
}

impl<C> fmt::Debug for CidrParseError<C>
where
	C: GenericCidr,
{
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidHostPart { address } => {
				write!(
					w,
					"host part of address {address} was not zero; did you mean {}?",
					address.network()
				)
			},
			Self::NetworkParseError(e) => e.fmt(w),
		}
	}
}

impl<C> fmt::Display for CidrParseError<C>
where
	C: GenericCidr,
{
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, w)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl<C> Error for CidrParseError<C>
where
	C: GenericCidr,
{
	fn description(&self) -> &str {
		"cidr parse error"
	}

	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::InvalidHostPart { .. } => None,
			Self::NetworkParseError(e) => Some(e),
		}
	}
}

impl<C> From<NetworkParseError> for CidrParseError<C>
where
	C: GenericCidr,
{
	fn from(e: NetworkParseError) -> Self {
		CidrParseError::NetworkParseError(e)
	}
}

impl<C> From<AddrParseError> for CidrParseError<C>
where
	C: GenericCidr,
{
	fn from(e: AddrParseError) -> Self {
		CidrParseError::NetworkParseError(e.into())
	}
}

impl<C> From<ParseIntError> for CidrParseError<C>
where
	C: GenericCidr,
{
	fn from(e: ParseIntError) -> Self {
		CidrParseError::NetworkParseError(e.into())
	}
}

impl From<CidrParseError<Ipv4Cidr>> for CidrParseError<IpCidr> {
	fn from(value: CidrParseError<Ipv4Cidr>) -> Self {
		value.upcast()
	}
}

impl From<CidrParseError<Ipv4Cidr>> for CidrParseError<AnyIpCidr> {
	fn from(value: CidrParseError<Ipv4Cidr>) -> Self {
		value.upcast()
	}
}

impl From<CidrParseError<Ipv6Cidr>> for CidrParseError<IpCidr> {
	fn from(value: CidrParseError<Ipv6Cidr>) -> Self {
		value.upcast()
	}
}

impl From<CidrParseError<Ipv6Cidr>> for CidrParseError<AnyIpCidr> {
	fn from(value: CidrParseError<Ipv6Cidr>) -> Self {
		value.upcast()
	}
}

impl From<CidrParseError<IpCidr>> for CidrParseError<AnyIpCidr> {
	fn from(value: CidrParseError<IpCidr>) -> Self {
		value.upcast()
	}
}

/// Error type returned when parsing IP networks
#[derive(Clone, PartialEq)]
pub enum NetworkParseError {
	/// Failed to parse the address
	AddrParseError(AddrParseError),
	/// Failed to parse the network length
	NetworkLengthParseError(ParseIntError),
	/// The network length was not valid (but was successfully parsed)
	NetworkLengthTooLongError(NetworkLengthTooLongError),
}
impl fmt::Debug for NetworkParseError {
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
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
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
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
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::NotInSharedNetwork => write!(w, "addresses not in shared network"),
			Self::NetworkLengthTooLongError(e) => {
				write!(w, "invalid length for network: {}", e)
			},
		}
	}
}

impl fmt::Display for InetTupleError {
	fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
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
