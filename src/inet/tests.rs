use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};

use Cidr;
use Inet;
use super::*;

fn test_v4(
	s: &'static str,
	addr: Ipv4Addr,
	first_addr: Ipv4Addr,
	last_addr: Ipv4Addr,
	mask: Ipv4Addr,
	l: u8)
{
	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap(),
		Ipv4Inet{
			address: addr.clone(),
			network_length: l,
		},
		"internal data through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().address(),
		addr.clone(),
		"address through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().first_address(),
		first_addr.clone(),
		"first address through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().network().first_address(),
		first_addr.clone(),
		"first address through Ipv4Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().last_address(),
		last_addr.clone(),
		"last address through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().network().last_address(),
		last_addr.clone(),
		"last address through Ipv4Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv4Inet>().unwrap().mask(),
		mask.clone(),
		"mask through Ipv4Inet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap(),
		IpInet::V4(Ipv4Inet{
			address: addr.clone(),
			network_length: l,
		}),
		"internal data through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().address(),
		IpAddr::V4(addr.clone()),
		"address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().first_address(),
		IpAddr::V4(first_addr.clone()),
		"first address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().first_address(),
		IpAddr::V4(first_addr.clone()),
		"first address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().last_address(),
		IpAddr::V4(last_addr.clone()),
		"last address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().last_address(),
		IpAddr::V4(last_addr.clone()),
		"last address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().mask(),
		IpAddr::V4(mask.clone()),
		"mask through IpInet"
	);
}

fn test_v4_contains(
	s: &'static str,
	addr: Ipv4Addr)
{
	let c1 = s.parse::<Ipv4Inet>().unwrap();
	assert!(
		c1.contains(&addr),
		"{:?} must include {:?} (through Ipv4Inet)", c1, addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		c2.contains(&IpAddr::V4(addr.clone())),
		"{:?} must include {:?} (through IpInet)", c2, addr
	);
}

fn test_v4_contains_not(
	s: &'static str,
	addr: Ipv4Addr)
{
	let c1 = s.parse::<Ipv4Inet>().unwrap();
	assert!(
		!c1.contains(&addr),
		"{:?} must not include {:?} (through Ipv4Inet)", c1, addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		!c2.contains(&IpAddr::V4(addr.clone())),
		"{:?} must not include {:?} (through IpInet)", c2, addr
	);
}

fn test_v6(
	s: &'static str,
	addr: Ipv6Addr,
	first_addr: Ipv6Addr,
	last_addr: Ipv6Addr,
	mask: Ipv6Addr,
	l: u8)
{
	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap(),
		Ipv6Inet{
			address: addr.clone(),
			network_length: l,
		},
		"internal data through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().address(),
		addr.clone(),
		"address through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().first_address(),
		first_addr.clone(),
		"first address through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().network().first_address(),
		first_addr.clone(),
		"first address through Ipv6Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().last_address(),
		last_addr.clone(),
		"last address through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().network().last_address(),
		last_addr.clone(),
		"last address through Ipv6Inet -> network"
	);

	assert_eq!(
		s.parse::<Ipv6Inet>().unwrap().mask(),
		mask.clone(),
		"mask through Ipv6Inet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap(),
		IpInet::V6(Ipv6Inet{
			address: addr.clone(),
			network_length: l,
		}),
		"internal data through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().address(),
		IpAddr::V6(addr.clone()),
		"address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().first_address(),
		IpAddr::V6(first_addr.clone()),
		"first address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().first_address(),
		IpAddr::V6(first_addr.clone()),
		"first address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().last_address(),
		IpAddr::V6(last_addr.clone()),
		"last address through IpInet"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().network().last_address(),
		IpAddr::V6(last_addr.clone()),
		"last address through IpInet -> network"
	);

	assert_eq!(
		s.parse::<IpInet>().unwrap().mask(),
		IpAddr::V6(mask.clone()),
		"mask through IpInet"
	);
}

