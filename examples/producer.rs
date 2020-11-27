extern crate hwaddr;

use std::env;
use hwaddr::HwAddr;

fn main() {
	let addr = env::args().nth(1).expect("no MAC address given")
		.parse::<HwAddr>().expect("not a MAC address");

	println!("{:#?}", addr);

	#[cfg(features = "database")]
	println!("{:#?}", addr.producer());
}
