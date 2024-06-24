//! Extra parsers
//!
//! The `FromStr` implementations in this crate are rather strict
//! about the allowed input.  Depending on the data format that needs
//! to be handled the functions here might help implementing custom
//! parsers.

mod combinators;
mod inetaddr;

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
};
