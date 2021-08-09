use core::str::FromStr;

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
	match s.rfind('/') {
		None => Ok(C::new_host(C::Address::address_from_str(s)?)),
		Some(pos) => C::new(
			C::Address::address_from_str(&s[0..pos])?,
			u8::from_str(&s[pos + 1..])?,
		),
	}
}
