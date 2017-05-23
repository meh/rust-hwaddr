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
	pub prefix:  [u8; 3],
	pub name:    &'static str,
	pub address: Address,
}

/// The address of the `Producer`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Address {
	pub street:  &'static str,
	pub city:    &'static str,
	pub country: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/database.rs"));
