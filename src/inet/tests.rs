use core::cmp::Ordering;
use std::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
};

use crate::{
	IpInet,
	Ipv4Inet,
	Ipv6Inet,
};

fn test_v4(
	s: &'static str,
	addr: Ipv4Addr,
	first_addr: Ipv4Addr,
	last_addr: Ipv4Addr,
	mask: Ipv4Addr,
	l: u8,
) {
	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap(),
		Ipv4Inet {
			address: addr,
			network_length: l
		},
		"internal data through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().address(),
		addr,
		"address through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().first_address(),
		first_addr,
		"first address through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().network().first_address(),
		first_addr,
		"first address through Ipv4Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().last_address(),
		last_addr,
		"last address through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().network().last_address(),
		last_addr,
		"last address through Ipv4Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().mask(),
		mask,
		"mask through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap(),
		IpInet::V4(Ipv4Inet {
			address: addr,
			network_length: l
		}),
		"internal data through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().address(),
		IpAddr::V4(addr),
		"address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().first_address(),
		IpAddr::V4(first_addr),
		"first address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().first_address(),
		IpAddr::V4(first_addr),
		"first address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().last_address(),
		IpAddr::V4(last_addr),
		"last address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().last_address(),
		IpAddr::V4(last_addr),
		"last address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().mask(),
		IpAddr::V4(mask),
		"mask through IpInet"
	);
}

fn test_v4_contains(s: &'static str, addr: Ipv4Addr) {
	let c1 = s.parse::<Ipv4Inet>().unwrap();
	assert!(
		c1.contains(&addr),
		"{:?} must include {:?} (through Ipv4Inet)",
		c1,
		addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		c2.contains(&IpAddr::V4(addr)),
		"{:?} must include {:?} (through IpInet)",
		c2,
		addr
	);
}

fn test_v4_contains_not(s: &'static str, addr: Ipv4Addr) {
	let c1 = s.parse::<Ipv4Inet>().unwrap();
	assert!(
		!c1.contains(&addr),
		"{:?} must not include {:?} (through Ipv4Inet)",
		c1,
		addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		!c2.contains(&IpAddr::V4(addr)),
		"{:?} must not include {:?} (through IpInet)",
		c2,
		addr
	);
}

fn test_v6(
	s: &'static str,
	addr: Ipv6Addr,
	first_addr: Ipv6Addr,
	last_addr: Ipv6Addr,
	mask: Ipv6Addr,
	l: u8,
) {
	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap(),
		Ipv6Inet {
			address: addr,
			network_length: l
		},
		"internal data through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().address(),
		addr,
		"address through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().first_address(),
		first_addr,
		"first address through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().network().first_address(),
		first_addr,
		"first address through Ipv6Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().last_address(),
		last_addr,
		"last address through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().network().last_address(),
		last_addr,
		"last address through Ipv6Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().mask(),
		mask,
		"mask through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap(),
		IpInet::V6(Ipv6Inet {
			address: addr,
			network_length: l
		}),
		"internal data through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().address(),
		IpAddr::V6(addr),
		"address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().first_address(),
		IpAddr::V6(first_addr),
		"first address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().first_address(),
		IpAddr::V6(first_addr),
		"first address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().last_address(),
		IpAddr::V6(last_addr),
		"last address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().last_address(),
		IpAddr::V6(last_addr),
		"last address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().mask(),
		IpAddr::V6(mask),
		"mask through IpInet"
	);
}

fn test_v6_contains(s: &'static str, addr: Ipv6Addr) {
	let c1 = s.parse::<Ipv6Inet>().unwrap();
	assert!(
		c1.contains(&addr),
		"{:?} must include {:?} (through Ipv6Inet)",
		c1,
		addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		c2.contains(&IpAddr::V6(addr)),
		"{:?} must include {:?} (through IpInet)",
		c2,
		addr
	);
}

