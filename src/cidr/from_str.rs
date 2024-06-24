use crate::{
	errors::*,
	local_addr_parser::ParseableAddress,
	Cidr,
};

pub fn cidr_from_str<C>(s: &str) -> Result<C, NetworkParseError>
where
	C: Cidr,
	C::Address: ParseableAddress,
{
	// TODO: use strict FromStr::from_str address parsing with version bump
	crate::parsers::parse_cidr(s, C::Address::address_from_str)
}
