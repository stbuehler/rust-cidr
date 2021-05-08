use super::cidr::*;
use super::inet::*;
use super::inet_pair::*;
use super::traits::Address;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl Address for Ipv4Addr {
	type Inet = Ipv4Inet;
	type Cidr = Ipv4Cidr;
	type InetPair = Ipv4InetPair;
}

impl Address for Ipv6Addr {
	type Inet = Ipv6Inet;
	type Cidr = Ipv6Cidr;
	type InetPair = Ipv6InetPair;
}

impl Address for IpAddr {
	type Inet = IpInet;
	type Cidr = IpCidr;
	type InetPair = IpInetPair;
}
