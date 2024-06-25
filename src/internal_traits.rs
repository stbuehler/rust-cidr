use std::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
};

use crate::num::NumberOfAddresses;

/// Implemented for IPv4Addr, IPv6Addr AND IpAddr
pub trait PrivUnspecAddress: Sized {
	type _Tools;
}

/// seal `Cidr` trait
pub trait PrivCidr {}

/// seal `Inet` trait
pub trait PrivInet {}

/// seal `InetPair` trait
pub trait PrivInetPair {
	fn _covered_addresses(&self) -> NumberOfAddresses;
	fn _inc_first(&mut self) -> bool;
	fn _dec_second(&mut self) -> bool;
}

#[derive(Clone, Copy)]
struct Ipv4OverflowingOp {
	address: u32,
	net_mask: u32,
	host_mask: u32,
}

impl Ipv4OverflowingOp {
	const fn new(address: Ipv4Addr, prefix_len: u8) -> Self {
		let host_mask = Ipv4AddrTools::native_host_mask(prefix_len);
		let net_mask = !host_mask;
		let address: u32 = Ipv4AddrTools::to_native(address);
		Self {
			address,
			net_mask,
			host_mask,
		}
	}

	const fn handle_result(self, (mut res, mut overflow): (u32, bool)) -> (Ipv4Addr, bool) {
		let net_address = self.address & self.net_mask;
		if res & self.net_mask != net_address {
			// replace network in result with original network
			res = (res & self.host_mask) | net_address;
			overflow = true
		}
		(Ipv4AddrTools::from_native(res), overflow)
	}
}

pub struct Ipv4AddrTools(());

impl Ipv4AddrTools {
	pub(crate) const fn to_native(ip: Ipv4Addr) -> u32 {
		// const: u32::from
		u32::from_be_bytes(ip.octets())
	}

	pub(crate) const fn from_native(ip: u32) -> Ipv4Addr {
		// const: Ipv4Addr::from
		let ip = ip.to_be_bytes();
		Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3])
	}

	pub(crate) const fn native_host_mask(prefix_len: u8) -> u32 {
		// const: unwrap_or(0)
		if let Some(mask) = (!0u32).checked_shr(prefix_len as u32) {
			mask
		} else {
			0
		}
	}

	pub(crate) const fn _has_zero_host_part(address: Ipv4Addr, prefix_len: u8) -> bool {
		let host_mask: u32 = Self::native_host_mask(prefix_len);
		let addr_num = Self::to_native(address);
		(addr_num & host_mask) == 0
	}

	pub(crate) const fn _overflowing_next(address: Ipv4Addr, prefix_len: u8) -> (Ipv4Addr, bool) {
		let op = Ipv4OverflowingOp::new(address, prefix_len);
		op.handle_result(op.address.overflowing_add(1))
	}

	pub(crate) const fn _overflowing_inc_u32(
		address: Ipv4Addr,
		prefix_len: u8,
		step_u32: u32,
	) -> (Ipv4Addr, bool) {
		let op = Ipv4OverflowingOp::new(address, prefix_len);
		let (res, overflow) = op.handle_result(op.address.overflowing_add(step_u32));
		(res, overflow)
	}

	pub(crate) const fn _overflowing_inc(
		address: Ipv4Addr,
		prefix_len: u8,
		step: u128,
	) -> (Ipv4Addr, bool) {
		let step_u32 = step as u32;
		let step_overflow = step_u32 as u128 != step;
		let (res, overflow) = Self::_overflowing_inc_u32(address, prefix_len, step_u32);
		(res, overflow || step_overflow)
	}

	pub(crate) const fn _overflowing_prev(address: Ipv4Addr, prefix_len: u8) -> (Ipv4Addr, bool) {
		let op = Ipv4OverflowingOp::new(address, prefix_len);
		op.handle_result(op.address.overflowing_sub(1))
	}

	pub(crate) const fn _overflowing_dec_u32(
		address: Ipv4Addr,
		prefix_len: u8,
		step_u32: u32,
	) -> (Ipv4Addr, bool) {
		let op = Ipv4OverflowingOp::new(address, prefix_len);
		let (res, overflow) = op.handle_result(op.address.overflowing_sub(step_u32));
		(res, overflow)
	}

	pub(crate) const fn _overflowing_dec(
		address: Ipv4Addr,
		prefix_len: u8,
		step: u128,
	) -> (Ipv4Addr, bool) {
		let step_u32 = step as u32;
		let step_overflow = step_u32 as u128 != step;
		let (res, overflow) = Self::_overflowing_dec_u32(address, prefix_len, step_u32);
		(res, overflow || step_overflow)
	}

	/*
		pub(crate) const fn _overflowing_sub(address: Ipv4Addr, other: Ipv4Addr) -> (u128, bool) {
			let (res, overflow) = Self::to_native(address).overflowing_sub(Self::to_native(other));
			(res as u128, overflow)
		}
	*/

	pub(crate) const fn _prefix_match(address: Ipv4Addr, other: Ipv4Addr, prefix_len: u8) -> bool {
		let net_mask: u32 = !Self::native_host_mask(prefix_len);
		(Self::to_native(address) & net_mask) == (Self::to_native(other) & net_mask)
	}

	pub(crate) const fn _network_address(address: Ipv4Addr, prefix_len: u8) -> Ipv4Addr {
		let net_mask: u32 = !Self::native_host_mask(prefix_len);
		Self::from_native(Self::to_native(address) & net_mask)
	}

	pub(crate) const fn _last_address(address: Ipv4Addr, prefix_len: u8) -> Ipv4Addr {
		let host_mask: u32 = Self::native_host_mask(prefix_len);
		Self::from_native(Self::to_native(address) | host_mask)
	}

	pub(crate) const fn _network_mask(prefix_len: u8) -> Ipv4Addr {
		Self::from_native(!Self::native_host_mask(prefix_len))
	}
}

