//! Extra parsers
//!
//! The `FromStr` implementations in this crate are rather strict
//! about the allowed input.  Depending on the data format that needs
//! to be handled the functions here might help implementing custom
//! parsers.

mod inetaddr;

pub use self::inetaddr::{inet_addr, parse_loose_ip, parse_loose_ipv4};
