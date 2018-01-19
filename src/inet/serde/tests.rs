use {IpInet, Ipv4Inet, Ipv6Inet};
use serde_test::{assert_de_tokens, assert_tokens, Configure, Token};

#[test]
fn test_ipv4() {
	let c: Ipv4Inet = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.clone().readable(), &[Token::Str("192.0.2.0/24")]);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Inet" },
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
		&c.clone().compact(),
		&[
			Token::NewtypeStruct { name: "Ipv4Inet" },
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
fn test_ipv6() {
	let c: Ipv6Inet = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(
		&c.clone().readable(),
		&[Token::Str("2001:db8:1234:5678::/64")],
	);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
fn test_inet_v4() {
	let c: IpInet = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.clone().readable(), &[Token::Str("192.0.2.0/24")]);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::NewtypeStruct { name: "IpInet" },
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
		&c.clone().compact(),
		&[
			Token::NewtypeStruct { name: "IpInet" },
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
fn test_inet_v6() {
	let c: IpInet = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(
		&c.clone().readable(),
		&[Token::Str("2001:db8:1234:5678::/64")],
	);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
