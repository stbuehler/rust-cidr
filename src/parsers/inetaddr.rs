use std::net::{
	AddrParseError,
	IpAddr,
	Ipv4Addr,
};

/// Parse "loose" IPv4 address similar to POSIX `inet_addr`
///
/// This also accepts inputs that are not 4 decimal represented octects separated by dots.
///
/// * Allows less than 4 numbers separated by dots; the last number is
///   interpreted as an 32-bit (no dot), 24-bit (1 dot), 16-bit (2 dots) number.
///   Other numbers still must be octets (i.e. 8 bit)
/// * Allows hexadecimal ("0x" or "0X" prefix) and octal ("0" prefix) numbers
///   in all places.
///
/// See <https://pubs.opengroup.org/onlinepubs/9699919799/functions/inet_addr.html>
pub fn inet_addr(s: &str) -> Option<Ipv4Addr> {
	let mut slots = [0u32; 4];
	let mut last_slot = 0;
	for (ndx, part) in s.split('.').enumerate() {
		if ndx >= 4 {
			// too many '.'
			return None;
		}
		let slot = if part.starts_with("0x") || part.starts_with("0X") {
			let part = &part[2..];
			if part.starts_with('+') {
				// we don't want to support "0x+..."
				return None;
			}
			u32::from_str_radix(part, 16)
		} else if part.starts_with('0') {
			u32::from_str_radix(part, 8)
		} else {
			part.parse::<u32>()
		};
		slots[ndx] = slot.ok()?;
		last_slot = ndx;
	}
	debug_assert!(last_slot <= 3, "can't have more than 4");

	let num = if last_slot == 0 {
		slots[0]
	} else {
		let mut base: u32 = 0;
		for (ndx, &slot) in slots.iter().enumerate().take(last_slot) {
			if slot >= 256 {
				// leading parts must be octects
				return None;
			}
			// shift by 24, 16 or 8
			base |= slots[ndx] << ((3 - ndx) * 8);
		}
		// last 3 => have 4 => 1 byte, last 2 => have 3 => 2 bytes, last 1 => have 2 => 3 bytes
		let last_slot_bit_limit = (4 - last_slot) * 8;
		if slots[last_slot] >= 1 << last_slot_bit_limit {
			// last part too big
			return None;
		}
		// never shift last slot
		base | slots[last_slot]
	};

	Some(Ipv4Addr::from(num))
}

/// Parse IPv4 address; fall back to `inet_addr` if normal parser fails
pub fn parse_loose_ipv4(s: &str) -> Result<Ipv4Addr, AddrParseError> {
	match s.parse() {
		Ok(a) => Ok(a),
		Err(e) => {
			if let Some(a) = inet_addr(s) {
				Ok(a)
			} else {
				Err(e)
			}
		},
	}
}

/// Parse IP address; fall back to `inet_addr` if normal parser fails
pub fn parse_loose_ip(s: &str) -> Result<IpAddr, AddrParseError> {
	match s.parse() {
		Ok(a) => Ok(a),
		Err(e) => {
			if let Some(a) = inet_addr(s) {
				Ok(IpAddr::V4(a))
			} else {
				Err(e)
			}
		},
	}
}

#[cfg(test)]
mod tests {
	use std::net::{
		AddrParseError,
		IpAddr,
		Ipv4Addr,
	};

	fn run(s: &str, expect: Ipv4Addr) {
		assert_eq!(super::inet_addr(s), Some(expect));
		assert_eq!(super::parse_loose_ipv4(s), Ok::<_, AddrParseError>(expect));
		assert_eq!(
			super::parse_loose_ip(s),
			Ok::<_, AddrParseError>(IpAddr::V4(expect))
		);
	}

	#[test]
	fn test_loose_ipv4() {
		run("10.0.1.2", Ipv4Addr::from([10, 0, 1, 2]));
		run("10.0.258", Ipv4Addr::from([10, 0, 1, 2]));
		run("0xA000102", Ipv4Addr::from([10, 0, 1, 2]));
		run("0XA000102", Ipv4Addr::from([10, 0, 1, 2]));
		run("012.0x102", Ipv4Addr::from([10, 0, 1, 2]));

		run("127.0.0.1", Ipv4Addr::from([127, 0, 0, 1]));
		run("127.0.1", Ipv4Addr::from([127, 0, 0, 1]));
		run("127.1", Ipv4Addr::from([127, 0, 0, 1]));

		run("0", Ipv4Addr::from([0, 0, 0, 0]));
		run("1", Ipv4Addr::from([0, 0, 0, 1]));
	}
}