fn test_v6_contains_not(s: &'static str, addr: Ipv6Addr) {
	let c1 = s.parse::<Ipv6Inet>().unwrap();
	assert!(
		!c1.contains(&addr),
		"{:?} must not include {:?} (through Ipv6Inet)",
		c1,
		addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		!c2.contains(&IpAddr::V6(addr)),
		"{:?} must not include {:?} (through IpInet)",
		c2,
		addr
	);
}

fn test_v4_order(o: Ordering, a: &'static str, b: &'static str) {
	let r1 = a
		.parse::<Ipv4Inet>()
		.unwrap()
		.cmp(&b.parse::<Ipv4Inet>().unwrap());
	assert!(
		o == r1,
		"Unexpected comparison outcome '{:?}' for {:?} <=> {:?}, expected '{:?}' (through Ipv4Inet)",
		r1,
		a,
		b,
		o
	);

	let r2 = a
		.parse::<IpInet>()
		.unwrap()
		.cmp(&b.parse::<IpInet>().unwrap());
	assert!(
		o == r2,
		"Unexpected comparison outcome '{:?}' for {:?} <=> {:?}, expected '{:?}' (through IpInet)",
		r2,
		a,
		b,
		o
	);

	if o == Ordering::Less {
		// reverse test
		test_v4_order(Ordering::Greater, b, a);
	}
}

fn test_v6_order(o: Ordering, a: &'static str, b: &'static str) {
	let r1 = a
		.parse::<Ipv6Inet>()
		.unwrap()
		.cmp(&b.parse::<Ipv6Inet>().unwrap());
	assert!(
		o == r1,
		"Unexpected comparison outcome '{:?}' for {:?} <=> {:?}, expected '{:?}' (through Ipv6Inet)",
		r1,
		a,
		b,
		o
	);

	let r2 = a
		.parse::<IpInet>()
		.unwrap()
		.cmp(&b.parse::<IpInet>().unwrap());
	assert!(
		o == r2,
		"Unexpected comparison outcome '{:?}' for {:?} <=> {:?}, expected '{:?}' (through IpInet)",
		r2,
		a,
		b,
		o
	);

	if o == Ordering::Less {
		// reverse test
		test_v6_order(Ordering::Greater, b, a);
	}
}

fn test_order(o: Ordering, a: &'static str, b: &'static str) {
	let r = a
		.parse::<IpInet>()
		.unwrap()
		.cmp(&b.parse::<IpInet>().unwrap());
	assert!(
		o == r,
		"Unexpected comparison outcome '{:?}' for {:?} <=> {:?}, expected '{:?}' (through IpCidr)",
		r,
		a,
		b,
		o
	);

	if o == Ordering::Less {
		// reverse test
		test_order(Ordering::Greater, b, a);
	}
}

#[test]
#[should_panic(
	expected = "invalid length for network: Network length 33 is too long for Ipv4 (maximum: 32)"
)]
fn parse_v4_33bit() {
	"192.0.2.48/33".parse::<Ipv4Inet>().unwrap();
}

#[test]
#[should_panic(
	expected = "invalid length for network: Network length 33 is too long for Ipv4 (maximum: 32)"
)]
fn parse_v4_33bit_2() {
	"192.0.2.48/33".parse::<IpInet>().unwrap();
}

#[test]
fn test_v4_host_address() {
	assert!("127.0.0.1/32"
		.parse::<Ipv4Inet>()
		.unwrap()
		.is_host_address());
	assert!("127.0.0.1/32".parse::<IpInet>().unwrap().is_host_address());
	assert!(!"127.0.0.1/31"
		.parse::<Ipv4Inet>()
		.unwrap()
		.is_host_address());
	assert!(!"127.0.0.1/31".parse::<IpInet>().unwrap().is_host_address());
}

#[test]
fn test_v4_representations_32bit() {
	assert_eq!(
		format!("{}", "127.0.0.1".parse::<Ipv4Inet>().unwrap()),
		"127.0.0.1"
	);

	assert_eq!(
		format!("{}", "127.0.0.1".parse::<IpInet>().unwrap()),
		"127.0.0.1"
	);

	assert_eq!(
		format!("{:?}", "127.0.0.1".parse::<Ipv4Inet>().unwrap()),
		"127.0.0.1/32"
	);

	assert_eq!(
		format!("{:?}", "127.0.0.1".parse::<IpInet>().unwrap()),
		"V4(127.0.0.1/32)"
	);
}

