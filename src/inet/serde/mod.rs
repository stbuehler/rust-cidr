#![cfg(feature = "serde")]

use Inet;
use inet::{IpInet, Ipv4Inet, Ipv6Inet};
use serde;
use serde_common::addr_len_map;

static NAME_IPV4_INET: &str = "Ipv4Inet";

impl serde::Serialize for Ipv4Inet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			addr_len_map::serialize(
				serializer,
				NAME_IPV4_INET,
				&self.address,
				self.network_length,
			)
		}
	}
}

impl<'de> serde::Deserialize<'de> for Ipv4Inet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		if deserializer.is_human_readable() {
			let s = String::deserialize(deserializer)?;
			s.parse().map_err(serde::de::Error::custom)
		} else {
			let (addr, network_length) =
				addr_len_map::deserialize(deserializer, NAME_IPV4_INET)?;
			Ipv4Inet::new(addr, network_length)
				.map_err(serde::de::Error::custom)
		}
	}
}

static NAME_IPV6_INET: &str = "Ipv6Inet";

impl serde::Serialize for Ipv6Inet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			addr_len_map::serialize(
				serializer,
				NAME_IPV6_INET,
				&self.address,
				self.network_length,
			)
		}
	}
}

impl<'de> serde::Deserialize<'de> for Ipv6Inet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		if deserializer.is_human_readable() {
			let s = String::deserialize(deserializer)?;
			s.parse().map_err(serde::de::Error::custom)
		} else {
			let (addr, network_length) =
				addr_len_map::deserialize(deserializer, NAME_IPV6_INET)?;
			Ipv6Inet::new(addr, network_length)
				.map_err(serde::de::Error::custom)
		}
	}
}

serde_nf_enum!{IpInet}

#[cfg(test)]
mod tests;
