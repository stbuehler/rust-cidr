#![cfg(feature = "serde")]
#![cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]

use crate::{
	inet::{
		IpInet,
		Ipv4Inet,
		Ipv6Inet,
	},
	serde_common,
};
use std::net::IpAddr;

static NAME_IPV4_CIDR: &str = "Ipv4Inet";

impl serde::Serialize for Ipv4Inet {
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
				serde_common::deserialize_v4(deserializer, NAME_IPV4_CIDR)?;
			Ipv4Inet::new(addr, network_length).map_err(serde::de::Error::custom)
		}
	}
}

static NAME_IPV6_CIDR: &str = "Ipv6Inet";

impl serde::Serialize for Ipv6Inet {
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
				serde_common::deserialize_v6(deserializer, NAME_IPV6_CIDR)?;
			Ipv6Inet::new(addr, network_length).map_err(serde::de::Error::custom)
		}
	}
}

static NAME_IP_CIDR: &str = "IpInet";

impl serde::Serialize for IpInet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&format!("{}", self))
		} else {
			let data = match self {
				Self::V4(i) => (IpAddr::V4(i.address), i.network_length),
				Self::V6(i) => (IpAddr::V6(i.address), i.network_length),
			};
			serde_common::serialize(serializer, NAME_IP_CIDR, data)
		}
	}
}

impl<'de> serde::Deserialize<'de> for IpInet {
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

#[cfg(test)]
mod tests;
