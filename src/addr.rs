//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[cfg(feature = "database")]
use producer;
use producer::Producer;

/// A MAC address.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HwAddr {
	value: [u8; 6]
}

impl HwAddr {
	/// Get the octets composing the MAC address.
	///
	/// # Example
	///
	/// ```
	/// use hwaddr::HwAddr;
	///
	/// assert_eq!(
	/// 	"00-14-22-01-23-45".parse::<HwAddr>().unwrap().octets(),
	/// 	[0, 20, 34, 1, 35, 69]);
	/// ```
	pub fn octets(&self) -> [u8; 6] {
		self.value
	}

	/// Checks if the address is broadcast.
	///
	/// # Example
	/// ```
	/// use hwaddr::HwAddr;
	///
	/// assert!("FF:FF:FF:FF:FF:FF".parse::<HwAddr>().unwrap().is_broadcast());
	/// assert!(!"00:00:00:00:00:00".parse::<HwAddr>().unwrap().is_broadcast());
	/// ```
	pub fn is_broadcast(&self) -> bool {
		self.value == [0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
	}

	/// Tries to find the producer for this MAC address.
	pub fn producer(&self) -> Option<&Producer> {
		#[cfg(feature = "database")]
		{
			producer::DATABASE.get(&self.value[0 .. 3])
		}

		#[cfg(not(feature = "database"))]
		{
			None
		}
	}
}

impl FromStr for HwAddr {
	type Err = ParseIntError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let mut result = [0; 6];

		for (i, byte) in value.split(|c| c == ':' || c == '-').enumerate() {
			if i > 5 {
				u8::from_str_radix("error", 10)?;
			}

			result[i] = u8::from_str_radix(byte, 16)?;
		}

		Ok(HwAddr {
			value: result
		})
	}
}

impl From<u32> for HwAddr {
	fn from(value: u32) -> HwAddr {
		let mut value  = value;
		let mut result = [0u8; 6];

		for i in 0 .. 6 {
			result[i]   = (value & 0xff) as u8;
			value     >>= 8;
		}

		HwAddr {
			value: result
		}
	}
}

impl From<[u8; 6]> for HwAddr {
	fn from(value: [u8; 6]) -> HwAddr {
		HwAddr {
			value: value
		}
	}
}

impl<'a> From<&'a [u8]> for HwAddr {
	fn from(value: &'a [u8]) -> HwAddr {
		HwAddr {
			value: [value[0], value[1], value[2], value[3], value[4], value[5]]
		}
	}
}

impl fmt::Display for HwAddr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
			self.value[0], self.value[1], self.value[2],
			self.value[3], self.value[4], self.value[5])
	}
}

#[cfg(test)]
mod test {
	use super::HwAddr;

	#[test]
	fn display() {
		assert_eq!("00:41:D0:24:00:0B",
			"00:41:D0:24:00:0B".parse::<HwAddr>().unwrap().to_string());
	}
}
