#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/cidr/0.2.1")]
#![allow(clippy::match_like_matches_macro)]

//! This library provides types to represent an IP network ([`Cidr`]) or
//! an IP host withing a network ([`Inet`])
//!
//! The naming follows the names of the [PostgreSQL data types](https://www.postgresql.org/docs/current/static/datatype-net-types.html)
//!
//! Address parsing also accepts IPv4 address with less than four octets
//! (but always parses those as decimal).
//!
//! If the `#` flag is used with the `Display` formatting (i.e. `{:#}`) the
//! prefix will be shown even for host addresses (added in `0.1.1`).
//!
//! # Feature `std`
//!
//! Enabled by default, also mandatory right now because [`std::net`] isn't
//! available in [`core`].
//!
//! # Feature `serde`
//!
//! This feature enables various types to be serialized using `serde`
//! (without `serde-derive`).
//!
//! In human readable formats the `Display` and `FromStr` interfaces are
//! used.  Otherwise all values are serialized in the same format (apart
//! from the newtype wrapping) as a tuple of two values:
//!
//! - `tag: u8`:
//!   - `0x00...0x20`: IPv4 with network length `tag`
//!   - `0x40...0xc0`: IPv6 with network length `tag - 0x40`
//!   - `0xff`: `any`
//! - address according to `tag`: [`Ipv4Addr`] (`[u8; 4]`), [`Ipv6Addr`]
//!   (`[u8; 16]`) or `()`
//!
//! The represenation hasn't been changed in 0.2; it is compatible with 0.1.
//!
//! # Feature `bitstring`
//!
//! This feature allows various types to be used as [`bitstring::BitString`],
//! which allows them being in used in containers like [bitstring-trees].
//!
//! [bitstring-trees]: https://crates.io/crates/bitstring-trees
//!
//! [`Ipv4Addr`]: std::net::Ipv4Addr
//! [`Ipv6Addr`]: std::net::Ipv6Addr

pub use self::{
	cidr::{
		AnyIpCidr,
		IpCidr,
		Ipv4Cidr,
		Ipv6Cidr,
	},
	family::Family,
	inet::{
		IpInet,
		Ipv4Inet,
		Ipv6Inet,
	},
	inet_iterator::{
		InetAddressIterator,
		InetIterator,
	},
	inet_pair::{
		IpInetPair,
		Ipv4InetPair,
		Ipv6InetPair,
	},
	traits::{
		Address,
		Cidr,
		Inet,
		InetPair,
	},
};

pub mod errors;

mod serde_common;

mod address;
mod cidr;
mod family;
mod inet;
mod inet_iterator;
mod inet_pair;
mod internal_traits;
mod local_addr_parser;
mod num;
mod traits;
