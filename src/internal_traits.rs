use std::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
};

/// Implemented for IPv4Addr, IPv6Addr AND IpAddr
pub trait PrivUnspecAddress: Sized {
	fn _overflowing_next(self, prefix_len: u8) -> (Self, bool);
	fn _overflowing_inc(self, prefix_len: u8, step: u128) -> (Self, bool);
	fn _overflowing_prev(self, prefix_len: u8) -> (Self, bool);
	fn _overflowing_dec(self, prefix_len: u8, step: u128) -> (Self, bool);
	fn _overflowing_sub(self, other: Self) -> (u128, bool);
	fn _prefix_match(self, other: Self, prefix_len: u8) -> bool;
	fn _has_zero_host_part(self, prefix_len: u8) -> bool;
	fn _network_address(self, prefix_len: u8) -> Self;
	fn _last_address(self, prefix_len: u8) -> Self;
}

/// Only implemented for IPv4Addr and IPv6Addr, NOT IpAddr
pub trait PrivAddress: PrivUnspecAddress {
	fn _network_mask(prefix_len: u8) -> Self;
}

/// seal `Cidr` trait
pub trait PrivCidr {}

/// seal `Inet` trait
pub trait PrivInet {}

/// seal `InetPair` trait
pub trait PrivInetPair {}

fn ipv4_host_mask(prefix_len: u8) -> u32 {
	(!0u32).checked_shr(prefix_len as u32).unwrap_or(0)
}

fn u32_overflowing_op<T, F>(address: T, prefix_len: u8, op: F) -> (T, bool)
where
	T: From<u32> + Into<u32>,
	F: FnOnce(u32) -> (u32, bool),
{
	let host_mask = ipv4_host_mask(prefix_len);
	let net_mask = !host_mask;
	let address: u32 = address.into();
	let net_address = address & net_mask;
	let (res, overflow) = op(address);
	if res & net_mask != net_address {
		// replace network in result with original network
		let res = (res & host_mask) | net_address;
		(T::from(res), true)
	} else {
		(T::from(res), overflow)
	}
}

impl PrivUnspecAddress for Ipv4Addr {
	fn _overflowing_next(self, prefix_len: u8) -> (Self, bool) {
		u32_overflowing_op(self, prefix_len, |addr| addr.overflowing_add(1))
	}

	fn _overflowing_inc(self, prefix_len: u8, step: u128) -> (Self, bool) {
		let step_u32 = step as u32;
		let step_overflow = step_u32 as u128 != step;
		let (res, overflow) =
			u32_overflowing_op(self, prefix_len, |addr| addr.overflowing_add(step_u32));
		(res, overflow || step_overflow)
	}

	fn _overflowing_prev(self, prefix_len: u8) -> (Self, bool) {
		u32_overflowing_op(self, prefix_len, |addr| addr.overflowing_sub(1))
	}

	fn _overflowing_dec(self, prefix_len: u8, step: u128) -> (Self, bool) {
		let step_u32 = step as u32;
		let step_overflow = step_u32 as u128 != step;
		let (res, overflow) =
			u32_overflowing_op(self, prefix_len, |addr| addr.overflowing_sub(step_u32));
		(res, overflow || step_overflow)
	}

	fn _overflowing_sub(self, other: Self) -> (u128, bool) {
		let (res, overflow) = u32::from(self).overflowing_sub(u32::from(other));
		(res as u128, overflow)
	}

	fn _prefix_match(self, other: Self, prefix_len: u8) -> bool {
		let net_mask: u32 = !ipv4_host_mask(prefix_len);
		(u32::from(self) & net_mask) == (u32::from(other) & net_mask)
	}

	fn _has_zero_host_part(self, prefix_len: u8) -> bool {
		let host_mask: u32 = ipv4_host_mask(prefix_len);
		(u32::from(self) & host_mask) == 0
	}

	fn _network_address(self, prefix_len: u8) -> Self {
		let net_mask: u32 = !ipv4_host_mask(prefix_len);
		Self::from(u32::from(self) & net_mask)
	}

	fn _last_address(self, prefix_len: u8) -> Self {
		let host_mask: u32 = ipv4_host_mask(prefix_len);
		Self::from(u32::from(self) | host_mask)
	}
}

impl PrivAddress for Ipv4Addr {
	fn _network_mask(prefix_len: u8) -> Self {
		Self::from(!ipv4_host_mask(prefix_len))
	}
}

fn ipv6_host_mask(prefix_len: u8) -> u128 {
	(!0u128).checked_shr(prefix_len as u32).unwrap_or(0)
}

fn u128_overflowing_op<T, F>(address: T, prefix_len: u8, op: F) -> (T, bool)
where
	T: From<u128> + Into<u128>,
	F: FnOnce(u128) -> (u128, bool),
{
	let host_mask = ipv6_host_mask(prefix_len);
	let net_mask = !host_mask;
	let address: u128 = address.into();
	let net_address = address & net_mask;
	let (res, overflow) = op(address);
	if res & net_mask != net_address {
		// replace network in result with original network
		let res = (res & host_mask) | net_address;
		(T::from(res), true)
	} else {
		(T::from(res), overflow)
	}
}

