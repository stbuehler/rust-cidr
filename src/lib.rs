#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/cidr/0.0.4")]

//! This library provides types to represent an IP network (`Cidr`) or
//! an IP host withing a network (`Inet`)
//!
//! The naming follows the names of the [PostgreSQL data types](https://www.postgresql.org/docs/current/static/datatype-net-types.html)
//!
//! Address parsing also accepts IPv4 address with less than four octets
//! (but always parses those as decimal).
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

pub use self::cidr::*;
pub use self::errors::*;
pub use self::family::*;
pub use self::inet::*;
pub use self::inet_iterator::*;
pub use self::traits::*;

extern crate bitstring;
#[cfg(feature = "serde")]
extern crate serde;

#[cfg(all(test, feature = "serde"))]
extern crate bincode;
#[cfg(all(test, feature = "serde"))]
extern crate serde_test;

mod serde_common;

mod cidr;
mod errors;
mod family;
mod inet;
mod inet_iterator;
mod local_addr_parser;
mod traits;
