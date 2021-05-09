use super::cidr::*;
use super::inet::*;
use super::traits::Address;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl Address for Ipv4Addr {
	type Inet = Ipv4Inet;
	type Cidr = Ipv4Cidr;
}

impl Address for Ipv6Addr {
	type Inet = Ipv6Inet;
	type Cidr = Ipv6Cidr;
}

impl Address for IpAddr {
	type Inet = IpInet;
	type Cidr = IpCidr;
}
