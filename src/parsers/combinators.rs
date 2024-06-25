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
	IpCidr,
	IpInet,
};

/// Parse [`Cidr`] with custom address and network (when no '/' separator was found) parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse with `host_parser`.
pub fn parse_cidr_full<C, AP, HP>(
	s: &str,
	address_parser: AP,
	host_parser: HP,
) -> Result<C, NetworkParseError>
where
	C: Cidr,
	AP: FnOnce(&str) -> Result<C::Address, AddrParseError>,
	HP: FnOnce(&str) -> Result<C, NetworkParseError>,
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
pub fn parse_cidr_full_ignore_hostbits<C, AP, HP>(
	s: &str,
	address_parser: AP,
	host_parser: HP,
) -> Result<C, NetworkParseError>
where
	C: Cidr,
	AP: FnOnce(&str) -> Result<C::Address, AddrParseError>,
	HP: FnOnce(&str) -> Result<C, NetworkParseError>,
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
/// Return [`AnyIpCidr::Any`] for `"any"`.
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse with `host_parser`.
pub fn parse_any_cidr_full<AP, HP>(
	s: &str,
	address_parser: AP,
	host_parser: HP,
) -> Result<AnyIpCidr, NetworkParseError>
where
	AP: FnOnce(&str) -> Result<IpAddr, AddrParseError>,
	HP: FnOnce(&str) -> Result<IpCidr, NetworkParseError>,
{
	if s == "any" {
		return Ok(AnyIpCidr::Any);
	}
	match s.rfind('/') {
		None => Ok(host_parser(s)?.into()),
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
		Ok(IpCidr::new_host(address_parser(s)?))
	})
}

/// Parse [`AnyIpCidr`] with custom address and network (when no '/' separator was found) parser
///
/// Similar to [`parse_any_cidr_full`] but ignores host bits in addresses.
pub fn parse_any_cidr_full_ignore_hostbits<AP, HP>(
	s: &str,
	address_parser: AP,
	host_parser: HP,
) -> Result<AnyIpCidr, NetworkParseError>
where
	AP: FnOnce(&str) -> Result<IpAddr, AddrParseError>,
	HP: FnOnce(&str) -> Result<IpCidr, NetworkParseError>,
{
	if s == "any" {
		return Ok(AnyIpCidr::Any);
	}
	match s.rfind('/') {
		None => Ok(host_parser(s)?.into()),
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
	parse_any_cidr_full_ignore_hostbits(s, &address_parser, |s| {
		Ok(IpCidr::new_host(address_parser(s)?))
	})
}

/// Parse [`Inet`] with custom address and network (when no '/' separator was found) parser
///
/// If a '/' is found, parse trailing number as prefix length and leading address with `address_parser`.
/// Otherwise parse with `host_parser`.
pub fn parse_inet_full<I, AP, HP>(
	s: &str,
	address_parser: AP,
	host_parser: HP,
) -> Result<I, NetworkParseError>
where
	I: Inet,
	AP: FnOnce(&str) -> Result<I::Address, AddrParseError>,
	HP: FnOnce(&str) -> Result<I, NetworkParseError>,
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