impl PrivUnspecAddress for Ipv4Addr {
	type _Tools = Ipv4AddrTools;
}

#[derive(Clone, Copy)]
struct Ipv6OverflowingOp {
	address: u128,
	net_mask: u128,
	host_mask: u128,
}

impl Ipv6OverflowingOp {
	const fn new(address: Ipv6Addr, prefix_len: u8) -> Self {
		let host_mask = Ipv6AddrTools::native_host_mask(prefix_len);
		let net_mask = !host_mask;
		let address: u128 = u128::from_be_bytes(address.octets());
		Self {
			address,
			net_mask,
			host_mask,
		}
	}

	const fn handle_result(self, (mut res, mut overflow): (u128, bool)) -> (Ipv6Addr, bool) {
		let net_address = self.address & self.net_mask;
		if res & self.net_mask != net_address {
			// replace network in result with original network
			res = (res & self.host_mask) | net_address;
			overflow = true
		}
		(Ipv6AddrTools::from_native(res), overflow)
	}
}

pub struct Ipv6AddrTools(());

impl Ipv6AddrTools {
	pub(crate) const fn to_native(ip: Ipv6Addr) -> u128 {
		// const: u128::from
		u128::from_be_bytes(ip.octets())
	}

	pub(crate) const fn from_native(ip: u128) -> Ipv6Addr {
		// const: Ipv6Addr::from
		let ip = ip.to_be_bytes();
		#[cfg(not(feature = "no_unsafe"))]
		{
			// Ipv6Addr is a newtype for the octets, it just doesn't have a constructor
			// for it apart from Ipv6Addr::from, which isn't const
			//
			// this should be "safe":
			// * if Ipv6Addr has a different size than [u8; 16] transmute won't compile
			// * if the size matches, Ipv6Addr can't store any other data - it must store exactly the octets
			// * it is unlikely std would ever store this in another endianness
			unsafe { std::mem::transmute(ip) }
		}
		#[cfg(feature = "no_unsafe")]
		{
			// safe variant, but "slow"
			Ipv6Addr::new(
				u16::from_be_bytes([ip[0], ip[1]]),
				u16::from_be_bytes([ip[2], ip[3]]),
				u16::from_be_bytes([ip[4], ip[5]]),
				u16::from_be_bytes([ip[6], ip[7]]),
				u16::from_be_bytes([ip[8], ip[9]]),
				u16::from_be_bytes([ip[10], ip[11]]),
				u16::from_be_bytes([ip[12], ip[13]]),
				u16::from_be_bytes([ip[14], ip[15]]),
			)
		}
	}

