#![macro_use]

use serde::de;
use serde::ser;
use std::fmt;
use std::marker::PhantomData;
use std::str;

const FIELD_ADDRESS: &str = "address";
const FIELD_NETWORK_LENGTH: &str = "network_length";
static FIELD_NAMES: &[&str] = &[FIELD_ADDRESS, FIELD_NETWORK_LENGTH];

enum FieldIdentifier {
	Address,
	NetworkLength,
}

impl<'de> de::Deserialize<'de> for FieldIdentifier {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		struct KindVisitor;

		impl<'de> de::Visitor<'de> for KindVisitor {
			type Value = FieldIdentifier;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("`Any`, `V4` or `V6`")
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				match value {
					FIELD_ADDRESS => Ok(FieldIdentifier::Address),
					FIELD_NETWORK_LENGTH => Ok(FieldIdentifier::NetworkLength),
					_ => Err(de::Error::unknown_variant(value, FIELD_NAMES)),
				}
			}

			fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				match value {
					b"address" => Ok(FieldIdentifier::Address),
					b"network_length" => Ok(FieldIdentifier::NetworkLength),
					_ => match str::from_utf8(value) {
						Ok(value) => {
							Err(de::Error::unknown_variant(value, FIELD_NAMES))
						},
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

pub fn serialize<A, S>(
	serializer: S,
	name: &'static str,
	addr: &A,
	network_length: u8,
) -> Result<S::Ok, S::Error>
where
	S: ser::Serializer,
	A: ser::Serialize,
{
	use serde::ser::SerializeStruct;

	let mut s = serializer.serialize_struct(name, 2)?;
	s.serialize_field(FIELD_ADDRESS, &addr)?;
	s.serialize_field(FIELD_NETWORK_LENGTH, &network_length)?;
	s.end()
}

struct StructVisitor<'de, Addr>
where
	Addr: de::Deserialize<'de> + 'de,
{
	name: &'static str,
	_mark: PhantomData<&'de Addr>,
}

impl<'de, Addr> de::Visitor<'de> for StructVisitor<'de, Addr>
where
	Addr: de::Deserialize<'de> + 'de,
{
	type Value = (Addr, u8);

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a `")?;
		formatter.write_str(self.name)?;
		formatter.write_str("`")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: de::SeqAccess<'de>,
	{
		let addr: Addr = seq.next_element()?
			.ok_or_else(|| de::Error::invalid_length(0, &self))?;
		let network_length: u8 = seq.next_element()?
			.ok_or_else(|| de::Error::invalid_length(0, &self))?;
		Ok((addr, network_length))
	}

	fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
	where
		A: de::MapAccess<'de>,
	{
		let mut addr = None;
		let mut network_length = None;

		while let Some(key) = map.next_key()? {
			match key {
				FieldIdentifier::Address => {
					if addr.is_some() {
						return Err(<A::Error as de::Error>::duplicate_field(
							FIELD_ADDRESS,
						));
					}
					addr = Some(map.next_value()?);
				},
				FieldIdentifier::NetworkLength => {
					if network_length.is_some() {
						return Err(<A::Error as de::Error>::duplicate_field(
							FIELD_NETWORK_LENGTH,
						));
					}
					network_length = Some(map.next_value()?);
				},
			}
		}

		let addr = addr.ok_or_else(
			|| <A::Error as de::Error>::missing_field(FIELD_ADDRESS),
		)?;
		let network_length = network_length.ok_or_else(
			|| <A::Error as de::Error>::missing_field(FIELD_ADDRESS),
		)?;
		Ok((addr, network_length))
	}
}

pub fn deserialize<'de, A, D>(
	deserializer: D,
	name: &'static str,
) -> Result<(A, u8), D::Error>
where
	D: de::Deserializer<'de>,
	A: de::Deserialize<'de> + 'de,
{
	deserializer.deserialize_struct(
		name,
		FIELD_NAMES,
		StructVisitor::<'de, A> {
			name,
			_mark: PhantomData,
		},
	)
}
