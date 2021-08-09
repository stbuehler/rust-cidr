/// Number of addresses (in a prefix / range)
///
/// Can be 2^128, so a u128 isn't enough - we need one more.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NumberOfAddresses {
	/// Given amount
	Count(u128),
	/// 2^128 = 340282366920938463463374607431768211456
	MaxIpv6Addresses,
}

impl NumberOfAddresses {
	pub(crate) fn count_from_distance(d: u128) -> Self {
		match d.checked_add(1) {
			Some(c) => Self::Count(c),
			None => Self::MaxIpv6Addresses,
		}
	}
}

impl core::fmt::Display for NumberOfAddresses {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Count(c) => c.fmt(f),
			Self::MaxIpv6Addresses => f.write_str("340282366920938463463374607431768211456"),
		}
	}
}
