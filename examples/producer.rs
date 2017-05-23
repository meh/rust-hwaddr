extern crate hwaddr;

use std::env;
use hwaddr::HwAddr;

fn main() {
	println!("{:#?}",
		env::args().nth(1).expect("no MAC address given")
			.parse::<HwAddr>().expect("not a MAC address")
			.producer().expect("no producer"));
}
