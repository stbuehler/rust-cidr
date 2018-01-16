#![macro_use]

use serde::de;
use serde::ser;
use std::fmt;
use std::str;

pub static NF_KIND_NAMES: &[&str] = &["V4", "V6"];

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum NFKind {
	V4 = 1,
	V6 = 2,
}

impl NFKind {
	pub fn name(self) -> &'static str {
		NF_KIND_NAMES[((self as u32) - 1) as usize]
	}

	pub fn serialize_newtype_variant<S, T>(
		self,
		serializer: S,
		name: &'static str,
		value: &T,
	) -> Result<S::Ok, S::Error>
	where
		S: ser::Serializer,
		T: ser::Serialize,
	{
		serializer.serialize_newtype_variant(
			name,
			self as u32,
			self.name(),
			value,
		)
	}
}

impl<'de> de::Deserialize<'de> for NFKind {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		struct KindVisitor;

		impl<'de> de::Visitor<'de> for KindVisitor {
			type Value = NFKind;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("`V4` or `V6`")
			}

			fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				match value {
					1 => Ok(NFKind::V4),
					2 => Ok(NFKind::V6),
					_ => Err(de::Error::invalid_value(
						de::Unexpected::Unsigned(value as u64),
						&self,
					)),
				}
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				match value {
					"V4" => Ok(NFKind::V4),
					"V6" => Ok(NFKind::V6),
					_ => Err(de::Error::unknown_variant(value, NF_KIND_NAMES)),
				}
			}

			fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				match value {
					b"V4" => Ok(NFKind::V4),
					b"V6" => Ok(NFKind::V6),
					_ => match str::from_utf8(value) {
						Ok(value) => Err(
							de::Error::unknown_variant(value, NF_KIND_NAMES),
						),
						Err(_) => Err(de::Error::invalid_value(
							de::Unexpected::Bytes(value),
							&self,
						)),
					},
				}
			}
		}

		deserializer.deserialize_identifier(KindVisitor)
	}
}

macro_rules! serde_nf_enum {
	($type:ident) => {
		impl ::serde::ser::Serialize for $type {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: ::serde::ser::Serializer,
			{
				use serde_common::variant::NFKind;

				if serializer.is_human_readable() {
					serializer.serialize_str(&format!("{}", self))
				} else {
					match *self {
						$type::V4(ref a) => {
							NFKind::V4.serialize_newtype_variant(serializer, stringify!($type), a)
						}
						$type::V6(ref a) => {
							NFKind::V6.serialize_newtype_variant(serializer, stringify!($type), a)
						}
					}
				}
			}
		}

		impl<'de> serde::Deserialize<'de> for $type {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				use serde_common::variant::{NFKind, NF_KIND_NAMES};

				if deserializer.is_human_readable() {
					let s = String::deserialize(deserializer)?;
					s.parse().map_err(serde::de::Error::custom)
				} else {
					struct EnumVisitor;
					impl<'de> serde::de::Visitor<'de> for EnumVisitor {
						type Value = $type;

						fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
							formatter.write_str(concat!("a `", stringify!($type), "`"))
						}

						fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
						where
							A: serde::de::EnumAccess<'de>,
						{
							use serde::de::VariantAccess;
							match data.variant()? {
								(NFKind::V4, v) => v.newtype_variant().map($type::V4),
								(NFKind::V6, v) => v.newtype_variant().map($type::V6),
							}
						}
					}

					deserializer.deserialize_enum(stringify!($type), NF_KIND_NAMES, EnumVisitor)
				}
			}
		}
	};
}
