use core::str::FromStr;

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
	Ok(match s.rfind('/') {
		None => I::new_host(I::Address::address_from_str(s)?),
		Some(pos) => I::new(
			I::Address::address_from_str(&s[0..pos])?,
			u8::from_str(&s[pos + 1..])?,
		)?,
	})
}
