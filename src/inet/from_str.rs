use std::net::AddrParseError;
use std::str::FromStr;

use super::super::errors::*;
use super::super::traits::*;

pub fn inet_from_str<I>(s: &str) -> Result<I, NetworkParseError>
where
	I: Inet,
	I::Address: FromStr<Err=AddrParseError>
{
	Ok(match s.rfind('/') {
		None => I::new_host(I::Address::from_str(s)?),
		Some(pos) => {
			I::new(
				I::Address::from_str(&s[0..pos])?,
				u8::from_str(&s[pos+1..])?,
			)?
		}
	})
}