impl PrivUnspecAddress for Ipv6Addr {
	fn _overflowing_next(self, prefix_len: u8) -> (Self, bool) {
		u128_overflowing_op(self, prefix_len, |addr| addr.overflowing_add(1))
	}

	fn _overflowing_inc(self, prefix_len: u8, step: u128) -> (Self, bool) {
		u128_overflowing_op(self, prefix_len, |addr| addr.overflowing_add(step))
	}

	fn _overflowing_prev(self, prefix_len: u8) -> (Self, bool) {
		u128_overflowing_op(self, prefix_len, |addr| addr.overflowing_sub(1))
	}

	fn _overflowing_dec(self, prefix_len: u8, step: u128) -> (Self, bool) {
		u128_overflowing_op(self, prefix_len, |addr| addr.overflowing_sub(step))
	}

	fn _overflowing_sub(self, other: Self) -> (u128, bool) {
		u128::from(self).overflowing_sub(u128::from(other))
	}

	fn _prefix_match(self, other: Self, prefix_len: u8) -> bool {
		let net_mask: u128 = !ipv6_host_mask(prefix_len);
		(u128::from(self) & net_mask) == (u128::from(other) & net_mask)
	}

	fn _has_zero_host_part(self, prefix_len: u8) -> bool {
		let host_mask: u128 = ipv6_host_mask(prefix_len);
		(u128::from(self) & host_mask) == 0
	}

	fn _network_address(self, prefix_len: u8) -> Self {
		let net_mask: u128 = !ipv6_host_mask(prefix_len);
		Self::from(u128::from(self) & net_mask)
	}

	fn _last_address(self, prefix_len: u8) -> Self {
		let host_mask: u128 = ipv6_host_mask(prefix_len);
		Self::from(u128::from(self) | host_mask)
	}
}

impl PrivAddress for Ipv6Addr {
	fn _network_mask(prefix_len: u8) -> Self {
		Self::from(!ipv6_host_mask(prefix_len))
	}
}

impl PrivUnspecAddress for IpAddr {
	fn _overflowing_next(self, prefix_len: u8) -> (Self, bool) {
		match self {
			Self::V4(a) => {
				let (res, of) = a._overflowing_next(prefix_len);
				(Self::V4(res), of)
			},
			Self::V6(a) => {
				let (res, of) = a._overflowing_next(prefix_len);
				(Self::V6(res), of)
			},
		}
	}

	fn _overflowing_inc(self, prefix_len: u8, step: u128) -> (Self, bool) {
		match self {
			Self::V4(a) => {
				let (res, of) = a._overflowing_inc(prefix_len, step);
				(Self::V4(res), of)
			},
			Self::V6(a) => {
				let (res, of) = a._overflowing_inc(prefix_len, step);
				(Self::V6(res), of)
			},
		}
	}

	fn _overflowing_prev(self, prefix_len: u8) -> (Self, bool) {
		match self {
			Self::V4(a) => {
				let (res, of) = a._overflowing_prev(prefix_len);
				(Self::V4(res), of)
			},
			Self::V6(a) => {
				let (res, of) = a._overflowing_prev(prefix_len);
				(Self::V6(res), of)
			},
		}
	}

	fn _overflowing_dec(self, prefix_len: u8, step: u128) -> (Self, bool) {
		match self {
			Self::V4(a) => {
				let (res, of) = a._overflowing_dec(prefix_len, step);
				(Self::V4(res), of)
			},
			Self::V6(a) => {
				let (res, of) = a._overflowing_dec(prefix_len, step);
				(Self::V6(res), of)
			},
		}
	}

	fn _overflowing_sub(self, other: Self) -> (u128, bool) {
		match (self, other) {
			(Self::V4(this), Self::V4(other)) => this._overflowing_sub(other),
			(Self::V6(this), Self::V6(other)) => this._overflowing_sub(other),
			_ => (0, false),
		}
	}

	fn _prefix_match(self, other: Self, prefix_len: u8) -> bool {
		match (self, other) {
			(Self::V4(this), Self::V4(other)) => this._prefix_match(other, prefix_len),
			(Self::V6(this), Self::V6(other)) => this._prefix_match(other, prefix_len),
			_ => false,
		}
	}

	fn _has_zero_host_part(self, prefix_len: u8) -> bool {
		match self {
			Self::V4(a) => a._has_zero_host_part(prefix_len),
			Self::V6(a) => a._has_zero_host_part(prefix_len),
		}
	}

	fn _network_address(self, prefix_len: u8) -> Self {
		match self {
			Self::V4(a) => Self::V4(a._network_address(prefix_len)),
			Self::V6(a) => Self::V6(a._network_address(prefix_len)),
		}
	}

	fn _last_address(self, prefix_len: u8) -> Self {
		match self {
			Self::V4(a) => Self::V4(a._last_address(prefix_len)),
			Self::V6(a) => Self::V6(a._last_address(prefix_len)),
		}
	}
}