fn test_v6_contains(
	s: &'static str,
	addr: Ipv6Addr)
{
	let c1 = s.parse::<Ipv6Inet>().unwrap();
	assert!(
		c1.contains(&addr),
		"{:?} must include {:?} (through Ipv6Inet)", c1, addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		c2.contains(&IpAddr::V6(addr.clone())),
		"{:?} must include {:?} (through IpInet)", c2, addr
	);
}

fn test_v6_contains_not(
	s: &'static str,
	addr: Ipv6Addr)
{
	let c1 = s.parse::<Ipv6Inet>().unwrap();
	assert!(
		!c1.contains(&addr),
		"{:?} must not include {:?} (through Ipv6Inet)", c1, addr
	);

	let c2 = s.parse::<IpInet>().unwrap();
	assert!(
		!c2.contains(&IpAddr::V6(addr.clone())),
		"{:?} must not include {:?} (through IpInet)", c2, addr
	);
}

#[test]
#[should_panic(expected = "invalid length for network: Network length 33 is too long for Ipv4 (maximum: 32)")]
fn parse_v4_33bit() {
	"192.0.2.48/33".parse::<Ipv4Inet>().unwrap();
}

#[test]
#[should_panic(expected = "invalid length for network: Network length 33 is too long for Ipv4 (maximum: 32)")]
fn parse_v4_33bit_2() {
	"192.0.2.48/33".parse::<IpInet>().unwrap();
}

#[test]
fn parse_v4_localhost() {
	test_v4(
		"127.0.0.1",
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(127, 0, 0, 1),
		Ipv4Addr::new(255, 255, 255, 255),
		32
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
		32
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
		28
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
		23
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
		23
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
		17
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
		17
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
		8
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
		0
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
		24
	);
}


#[test]
fn contains_v4_24bit() {
	test_v4_contains(
		"192.0.2.0/24",
		Ipv4Addr::new(0xc0, 0x00, 0x02, 0x01)
	);
}

#[test]
fn contains_not_v4_24bit() {
	test_v4_contains_not(
		"192.0.2.0/24",
		Ipv4Addr::new(0x40, 0x00, 0x02, 0x01)
	);
}

#[test]
fn contains_not_v4_24bit_2() {
	test_v4_contains_not(
		"192.0.2.0/24",
		Ipv4Addr::new(0xc0, 0x00, 0x03, 0x01)
	);
}

#[test]
#[should_panic(expected = "invalid length for network: Network length 129 is too long for Ipv6 (maximum: 128)")]
fn parse_v6_129bit() {
	"2001:DB8::/129".parse::<Ipv6Inet>().unwrap();
}

#[test]
#[should_panic(expected = "invalid length for network: Network length 129 is too long for Ipv6 (maximum: 128)")]
fn parse_v6_33bit_2() {
	"2001:DB8::/129".parse::<IpInet>().unwrap();
}

#[test]
fn parse_v6_unspec() {
	test_v6(
		"::",
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0),
		128
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
		128
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
		128
	);
}

#[test]
fn parse_v6_v4_124bit() {
	test_v6(
		"::192.168.4.48/124",
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192*256+168, 4*256+48),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192*256+168, 4*256+48),
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 192*256+168, 4*256+63),
		Ipv6Addr::new(!0, !0, !0, !0, !0, !0, !0, !0-15),
		124
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
		64
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
		64
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
		0
	);
}

#[test]
fn contains_v6_64bit() {
	test_v6_contains(
		"2001:DB8:1234:5678::/64",
		Ipv6Addr::new(0x2001, 0xdb8, 0x1234, 0x5678, 0x1001, 2, 3, 4)
	);
}

#[test]
fn contains_not_v6_64bit() {
	test_v6_contains_not(
		"2001:DB8:1234:5678::/64",
		Ipv6Addr::new(0xa001, 0xdb8, 0x1234, 0x5678, 0x1001, 2, 3, 4)
	);
}

#[test]
fn contains_not_v6_64bit_2() {
	test_v6_contains_not(
		"2001:DB8:1234:5678::/64",
		Ipv6Addr::new(0xa001, 0xdb8, 0x1234, 0x5679, 0x1001, 2, 3, 4)
	);
}
