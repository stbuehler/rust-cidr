use {AnyIpCidr, IpCidr, Ipv4Cidr, Ipv6Cidr};
use serde_test::{assert_de_tokens, assert_tokens, Configure, Token};

#[test]
fn test_ipv4() {
	let c: Ipv4Cidr = "192.0.2.0/24".parse().unwrap();

	assert_tokens(&c.clone().readable(), &[Token::Str("192.0.2.0/24")]);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Map { len: None },
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::MapEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Seq { len: None },
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::U8(24),
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
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Map { len: None },
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::MapEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Seq { len: None },
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
			Token::U8(64),
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
			Token::NewtypeVariant {
				name: "IpCidr",
				variant: "V4",
			},
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
		],
	);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "IpCidr" },
			Token::Str("V4"),
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "IpCidr" },
			Token::Bytes(b"V4"),
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "IpCidr" },
			Token::U32(1),
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
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
			Token::NewtypeVariant {
				name: "IpCidr",
				variant: "V6",
			},
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "IpCidr" },
			Token::Str("V6"),
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "IpCidr" },
			Token::Bytes(b"V6"),
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "IpCidr" },
			Token::U32(2),
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
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
			Token::UnitVariant {
				name: "AnyIpCidr",
				variant: "Any",
			},
		],
	);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::Str("Any"),
			Token::Unit,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::Bytes(b"Any"),
			Token::Unit,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::U32(0),
			Token::Unit,
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
			Token::NewtypeVariant {
				name: "AnyIpCidr",
				variant: "V4",
			},
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
		],
	);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::Str("V4"),
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::Bytes(b"V4"),
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::U32(1),
			Token::Struct {
				name: "Ipv4Cidr",
				len: 2,
			},
			Token::Str("address"),
			Token::Tuple { len: 4 },
			Token::U8(192),
			Token::U8(0),
			Token::U8(2),
			Token::U8(0),
			Token::TupleEnd,
			Token::Str("network_length"),
			Token::U8(24),
			Token::StructEnd,
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
			Token::NewtypeVariant {
				name: "AnyIpCidr",
				variant: "V6",
			},
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);

	assert_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::Str("V6"),
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::Bytes(b"V6"),
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);

	assert_de_tokens(
		&c.clone().compact(),
		&[
			Token::Enum { name: "AnyIpCidr" },
			Token::U32(2),
			Token::Struct {
				name: "Ipv6Cidr",
				len: 2,
			},
			Token::Str("address"),
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
			Token::Str("network_length"),
			Token::U8(64),
			Token::StructEnd,
		],
	);
}