	pub(crate) const fn native_host_mask(prefix_len: u8) -> u128 {
		// const: unwrap_or(0)
		if let Some(mask) = (!0u128).checked_shr(prefix_len as u32) {
			mask
		} else {
			0
		}
	}

	pub(crate) const fn _has_zero_host_part(address: Ipv6Addr, prefix_len: u8) -> bool {
		let host_mask: u128 = Self::native_host_mask(prefix_len);
		let addr_num = Self::to_native(address);
		(addr_num & host_mask) == 0
	}

	pub(crate) const fn _overflowing_next(address: Ipv6Addr, prefix_len: u8) -> (Ipv6Addr, bool) {
		let op = Ipv6OverflowingOp::new(address, prefix_len);
		op.handle_result(op.address.overflowing_add(1))
	}

	pub(crate) const fn _overflowing_inc(
		address: Ipv6Addr,
		prefix_len: u8,
		step: u128,
	) -> (Ipv6Addr, bool) {
		let op = Ipv6OverflowingOp::new(address, prefix_len);
		op.handle_result(op.address.overflowing_add(step))
	}

	pub(crate) const fn _overflowing_prev(address: Ipv6Addr, prefix_len: u8) -> (Ipv6Addr, bool) {
		let op = Ipv6OverflowingOp::new(address, prefix_len);
		op.handle_result(op.address.overflowing_sub(1))
	}

	pub(crate) const fn _overflowing_dec(
		address: Ipv6Addr,
		prefix_len: u8,
		step: u128,
	) -> (Ipv6Addr, bool) {
		let op = Ipv6OverflowingOp::new(address, prefix_len);
		op.handle_result(op.address.overflowing_sub(step))
	}

	/*
		pub(crate) const fn _overflowing_sub(address: Ipv6Addr, other: Ipv6Addr) -> (u128, bool) {
			Self::to_native(address).overflowing_sub(Self::to_native(other))
		}
	*/

	pub(crate) const fn _prefix_match(address: Ipv6Addr, other: Ipv6Addr, prefix_len: u8) -> bool {
		let net_mask: u128 = !Self::native_host_mask(prefix_len);
		(Self::to_native(address) & net_mask) == (Self::to_native(other) & net_mask)
	}

	pub(crate) const fn _network_address(address: Ipv6Addr, prefix_len: u8) -> Ipv6Addr {
		let net_mask: u128 = !Self::native_host_mask(prefix_len);
		Self::from_native(Self::to_native(address) & net_mask)
	}

	pub(crate) const fn _last_address(address: Ipv6Addr, prefix_len: u8) -> Ipv6Addr {
		let host_mask: u128 = Self::native_host_mask(prefix_len);
		Self::from_native(Self::to_native(address) | host_mask)
	}

	pub(crate) const fn _network_mask(prefix_len: u8) -> Ipv6Addr {
		Self::from_native(!Self::native_host_mask(prefix_len))
	}
}

impl PrivUnspecAddress for Ipv6Addr {
	type _Tools = Ipv6AddrTools;
}

pub struct IpAddrTools(());