#[test]
fn test_v4_representations_8bit() {
	assert_eq!(
		format!("{}", "10.0.0.0/8".parse::<Ipv4Inet>().unwrap()),
		"10.0.0.0/8"
	);

	assert_eq!(
		format!("{}", "10.0.0.0/8".parse::<IpInet>().unwrap()),
		"10.0.0.0/8"
	);

	assert_eq!(
		format!("{:?}", "10.0.0.0/8".parse::<Ipv4Inet>().unwrap()),
		"10.0.0.0/8"
	);

	assert_eq!(
		format!("{:?}", "10.0.0.0/8".parse::<IpInet>().unwrap()),
		"V4(10.0.0.0/8)"
	);

	// non-zero host part
	assert_eq!(
		format!("{}", "10.4.3.2/8".parse::<Ipv4Inet>().unwrap()),
		"10.4.3.2/8"
	);

	assert_eq!(
		format!("{}", "10.4.3.2/8".parse::<IpInet>().unwrap()),
		"10.4.3.2/8"
	);

	assert_eq!(
		format!("{:?}", "10.4.3.2/8".parse::<Ipv4Inet>().unwrap()),
		"10.4.3.2/8"
	);

	assert_eq!(
		format!("{:?}", "10.4.3.2/8".parse::<IpInet>().unwrap()),
		"V4(10.4.3.2/8)"
	);
}

#[test]
fn test_v4_representations_0bit() {
	assert_eq!(
		format!("{}", "0.0.0.0/0".parse::<Ipv4Inet>().unwrap()),
		"0.0.0.0/0"
	);

	assert_eq!(
		format!("{}", "0.0.0.0/0".parse::<IpInet>().unwrap()),
		"0.0.0.0/0"
	);

	assert_eq!(
		format!("{:?}", "0.0.0.0/0".parse::<Ipv4Inet>().unwrap()),
		"0.0.0.0/0"
	);

	assert_eq!(
		format!("{:?}", "0.0.0.0/0".parse::<IpInet>().unwrap()),
		"V4(0.0.0.0/0)"
	);

	// non-zero host part
	assert_eq!(
		format!("{}", "192.0.2.0/0".parse::<Ipv4Inet>().unwrap()),
		"192.0.2.0/0"
	);

	assert_eq!(
		format!("{}", "192.0.2.0/0".parse::<IpInet>().unwrap()),
		"192.0.2.0/0"
	);

	assert_eq!(
		format!("{:?}", "192.0.2.0/0".parse::<Ipv4Inet>().unwrap()),
		"192.0.2.0/0"
	);

	assert_eq!(
		format!("{:?}", "192.0.2.0/0".parse::<IpInet>().unwrap()),
		"V4(192.0.2.0/0)"
	);
}

#[test]
fn parse_v4_localhost() {
	test_v4(
		"127.0.0.1",
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(255, 255, 255, 255),
		32,
	);
}

#[test]
fn parse_v4_localhost_32() {
	test_v4(
		"127.0.0.1/32",
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(255, 255, 255, 255),
		32,
	);
}

#[test]
fn parse_v4_28bit() {
	test_v4(
		"192.0.2.48/28",
		Ipv4Addr::new(192, 0, 2, 48),
		Ipv4Addr::new(192, 0, 2, 48),
		Ipv4Addr::new(192, 0, 2, 63),
		Ipv4Addr::new(255, 255, 255, 240),
		28,
	);
}

#[test]
fn parse_v4_23bit() {
	test_v4(
		"192.0.2.0/23",
		Ipv4Addr::new(192, 0, 2, 0),
		Ipv4Addr::new(192, 0, 2, 0),
		Ipv4Addr::new(192, 0, 3, 255),
		Ipv4Addr::new(255, 255, 254, 0),
		23,
	);
}

