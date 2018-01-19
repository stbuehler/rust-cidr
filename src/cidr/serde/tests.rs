use {AnyIpCidr, IpCidr, Ipv4Cidr, Ipv6Cidr};
use serde_test::{assert_de_tokens, assert_tokens, Configure, Token};

#[test]
fn test_ipv4() {
	let c: Ipv4Cidr = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.clone().readable(), &[Token::Str("192.0.2.0/24")]);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
fn test_ipv6() {
	let c: Ipv6Cidr = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(
		&c.clone().readable(),
		&[Token::Str("2001:db8:1234:5678::/64")],
	);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
fn test_cidr_v4() {
	let c: IpCidr = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.clone().readable(), &[Token::Str("192.0.2.0/24")]);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
fn test_cidr_v6() {
	let c: IpCidr = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(
		&c.clone().readable(),
		&[Token::Str("2001:db8:1234:5678::/64")],
	);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
fn test_any_cidr_any() {
	let c: AnyIpCidr = "any".parse().unwrap();

	assert_tokens(&c.clone().readable(), &[Token::Str("any")]);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::NewtypeStruct { name: "AnyIpCidr" },
			Token::Tuple { len: 2 },
			Token::U8(0xff),
			Token::Unit,
			Token::TupleEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
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

	assert_tokens(&c.clone().readable(), &[Token::Str("192.0.2.0/24")]);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
fn test_any_cidr_v6() {
	let c: AnyIpCidr = "2001:DB8:1234:5678::/64".parse().unwrap();

	assert_tokens(
		&c.clone().readable(),
		&[Token::Str("2001:db8:1234:5678::/64")],
	);

	assert_tokens(
		&c.clone().compact(),
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
		&c.clone().compact(),
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
