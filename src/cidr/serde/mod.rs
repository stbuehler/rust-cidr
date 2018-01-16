#![cfg(feature = "serde")]

use Cidr;
use cidr::{AnyIpCidr, IpCidr, Ipv4Cidr, Ipv6Cidr};
use serde;
use serde_common::addr_len_map;

static NAME_IPV4_CIDR: &str = "Ipv4Cidr";

impl serde::Serialize for Ipv4Cidr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			addr_len_map::serialize(
				serializer,
				NAME_IPV4_CIDR,
				&self.address,
				self.network_length,
			)
		}
	}
}

impl<'de> serde::Deserialize<'de> for Ipv4Cidr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		if deserializer.is_human_readable() {
			let s = String::deserialize(deserializer)?;
			s.parse().map_err(serde::de::Error::custom)
		} else {
			let (addr, network_length) =
				addr_len_map::deserialize(deserializer, NAME_IPV4_CIDR)?;
			Ipv4Cidr::new(addr, network_length)
				.map_err(serde::de::Error::custom)
		}
	}
}

static NAME_IPV6_CIDR: &str = "Ipv6Cidr";

impl serde::Serialize for Ipv6Cidr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			addr_len_map::serialize(
				serializer,
				NAME_IPV6_CIDR,
				&self.address,
				self.network_length,
			)
		}
	}
}

impl<'de> serde::Deserialize<'de> for Ipv6Cidr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		if deserializer.is_human_readable() {
			let s = String::deserialize(deserializer)?;
			s.parse().map_err(serde::de::Error::custom)
		} else {
			let (addr, network_length) =
				addr_len_map::deserialize(deserializer, NAME_IPV6_CIDR)?;
			Ipv6Cidr::new(addr, network_length)
				.map_err(serde::de::Error::custom)
		}
	}
}

serde_nf_enum!{IpCidr}

serde_nf_any_enum!{AnyIpCidr}

#[cfg(test)]
mod tests;