#[test]
fn parse_v4_23bit_non_zero_host_bits() {
	test_v4(
		"192.0.3.0/23",
		Ipv4Addr::new(192, 0, 3, 0),
		Ipv4Addr::new(192, 0, 2, 0),
		Ipv4Addr::new(192, 0, 3, 255),
		Ipv4Addr::new(255, 255, 254, 0),
		23,
	);
}

#[test]
fn parse_v4_17bit() {
	test_v4(
		"192.0.128.0/17",
		Ipv4Addr::new(192, 0, 128, 0),
		Ipv4Addr::new(192, 0, 128, 0),
		Ipv4Addr::new(192, 0, 255, 255),
		Ipv4Addr::new(255, 255, 128, 0),
		17,
	);
}

#[test]
fn parse_v4_17bit_non_zero_host_bits() {
	test_v4(
		"192.0.192.0/17",
		Ipv4Addr::new(192, 0, 192, 0),
		Ipv4Addr::new(192, 0, 128, 0),
		Ipv4Addr::new(192, 0, 255, 255),
		Ipv4Addr::new(255, 255, 128, 0),
		17,
	);
}

#[test]
fn parse_v4_8bit() {
	test_v4(
		"10.0.0.0/8",
		Ipv4Addr::new(10, 0, 0, 0),
		Ipv4Addr::new(10, 0, 0, 0),
		Ipv4Addr::new(10, 255, 255, 255),
		Ipv4Addr::new(255, 0, 0, 0),
		8,
	);
}

#[test]
fn parse_v4_0bit() {
	test_v4(
		"0.0.0.0/0",
		Ipv4Addr::new(0, 0, 0, 0),
		Ipv4Addr::new(0, 0, 0, 0),
		Ipv4Addr::new(255, 255, 255, 255),
		Ipv4Addr::new(0, 0, 0, 0),
		0,
	);
}

#[test]
fn parse_v4_non_zero_host_bits() {
	test_v4(
		"10.1.1.1/24",
		Ipv4Addr::new(10, 1, 1, 1),
		Ipv4Addr::new(10, 1, 1, 0),
		Ipv4Addr::new(10, 1, 1, 255),
		Ipv4Addr::new(255, 255, 255, 0),
		24,
	);
}

#[test]
fn contains_v4_24bit() {
	test_v4_contains("192.0.2.0/24", Ipv4Addr::new(0xc0, 0x00, 0x02, 0x01));
}

#[test]
fn contains_not_v4_24bit() {
	test_v4_contains_not("192.0.2.0/24", Ipv4Addr::new(0x40, 0x00, 0x02, 0x01));
}

#[test]
fn contains_not_v4_24bit_2() {
	test_v4_contains_not("192.0.2.0/24", Ipv4Addr::new(0xc0, 0x00, 0x03, 0x01));
}

#[test]
#[should_panic(
	expected = "invalid length for network: Network length 129 is too long for Ipv6 (maximum: 128)"
)]
fn parse_v6_129bit() {
	"2001:DB8::/129".parse::<Ipv6Inet>().unwrap();
}

#[test]
#[should_panic(
	expected = "invalid length for network: Network length 129 is too long for Ipv6 (maximum: 128)"
)]
fn parse_v6_33bit_2() {
	"2001:DB8::/129".parse::<IpInet>().unwrap();
}

#[test]
fn test_v6_host_address() {
	assert!("::/128".parse::<Ipv6Inet>().unwrap().is_host_address());
	assert!("::/128".parse::<IpInet>().unwrap().is_host_address());
	assert!(!"::/127".parse::<Ipv6Inet>().unwrap().is_host_address());
	assert!(!"::/127".parse::<IpInet>().unwrap().is_host_address());
}

#[test]
fn test_v6_representations_128bit() {
	assert_eq!(format!("{}", "::".parse::<Ipv6Inet>().unwrap()), "::");

	assert_eq!(format!("{}", "::".parse::<IpInet>().unwrap()), "::");

	assert_eq!(format!("{:?}", "::".parse::<Ipv6Inet>().unwrap()), "::/128");

	assert_eq!(
		format!("{:?}", "::".parse::<IpInet>().unwrap()),
		"V6(::/128)"
	);
}