// we currently don't use the tools on IpAddr and related enums
impl IpAddrTools {
	/*
		pub(crate) const fn _has_zero_host_part(address: IpAddr, prefix_len: u8) -> bool {
			match address {
				IpAddr::V4(a) => Ipv4AddrTools::_has_zero_host_part(a, prefix_len),
				IpAddr::V6(a) => Ipv6AddrTools::_has_zero_host_part(a, prefix_len),
			}
		}

		pub(crate) const fn _overflowing_next(address: IpAddr, prefix_len: u8) -> (IpAddr, bool) {
			match address {
				IpAddr::V4(a) => {
					let (res, of) = Ipv4AddrTools::_overflowing_next(a, prefix_len);
					(IpAddr::V4(res), of)
				},
				IpAddr::V6(a) => {
					let (res, of) = Ipv6AddrTools::_overflowing_next(a, prefix_len);
					(IpAddr::V6(res), of)
				},
			}
		}

		pub(crate) const fn _overflowing_inc(address: IpAddr, prefix_len: u8, step: u128) -> (IpAddr, bool) {
			match address {
				IpAddr::V4(a) => {
					let (res, of) = Ipv4AddrTools::_overflowing_inc(a, prefix_len, step);
					(IpAddr::V4(res), of)
				},
				IpAddr::V6(a) => {
					let (res, of) = Ipv6AddrTools::_overflowing_inc(a, prefix_len, step);
					(IpAddr::V6(res), of)
				},
			}
		}

		pub(crate) const fn _overflowing_prev(address: IpAddr, prefix_len: u8) -> (IpAddr, bool) {
			match address {
				IpAddr::V4(a) => {
					let (res, of) = Ipv4AddrTools::_overflowing_prev(a, prefix_len);
					(IpAddr::V4(res), of)
				},
				IpAddr::V6(a) => {
					let (res, of) = Ipv6AddrTools::_overflowing_prev(a, prefix_len);
					(IpAddr::V6(res), of)
				},
			}
		}

		pub(crate) const fn _overflowing_dec(address: IpAddr, prefix_len: u8, step: u128) -> (IpAddr, bool) {
			match address {
				IpAddr::V4(a) => {
					let (res, of) = Ipv4AddrTools::_overflowing_dec(a, prefix_len, step);
					(IpAddr::V4(res), of)
				},
				IpAddr::V6(a) => {
					let (res, of) = Ipv6AddrTools::_overflowing_dec(a, prefix_len, step);
					(IpAddr::V6(res), of)
				},
			}
		}
	*/

	/*
		pub(crate) const fn _overflowing_sub(address: IpAddr, other: IpAddr) -> (u128, bool) {
			match (address, other) {
				(IpAddr::V4(a), IpAddr::V4(other)) => Ipv4AddrTools::_overflowing_sub(a, other),
				(IpAddr::V6(a), IpAddr::V6(other)) => Ipv6AddrTools::_overflowing_sub(a, other),
				_ => (0, false),
			}
		}
	*/

	/*
		fn _prefix_match(address: IpAddr, other: IpAddr, prefix_len: u8) -> bool {
			match (address, other) {
				(IpAddr::V4(a), IpAddr::V4(other)) => {
					Ipv4AddrTools::_prefix_match(a, other, prefix_len)
				},
				(IpAddr::V6(a), IpAddr::V6(other)) => {
					Ipv6AddrTools::_prefix_match(a, other, prefix_len)
				},
				_ => false,
			}
		}

		pub(crate) const fn _network_address(address: IpAddr, prefix_len: u8) -> IpAddr {
			match address {
				IpAddr::V4(a) => IpAddr::V4(Ipv4AddrTools::_network_address(a, prefix_len)),
				IpAddr::V6(a) => IpAddr::V6(Ipv6AddrTools::_network_address(a, prefix_len)),
			}
		}

		fn _last_address(address: IpAddr, prefix_len: u8) -> IpAddr {
			match address {
				IpAddr::V4(a) => IpAddr::V4(Ipv4AddrTools::_last_address(a, prefix_len)),
				IpAddr::V6(a) => IpAddr::V6(Ipv6AddrTools::_last_address(a, prefix_len)),
			}
		}
	*/
}

impl PrivUnspecAddress for IpAddr {
	type _Tools = IpAddrTools;
}
