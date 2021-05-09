#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/cidr/0.1.1")]
#![allow(clippy::match_like_matches_macro)]

//! This library provides types to represent an IP network (`Cidr`) or
//! an IP host withing a network (`Inet`)
//!
//! The naming follows the names of the [PostgreSQL data types](https://www.postgresql.org/docs/current/static/datatype-net-types.html)
//!
//! Address parsing also accepts IPv4 address with less than four octets
//! (but always parses those as decimal).
//!
//! If the `#` flag is used with the `Display` formatting (i.e. `{:#}`) the
//! prefix will be shown even for host addresses (added in `0.1.1`).
//!
//! # Feature `serde`
//!
//! This feature is enabled by default (not using `serde-derive`, only
//! `serde`).
//!
//! In human readable formats the `Display` and `FromStr` interfaces are
//! used.  Otherwise all values are serialized in the same format (apart
//! from the newtype wrapping) as a tuple of two values:
//!
//! - `tag: u8`:
//!   - `0x00...0x20`: IPv4 with network length `tag`
//!   - `0x40...0xc0`: IPv6 with network length `tag - 0x40`
//!   - `0xff`: `any`
//! - address according to `tag`: `Ipv4Addr` (`[u8; 4]`), `Ipv6Addr`
//!   (`[u8; 16]`) or `()`

pub use self::{
	cidr::{AnyIpCidr, IpCidr, Ipv4Cidr, Ipv6Cidr},
	family::Family,
	inet::{IpInet, Ipv4Inet, Ipv6Inet},
	inet_iterator::InetIterator,
	inet_pair::{IpInetPair, Ipv4InetPair, Ipv6InetPair},
	traits::{Address, Cidr, HasAddressType, Inet, InetPair},
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