#[test]
fn test_v6_representations_64bit() {
	assert_eq!(
		format!("{}", "2001:DB8:1234:5678::/64".parse::<Ipv6Inet>().unwrap()),
		"2001:db8:1234:5678::/64"
	);

	assert_eq!(
		format!("{}", "2001:DB8:1234:5678::/64".parse::<IpInet>().unwrap()),
		"2001:db8:1234:5678::/64"
	);

	assert_eq!(
		format!(
			"{:?}",
			"2001:DB8:1234:5678::/64".parse::<Ipv6Inet>().unwrap()
		),
		"2001:db8:1234:5678::/64"
	);

	assert_eq!(
		format!("{:?}", "2001:DB8:1234:5678::/64".parse::<IpInet>().unwrap()),
		"V6(2001:db8:1234:5678::/64)"
	);

	// non-zero host part
	assert_eq!(
		format!(
			"{}",
			"2001:DB8:1234:5678:1:2:3:4/64".parse::<Ipv6Inet>().unwrap()
		),
		"2001:db8:1234:5678:1:2:3:4/64"
	);

	assert_eq!(
		format!(
			"{}",
			"2001:DB8:1234:5678:1:2:3:4/64".parse::<IpInet>().unwrap()
		),
		"2001:db8:1234:5678:1:2:3:4/64"
	);

	assert_eq!(
		format!(
			"{:?}",
			"2001:DB8:1234:5678:1:2:3:4/64".parse::<Ipv6Inet>().unwrap()
		),
		"2001:db8:1234:5678:1:2:3:4/64"
	);

	assert_eq!(
		format!(
			"{:?}",
			"2001:DB8:1234:5678:1:2:3:4/64".parse::<IpInet>().unwrap()
		),
		"V6(2001:db8:1234:5678:1:2:3:4/64)"
	);
}

#[test]
fn test_v6_representations_0bit() {
	assert_eq!(format!("{}", "::/0".parse::<Ipv6Inet>().unwrap()), "::/0");

	assert_eq!(format!("{}", "::/0".parse::<IpInet>().unwrap()), "::/0");

	assert_eq!(format!("{:?}", "::/0".parse::<Ipv6Inet>().unwrap()), "::/0");

	assert_eq!(
		format!("{:?}", "::/0".parse::<IpInet>().unwrap()),
		"V6(::/0)"
	);

	// non-zero host part
	assert_eq!(
		format!(
			"{}",
			"2001:DB8:1234:5678:1:2:3:4/0".parse::<Ipv6Inet>().unwrap()
		),
		"2001:db8:1234:5678:1:2:3:4/0"
	);

	assert_eq!(
		format!(
			"{}",
			"2001:DB8:1234:5678:1:2:3:4/0".parse::<IpInet>().unwrap()
		),
		"2001:db8:1234:5678:1:2:3:4/0"
	);

	assert_eq!(
		format!(
			"{:?}",
			"2001:DB8:1234:5678:1:2:3:4/0".parse::<Ipv6Inet>().unwrap()
		),
		"2001:db8:1234:5678:1:2:3:4/0"
	);

	assert_eq!(
		format!(
			"{:?}",
			"2001:DB8:1234:5678:1:2:3:4/0".parse::<IpInet>().unwrap()
		),
		"V6(2001:db8:1234:5678:1:2:3:4/0)"
	);
}

#[test]
fn parse_v6_unspec() {
	test_v6(
		"::",
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0),
		128,
	);
}

#[test]
fn parse_v6_localhost() {
	test_v6(
		"::1",
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0),
		128,
	);
}

#[test]
fn parse_v6_localhost_128() {
	test_v6(
		"::1/128",
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0),
		128,
	);
}

#[test]
fn parse_v6_v4_124bit() {
	test_v6(
		"::192.168.4.48/124",
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 48),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 48),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192 * 256 + 168, 4 * 256 + 63),
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0 - 15),
		124,
	);
}

