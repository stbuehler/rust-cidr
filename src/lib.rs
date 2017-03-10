#![warn(missing_docs)]

//! This library provides types to represent an IP network (`Cidr`) or
//! an IP host withing a network (`Inet`)
//!
//! The naming follows the names of the [PostgreSQL data types](https://www.postgresql.org/docs/current/static/datatype-net-types.html)
//!
//! Address parsing also accepts IPv4 address with less than four octets (but always parses those as decimal).

pub use self::cidr::*;
pub use self::errors::*;
pub use self::family::*;
pub use self::inet::*;
pub use self::inet_iterator::*;
pub use self::traits::*;

/// should perhaps be a separate crate
mod bitstring;

/// implementation helper for bitstrings
mod bigendian;

mod cidr;
mod errors;
mod family;
mod inet;
mod inet_iterator;
mod local_addr_parser;
mod traits;
