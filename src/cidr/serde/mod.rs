#![cfg(feature = "serde")]
#![cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]

use crate::{
	cidr::{
		AnyIpCidr,
		IpCidr,
		Ipv4Cidr,
		Ipv6Cidr,
	},
	serde_common,
};
use std::net::IpAddr;

static NAME_IPV4_CIDR: &str = "Ipv4Cidr";

impl serde::Serialize for Ipv4Cidr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			serde_common::serialize_v4(
				serializer,
				NAME_IPV4_CIDR,
				(self.address, self.network_length),
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
				serde_common::deserialize_v4(deserializer, NAME_IPV4_CIDR)?;
			Self::new(addr, network_length).map_err(serde::de::Error::custom)
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
			serde_common::serialize_v6(
				serializer,
				NAME_IPV6_CIDR,
				(self.address, self.network_length),
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
				serde_common::deserialize_v6(deserializer, NAME_IPV6_CIDR)?;
			Ipv6Cidr::new(addr, network_length).map_err(serde::de::Error::custom)
		}
	}
}

static NAME_IP_CIDR: &str = "IpCidr";

impl serde::Serialize for IpCidr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			let data = match self {
				Self::V4(c) => (IpAddr::V4(c.address), c.network_length),
				Self::V6(c) => (IpAddr::V6(c.address), c.network_length),
			};
			serde_common::serialize(serializer, NAME_IP_CIDR, data)
		}
	}
}

impl<'de> serde::Deserialize<'de> for IpCidr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		if deserializer.is_human_readable() {
			let s = String::deserialize(deserializer)?;
			s.parse().map_err(serde::de::Error::custom)
		} else {
			let (addr, network_length) = serde_common::deserialize(deserializer, NAME_IP_CIDR)?;
			Self::new(addr, network_length).map_err(serde::de::Error::custom)
		}
	}
}

static NAME_ANY_IP_CIDR: &str = "AnyIpCidr";

impl serde::Serialize for AnyIpCidr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			let data = match self {
				Self::Any => None,
				Self::V4(c) => Some((IpAddr::V4(c.address), c.network_length)),
				Self::V6(c) => Some((IpAddr::V6(c.address), c.network_length)),
			};
			serde_common::serialize_any(serializer, NAME_ANY_IP_CIDR, data)
		}
	}
}

impl<'de> serde::Deserialize<'de> for AnyIpCidr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		if deserializer.is_human_readable() {
			let s = String::deserialize(deserializer)?;
			s.parse().map_err(serde::de::Error::custom)
		} else {
			match serde_common::deserialize_any(deserializer, NAME_ANY_IP_CIDR)? {
				None => Ok(Self::Any),
				Some((addr, network_length)) => {
					Self::new(addr, network_length).map_err(serde::de::Error::custom)
				},
			}
		}
	}
}

#[cfg(test)]
mod tests;