#[test]
fn parse_v6_64bit() {
	test_v6(
		"2001:DB8:1234:5678::/64",
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, 0, 0, 0, 0),
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, 0, 0, 0, 0),
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, !0, !0, !0, !0),
		Ipv6Addr::new(!0, !0, !0, !0, 0, 0, 0, 0),
		64,
	);
}

#[test]
fn parse_v6_non_zero_host_bits() {
	test_v6(
		"2001:DB8:1234:5678:1::/64",
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, 1, 0, 0, 0),
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, 0, 0, 0, 0),
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, !0, !0, !0, !0),
		Ipv6Addr::new(!0, !0, !0, !0, 0, 0, 0, 0),
		64,
	);
}

#[test]
fn parse_v6_0bit() {
	test_v6(
		"::/0",
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		0,
	);
}

#[test]
fn contains_v6_64bit() {
	test_v6_contains(
		"2001:DB8:1234:5678::/64",
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, 0x1001, 2, 3, 4),
	);
}

#[test]
fn contains_not_v6_64bit() {
	test_v6_contains_not(
		"2001:DB8:1234:5678::/64",
		Ipv6Addr::new(0xa001, 0xdb8, 0x1234, 0x5678, 0x1001, 2, 3, 4),
	);
}

#[test]
fn contains_not_v6_64bit_2() {
	test_v6_contains_not(
		"2001:DB8:1234:5678::/64",
		Ipv6Addr::new(0xa001, 0xdb8, 0x1234, 0x5679, 0x1001, 2, 3, 4),
	);
}

#[test]
fn order_v4() {
	test_v4_order(Ordering::Equal, "192.0.2.0/24", "192.0.2.0/24");
	test_v4_order(Ordering::Less, "192.0.2.0/24", "192.0.3.0/24");
	test_v4_order(Ordering::Less, "192.0.2.0/24", "192.0.2.0/25");
	test_v4_order(Ordering::Less, "192.0.2.0/24", "192.0.2.128/25");

	test_v4_order(Ordering::Equal, "192.0.2.42/24", "192.0.2.42/24");
	test_v4_order(Ordering::Less, "192.0.2.41/16", "192.0.2.42/24");
	test_v4_order(Ordering::Less, "192.0.2.41/24", "192.0.2.42/24");
	test_v4_order(Ordering::Less, "192.0.2.42/24", "192.0.2.42/25");
	test_v4_order(Ordering::Less, "192.0.2.42/24", "192.0.2.170/25");
}

#[test]
fn order_v6() {
	test_v6_order(
		Ordering::Equal,
		"2001:DB8:1234:5678::/64",
		"2001:DB8:1234:5678::/64",
	);
	test_v6_order(
		Ordering::Less,
		"2001:DB8:1234:5678:1000::/80",
		"2001:DB8:1234:5678:1001::/80",
	);
	test_v6_order(
		Ordering::Less,
		"2001:DB8:1234:5678:1000::/80",
		"2001:DB8:1234:5678:1000::/81",
	);
	test_v6_order(
		Ordering::Less,
		"2001:DB8:1234:5678:1000::/80",
		"2001:DB8:1234:5678:1000:8000::/81",
	);

	test_v6_order(
		Ordering::Equal,
		"2001:DB8:1234:5678::42/64",
		"2001:DB8:1234:5678::42/64",
	);
	test_v6_order(
		Ordering::Less,
		"2001:DB8:1234:5678:1000::41/48",
		"2001:DB8:1234:5678:1000::42/80",
	);
	test_v6_order(
		Ordering::Less,
		"2001:DB8:1234:5678:1000::41/80",
		"2001:DB8:1234:5678:1000::42/80",
	);
	test_v6_order(
		Ordering::Less,
		"2001:DB8:1234:5678:1000::42/80",
		"2001:DB8:1234:5678:1000::42/81",
	);
	test_v6_order(
		Ordering::Less,
		"2001:DB8:1234:5678:1000::42/80",
		"2001:DB8:1234:5678:1000:8000::42/81",
	);
}

#[test]
fn order() {
	test_order(Ordering::Less, "192.0.2.0/24", "2001:DB8:1234:5678::/64");
}
