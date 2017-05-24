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

/// The producer for a specific MAC address range.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Producer {
	/// The MAC address prefix they were assigned.
	pub prefix: [u8; 3],

	/// The name of the company.
	pub name: &'static str,

	/// The address for the company.
	pub address: Address,
}

/// The address of the `Producer`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Address {
	/// The street.
	pub street: &'static str,

	/// The city.
	pub city: &'static str,

	/// The country.
	pub country: &'static str,
}

#[cfg(feature = "database")]
include!(concat!(env!("OUT_DIR"), "/database.rs"));
