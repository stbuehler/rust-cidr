use crate::{
	errors::NetworkParseError,
	local_addr_parser::ParseableAddress,
	Inet,
};

pub fn inet_from_str<I>(s: &str) -> Result<I, NetworkParseError>
where
	I: Inet,
	I::Address: ParseableAddress,
{
	// TODO: use strict FromStr::from_str address parsing with version bump
	crate::parsers::parse_inet(s, I::Address::address_from_str)
}
