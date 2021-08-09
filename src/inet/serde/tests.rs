use crate::{
	IpInet,
	Ipv4Inet,
	Ipv6Inet,
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
	let c: Ipv4Inet = "192.0.2.1/24".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.1/24")]);

	assert_bincode(&c.compact(), &[24, 192, 0, 2, 1]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Inet" },
			Token::Tuple { len: 2 },
			Token::U8(24),
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
			Token::NewtypeStruct { name: "Ipv4Inet" },
			Token::Seq { len: None },
			Token::U8(24),
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
fn test_ipv4_host() {
	let c: Ipv4Inet = "192.0.2.1/32".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.1")]);

	assert_bincode(&c.compact(), &[32, 192, 0, 2, 1]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Inet" },
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
			Token::NewtypeStruct { name: "Ipv4Inet" },
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
	let c: Ipv6Inet = "2001:DB8:1234:5678:1:2:3:4/64".parse().unwrap();

	assert_tokens(
		&c.readable(),
		&[Token::Str("2001:db8:1234:5678:1:2:3:4/64")],
	);

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
			Token::NewtypeStruct { name: "Ipv6Inet" },
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
			Token::NewtypeStruct { name: "Ipv6Inet" },
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
fn test_ipv6_host() {
	let c: Ipv6Inet = "2001:DB8:1234:5678:1:2:3:4/128".parse().unwrap();

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
			Token::NewtypeStruct { name: "Ipv6Inet" },
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
			Token::NewtypeStruct { name: "Ipv6Inet" },
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
fn test_inet_v4() {
	let c: IpInet = "192.0.2.1/24".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.1/24")]);

	assert_bincode(&c.compact(), &[24, 192, 0, 2, 1]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpInet" },
			Token::Tuple { len: 2 },
			Token::U8(24),
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
			Token::NewtypeStruct { name: "IpInet" },
			Token::Seq { len: None },
			Token::U8(24),
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
fn test_inet_v4_host() {
	let c: IpInet = "192.0.2.1/32".parse().unwrap();

	assert_tokens(&c.readable(), &[Token::Str("192.0.2.1")]);

	assert_bincode(&c.compact(), &[32, 192, 0, 2, 1]);

	assert_tokens(
		&c.compact(),
		&[
			Token::NewtypeStruct { name: "IpInet" },
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
			Token::NewtypeStruct { name: "IpInet" },
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
fn test_inet_v6() {
	let c: IpInet = "2001:DB8:1234:5678:1:2:3:4/64".parse().unwrap();

	assert_tokens(
		&c.readable(),
		&[Token::Str("2001:db8:1234:5678:1:2:3:4/64")],
	);

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
			Token::NewtypeStruct { name: "IpInet" },
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
			Token::NewtypeStruct { name: "IpInet" },
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
fn test_inet_v6_host() {
	let c: IpInet = "2001:DB8:1234:5678:1:2:3:4/128".parse().unwrap();

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
			Token::NewtypeStruct { name: "IpInet" },
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
			Token::NewtypeStruct { name: "IpInet" },
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
