//! Extra parsers
//!
//! The `FromStr` implementations in this crate are rather strict
//! about the allowed input.  Depending on the data format that needs
//! to be handled the functions here might help implementing custom
//! parsers.
//!
//! The parser combinators that take an `address_parser` can either
//! take `FromStr::from_str` or a non-default parser like [`parse_loose_ip`].
//! They are used to parse addresses (either as part of a `"/"` separated
//! notation or as single host).
//!
//! Parser combinators that take an additional `host_parser` use that
//! to parse strings that don't have an `"/"` separator - usually these
//! should return Cidr/Inet "host" values, but they can allow special
//! syntax like [`parse_short_ip_cidr`] to represent non-host networks too.

mod combinators;
mod inetaddr;
mod ipv4_short;

pub use self::{
	combinators::{
		parse_any_cidr,
		parse_any_cidr_full,
		parse_any_cidr_full_ignore_hostbits,
		parse_any_cidr_ignore_hostbits,
		parse_cidr,
		parse_cidr_full,
		parse_cidr_full_ignore_hostbits,
		parse_cidr_ignore_hostbits,
		parse_inet,
		parse_inet_full,
	},
	inetaddr::{
		inet_addr,
		parse_loose_ip,
		parse_loose_ipv4,
	},
	ipv4_short::{
		parse_short_any_ip_cidr,
		parse_short_ip_address_as_cidr,
		parse_short_ip_cidr,
		parse_short_ipv4_address_as_cidr,
		parse_short_ipv4_cidr,
	},
};
