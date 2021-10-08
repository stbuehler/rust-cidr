use bitstring::BitString;

use crate::{
	AnyIpCidr,
	Ipv4Cidr,
	Ipv6Cidr,
};

#[test]
fn ipv4_clip() {
	let mut c = "192.168.0.0/16".parse::<Ipv4Cidr>().unwrap();
	c.clip(2);
	assert_eq!(c.len(), 2);
	assert_eq!(c, "192.0.0.0/2".parse::<Ipv4Cidr>().unwrap());
}

#[test]
fn ipv4_any_clip() {
	use bitstring::BitString;
	let mut c = "192.168.0.0/16".parse::<AnyIpCidr>().unwrap();
	c.clip(2);
	assert_eq!(c.len(), 2);
	assert_eq!(c, "128.0.0.0/1".parse::<AnyIpCidr>().unwrap());
}

#[test]
fn ipv6_clip() {
	use bitstring::BitString;
	let mut c = "2001:db8:1234:5678::/64".parse::<Ipv6Cidr>().unwrap();
	c.clip(4);
	assert_eq!(c.len(), 4);
	assert_eq!(c, "2000::/4".parse::<Ipv6Cidr>().unwrap());
}

#[test]
fn ipv6_any_clip() {
	use bitstring::BitString;
	let mut c = "2001:db8:1234:5678::/64".parse::<AnyIpCidr>().unwrap();
	c.clip(4);
	assert_eq!(c.len(), 4);
	assert_eq!(c, "2000::/3".parse::<AnyIpCidr>().unwrap());
}
