#![cfg(feature = "serde")]
#![cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]

use core::fmt;
use serde::{
	de,
	ser,
};
use std::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
};

pub fn serialize_any<S>(
	serializer: S,
	name: &'static str,
	data: Option<(IpAddr, u8)>,
) -> Result<S::Ok, S::Error>
where
	S: ser::Serializer,
{
	// inner tuple always has length 2, and the first entry is always
	// u8.  the type of the second entry depends on the first ("tag").
	match data {
		// tag: 0xff
		None => {
			let tag: u8 = 0xff;
			let value = (tag, ());
			serializer.serialize_newtype_struct(name, &value)
		},
		// tag: 0x00...0x20
		Some((IpAddr::V4(addr), len)) => {
			assert!(len <= 32, "network length out of bounds for IPv4: {}", len);
			let tag: u8 = len;
			let value = (tag, addr);
			serializer.serialize_newtype_struct(name, &value)
		},
		// tag: 0x40...0xc0
		Some((IpAddr::V6(addr), len)) => {
			assert!(len <= 128, "network length out of bounds for IPv4: {}", len);
			let tag: u8 = len + 64;
			let value = (tag, addr);
			serializer.serialize_newtype_struct(name, &value)
		},
	}
}

pub fn serialize<S>(
	serializer: S,
	name: &'static str,
	data: (IpAddr, u8),
) -> Result<S::Ok, S::Error>
where
	S: ser::Serializer,
{
	serialize_any(serializer, name, Some(data))
}

pub fn serialize_v4<S>(
	serializer: S,
	name: &'static str,
	data: (Ipv4Addr, u8),
) -> Result<S::Ok, S::Error>
where
	S: ser::Serializer,
{
	serialize_any(serializer, name, Some((IpAddr::V4(data.0), data.1)))
}

pub fn serialize_v6<S>(
	serializer: S,
	name: &'static str,
	data: (Ipv6Addr, u8),
) -> Result<S::Ok, S::Error>
where
	S: ser::Serializer,
{
	serialize_any(serializer, name, Some((IpAddr::V6(data.0), data.1)))
}

pub fn deserialize_any<'de, D>(
	deserializer: D,
	name: &'static str,
) -> Result<Option<(IpAddr, u8)>, D::Error>
where
	D: de::Deserializer<'de>,
{
	struct Visitor;
	impl<'de> de::Visitor<'de> for Visitor {
		type Value = Option<(IpAddr, u8)>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("a tuple with two fields")
		}

		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: de::SeqAccess<'de>,
		{
			let tag: u8 = seq
				.next_element()?
				.ok_or_else(|| de::Error::invalid_length(0, &self))?;
			match tag {
				0xff => {
					let _data: () = seq
						.next_element()?
						.ok_or_else(|| de::Error::invalid_length(1, &self))?;
					Ok(None)
				},
				0x00..=0x20 => {
					let network_length = tag;
					let addr: Ipv4Addr = seq
						.next_element()?
						.ok_or_else(|| de::Error::invalid_length(1, &self))?;
					Ok(Some((IpAddr::V4(addr), network_length)))
				},
				0x40..=0xc0 => {
					let network_length = tag - 0x40;
					let addr: Ipv6Addr = seq
						.next_element()?
						.ok_or_else(|| de::Error::invalid_length(1, &self))?;
					Ok(Some((IpAddr::V6(addr), network_length)))
				},
				_ => Err(de::Error::custom("invalid tag")),
			}
		}
	}

	struct NewTypeVisitor(&'static str);
	impl<'de> de::Visitor<'de> for NewTypeVisitor {
		type Value = Option<(IpAddr, u8)>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("newtype `")?;
			formatter.write_str(self.0)?;
			formatter.write_str("`")
		}

		fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
		where
			D: de::Deserializer<'de>,
		{
			deserializer.deserialize_tuple(2, Visitor)
		}
	}

	deserializer.deserialize_newtype_struct(name, NewTypeVisitor(name))
}

pub fn deserialize<'de, D>(deserializer: D, name: &'static str) -> Result<(IpAddr, u8), D::Error>
where
	D: de::Deserializer<'de>,
{
	deserialize_any(deserializer, name)?.ok_or_else(|| de::Error::custom("invalid value: `any`"))
}

pub fn deserialize_v4<'de, D>(
	deserializer: D,
	name: &'static str,
) -> Result<(Ipv4Addr, u8), D::Error>
where
	D: de::Deserializer<'de>,
{
	match deserialize(deserializer, name)? {
		(IpAddr::V4(addr), len) => Ok((addr, len)),
		(IpAddr::V6(_), _) => Err(de::Error::custom("invalid type: `Ipv6Addr`")),
	}
}

pub fn deserialize_v6<'de, D>(
	deserializer: D,
	name: &'static str,
) -> Result<(Ipv6Addr, u8), D::Error>
where
	D: de::Deserializer<'de>,
{
	match deserialize(deserializer, name)? {
		(IpAddr::V4(_), _) => Err(de::Error::custom("invalid type: `Ipv4Addr`")),
		(IpAddr::V6(addr), len) => Ok((addr, len)),
	}
}
