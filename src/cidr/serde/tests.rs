use crate::{
	AnyIpCidr,
	IpCidr,
	Ipv4Cidr,
	Ipv6Cidr,
};
use serde_test::{
	assert_de_tokens,
	assert_tokens,
	Configure,
	Token,
};

pub fn assert_bincode<'de, T>(value: &T, raw: &'de [u8])
where
	T: serde::Serialize + serde::Deserialize<'de> + PartialEq + ::core::fmt::Debug,
{
	use bincode::Options;
	let config = bincode::options().with_limit(256.max(raw.len() as u64));

	let s = config.serialize(value).unwrap_or_else(|e| {
		panic!("failed serializing {:?}: {}", value, e);
	});

	assert_eq!(
		&s as &[u8], raw,
		"unexpected result serializing {:?}",
		value
	);

	assert_eq!(
		&bincode::deserialize::<T>(raw).unwrap(),
		value,
		"unexpected result deserializing {:?}",
		raw
	);
}

#[test]
fn test_ipv4() {
	let c: Ipv4Cidr = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.0/24")]);

	assert_bincode(&c.compact(), &[24, 192, 0, 2, 0]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Cidr" },
			Token::Tuple { len: 2 },
			Token::U8(24),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Cidr" },
			Token::Seq { len: None },
			Token::U8(24),
			Token::Seq { len: None },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_ipv4_host() {
	let c: Ipv4Cidr = "192.0.2.1/32".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.1")]);

	assert_bincode(&c.compact(), &[32, 192, 0, 2, 1]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Cidr" },
			Token::Tuple { len: 2 },
			Token::U8(32),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(1),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Cidr" },
			Token::Seq { len: None },
			Token::U8(32),
			Token::Seq { len: None },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(1),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_ipv6() {
	let c: Ipv6Cidr = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("2001:db8:1234:5678::/64")]);

	assert_bincode(
		&c.compact(),
		&[
			64 + 0x40,
			0x20,
			0x01,
			0x0d,
			0xb8,
			0x12,
			0x34,
			0x56,
			0x78,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
		],
	);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv6Cidr" },
			Token::Tuple { len: 2 },
			Token::U8(64 + 0x40),
			Token::Tuple { len: 16 },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv6Cidr" },
			Token::Seq { len: None },
			Token::U8(64 + 0x40),
			Token::Seq { len: None },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_ipv6_host() {
	let c: Ipv6Cidr = "2001:DB8:1234:5678:1:2:3:4/128".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("2001:db8:1234:5678:1:2:3:4")]);

	assert_bincode(
		&c.compact(),
		&[
			128 + 0x40,
			0x20,
			0x01,
			0x0d,
			0xb8,
			0x12,
			0x34,
			0x56,
			0x78,
			0x00,
			0x01,
			0x00,
			0x02,
			0x00,
			0x03,
			0x00,
			0x04,
		],
	);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv6Cidr" },
			Token::Tuple { len: 2 },
			Token::U8(128 + 0x40),
			Token::Tuple { len: 16 },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x01),
			Token::U8(0x00),
			Token::U8(0x02),
			Token::U8(0x00),
			Token::U8(0x03),
			Token::U8(0x00),
			Token::U8(0x04),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv6Cidr" },
			Token::Seq { len: None },
			Token::U8(128 + 0x40),
			Token::Seq { len: None },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x01),
			Token::U8(0x00),
			Token::U8(0x02),
			Token::U8(0x00),
			Token::U8(0x03),
			Token::U8(0x00),
			Token::U8(0x04),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_cidr_v4() {
	let c: IpCidr = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.0/24")]);

	assert_bincode(&c.compact(), &[24, 192, 0, 2, 0]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(24),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Seq { len: None },
			Token::U8(24),
			Token::Seq { len: None },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_cidr_v4_host() {
	let c: IpCidr = "192.0.2.1/32".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.1")]);

	assert_bincode(&c.compact(), &[32, 192, 0, 2, 1]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(32),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(1),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Seq { len: None },
			Token::U8(32),
			Token::Seq { len: None },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(1),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_cidr_v6() {
	let c: IpCidr = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("2001:db8:1234:5678::/64")]);

	assert_bincode(
		&c.compact(),
		&[
			64 + 0x40,
			0x20,
			0x01,
			0x0d,
			0xb8,
			0x12,
			0x34,
			0x56,
			0x78,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
		],
	);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(64 + 0x40),
			Token::Tuple { len: 16 },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Seq { len: None },
			Token::U8(64 + 0x40),
			Token::Seq { len: None },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_cidr_v6_host() {
	let c: IpCidr = "2001:DB8:1234:5678:1:2:3:4/128".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("2001:db8:1234:5678:1:2:3:4")]);

	assert_bincode(
		&c.compact(),
		&[
			128 + 0x40,
			0x20,
			0x01,
			0x0d,
			0xb8,
			0x12,
			0x34,
			0x56,
			0x78,
			0x00,
			0x01,
			0x00,
			0x02,
			0x00,
			0x03,
			0x00,
			0x04,
		],
	);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(128 + 0x40),
			Token::Tuple { len: 16 },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x01),
			Token::U8(0x00),
			Token::U8(0x02),
			Token::U8(0x00),
			Token::U8(0x03),
			Token::U8(0x00),
			Token::U8(0x04),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpCidr" },
			Token::Seq { len: None },
			Token::U8(128 + 0x40),
			Token::Seq { len: None },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x01),
			Token::U8(0x00),
			Token::U8(0x02),
			Token::U8(0x00),
			Token::U8(0x03),
			Token::U8(0x00),
			Token::U8(0x04),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_any_cidr_any() {
	let c: AnyIpCidr = "any".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("any")]);

	assert_bincode(&c.compact(), &[0xff]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(0xff),
			Token::Unit,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Seq { len: None },
			Token::U8(0xff),
			Token::Unit,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_any_cidr_v4() {
	let c: AnyIpCidr = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.0/24")]);

	assert_bincode(&c.compact(), &[24, 192, 0, 2, 0]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(24),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Seq { len: None },
			Token::U8(24),
			Token::Seq { len: None },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_any_cidr_v4_host() {
	let c: AnyIpCidr = "192.0.2.1/32".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.1")]);

	assert_bincode(&c.compact(), &[32, 192, 0, 2, 1]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(32),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(1),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Seq { len: None },
			Token::U8(32),
			Token::Seq { len: None },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(1),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_any_cidr_v6() {
	let c: AnyIpCidr = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("2001:db8:1234:5678::/64")]);

	assert_bincode(
		&c.compact(),
		&[
			64 + 0x40,
			0x20,
			0x01,
			0x0d,
			0xb8,
			0x12,
			0x34,
			0x56,
			0x78,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
			0x00,
		],
	);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(64 + 0x40),
			Token::Tuple { len: 16 },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Seq { len: None },
			Token::U8(64 + 0x40),
			Token::Seq { len: None },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::U8(0x00),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}

#[test]
fn test_any_cidr_v6_host() {
	let c: AnyIpCidr = "2001:DB8:1234:5678:1:2:3:4/128".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("2001:db8:1234:5678:1:2:3:4")]);

	assert_bincode(
		&c.compact(),
		&[
			128 + 0x40,
			0x20,
			0x01,
			0x0d,
			0xb8,
			0x12,
			0x34,
			0x56,
			0x78,
			0x00,
			0x01,
			0x00,
			0x02,
			0x00,
			0x03,
			0x00,
			0x04,
		],
	);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(128 + 0x40),
			Token::Tuple { len: 16 },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x01),
			Token::U8(0x00),
			Token::U8(0x02),
			Token::U8(0x00),
			Token::U8(0x03),
			Token::U8(0x00),
			Token::U8(0x04),
			Token::TupleEnd,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Seq { len: None },
			Token::U8(128 + 0x40),
			Token::Seq { len: None },
			Token::U8(0x20),
			Token::U8(0x01),
			Token::U8(0x0d),
			Token::U8(0xb8),
			Token::U8(0x12),
			Token::U8(0x34),
			Token::U8(0x56),
			Token::U8(0x78),
			Token::U8(0x00),
			Token::U8(0x01),
			Token::U8(0x00),
			Token::U8(0x02),
			Token::U8(0x00),
			Token::U8(0x03),
			Token::U8(0x00),
			Token::U8(0x04),
			Token::SeqEnd,
			Token::SeqEnd,
		],
	);
}
