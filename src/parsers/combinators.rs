use std::net::{
	AddrParseError,
	IpAddr,
};

use crate::{
	errors::NetworkParseError,
	Address,
	AnyIpCidr,
	Cidr,
	Inet,
	IpInet,
};

/// Parse [`Cidr`] with custom address and network (when no '/' separator was found) parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse with `host_parser`.
pub fn parse_cidr_full<C, AP, NP>(
	s: &str,
	address_parser: AP,
	host_parser: NP,
) -> Result<C, NetworkParseError>
where
	C: Cidr,
	AP: FnOnce(&str) -> Result<C::Address, AddrParseError>,
	NP: FnOnce(&str) -> Result<C, NetworkParseError>,
{
	match s.rfind('/') {
		None => host_parser(s),
		Some(pos) => C::new(address_parser(&s[0..pos])?, s[pos + 1..].parse()?),
	}
}

/// Parse [`Cidr`] with custom address parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse `address_parser` and treat as host (maximum prefix length).
pub fn parse_cidr<C, AP>(s: &str, address_parser: AP) -> Result<C, NetworkParseError>
where
	C: Cidr,
	AP: Fn(&str) -> Result<C::Address, AddrParseError>,
{
	parse_cidr_full(s, &address_parser, |s| Ok(C::new_host(address_parser(s)?)))
}

/// Parse [`Cidr`] with custom address and network (when no '/' separator was found) parser
///
/// Similar to [`parse_cidr_full`] but ignores host bits in addresses.
pub fn parse_cidr_full_ignore_hostbits<C, AP, NP>(
	s: &str,
	address_parser: AP,
	host_parser: NP,
) -> Result<C, NetworkParseError>
where
	C: Cidr,
	AP: FnOnce(&str) -> Result<C::Address, AddrParseError>,
	NP: FnOnce(&str) -> Result<C, NetworkParseError>,
{
	match s.rfind('/') {
		None => host_parser(s),
		Some(pos) => {
			let inet = <C::Address as Address>::Inet::new(
				address_parser(&s[0..pos])?,
				s[pos + 1..].parse()?,
			)?;
			Ok(inet.network())
		},
	}
}

/// Parse [`Cidr`] with custom address parser
///
/// Similar to [`parse_cidr`] but ignores host bits in addresses.
pub fn parse_cidr_ignore_hostbits<C, AP>(
	s: &str,
	address_parser: AP,
) -> Result<C, NetworkParseError>
where
	C: Cidr,
	AP: Fn(&str) -> Result<C::Address, AddrParseError>,
{
	parse_cidr_full_ignore_hostbits(s, &address_parser, |s| Ok(C::new_host(address_parser(s)?)))
}

/// Parse [`AnyIpCidr`] with custom address and network (when no '/' separator was found) parser
///
/// Similar to [`parse_any_cidr_full`] but ignores host bits in addresses.
pub fn parse_any_cidr_full_ignore_hostbits<AP, NP>(
	s: &str,
	address_parser: AP,
	host_parser: NP,
) -> Result<AnyIpCidr, NetworkParseError>
where
	AP: FnOnce(&str) -> Result<IpAddr, AddrParseError>,
	NP: FnOnce(&str) -> Result<AnyIpCidr, NetworkParseError>,
{
	match s.rfind('/') {
		None => host_parser(s),
		Some(pos) => Ok(
			IpInet::new(address_parser(&s[0..pos])?, s[pos + 1..].parse()?)?
				.network()
				.into(),
		),
	}
}

/// Parse [`AnyIpCidr`] with custom address parser
///
/// Similar to [`parse_any_cidr`] but ignores host bits in addresses.
pub fn parse_any_cidr_ignore_hostbits<AP>(
	s: &str,
	address_parser: AP,
) -> Result<AnyIpCidr, NetworkParseError>
where
	AP: Fn(&str) -> Result<IpAddr, AddrParseError>,
{
	parse_any_cidr_full(s, &address_parser, |s| {
		if s == "any" {
			Ok(AnyIpCidr::Any)
		} else {
			Ok(AnyIpCidr::new_host(address_parser(s)?))
		}
	})
}

/// Parse [`AnyIpCidr`] with custom address and network (when no '/' separator was found) parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse with `host_parser`.
pub fn parse_any_cidr_full<AP, NP>(
	s: &str,
	address_parser: AP,
	host_parser: NP,
) -> Result<AnyIpCidr, NetworkParseError>
where
	AP: FnOnce(&str) -> Result<IpAddr, AddrParseError>,
	NP: FnOnce(&str) -> Result<AnyIpCidr, NetworkParseError>,
{
	match s.rfind('/') {
		None => host_parser(s),
		Some(pos) => AnyIpCidr::new(address_parser(&s[0..pos])?, s[pos + 1..].parse()?),
	}
}

/// Parse [`AnyIpCidr`] with custom address parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// If input is just `"any"` returns [`AnyIpCidr::Any`].
/// Otherwise parse `address_parser` and treat as host (maximum prefix length).
pub fn parse_any_cidr<AP>(s: &str, address_parser: AP) -> Result<AnyIpCidr, NetworkParseError>
where
	AP: Fn(&str) -> Result<IpAddr, AddrParseError>,
{
	parse_any_cidr_full(s, &address_parser, |s| {
		if s == "any" {
			Ok(AnyIpCidr::Any)
		} else {
			Ok(AnyIpCidr::new_host(address_parser(s)?))
		}
	})
}

/// Parse [`Inet`] with custom address and network (when no '/' separator was found) parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse with `host_parser`.
pub fn parse_inet_full<I, AP, NP>(
	s: &str,
	address_parser: AP,
	host_parser: NP,
) -> Result<I, NetworkParseError>
where
	I: Inet,
	AP: FnOnce(&str) -> Result<I::Address, AddrParseError>,
	NP: FnOnce(&str) -> Result<I, NetworkParseError>,
{
	match s.rfind('/') {
		None => host_parser(s),
		Some(pos) => Ok(I::new(address_parser(&s[0..pos])?, s[pos + 1..].parse()?)?),
	}
}

/// Parse [`Inet`] with custom address parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse `address_parser` and treat as host (maximum prefix length).
pub fn parse_inet<I, AP>(s: &str, address_parser: AP) -> Result<I, NetworkParseError>
where
	I: Inet,
	AP: Fn(&str) -> Result<I::Address, AddrParseError>,
{
	parse_inet_full(s, &address_parser, |s| Ok(I::new_host(address_parser(s)?)))
}
