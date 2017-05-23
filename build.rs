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

extern crate reqwest;
extern crate regex;
extern crate phf_codegen;

use std::env;
use std::io::{Read, Write, BufWriter};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

use regex::Regex;

fn main() {
	let mut content = String::new();

	if let Ok(value) = env::var("HWADDR_DATABASE") {
		File::open(value).unwrap().read_to_string(&mut content).unwrap();
	}
	else {
		reqwest::get("http://standards.ieee.org/develop/regauth/oui/oui.txt").unwrap()
			.read_to_string(&mut content).unwrap();
	}

	let     path    = Path::new(&env::var("OUT_DIR").unwrap()).join("database.rs");
	let mut file    = BufWriter::new(File::create(&path).unwrap());
	let mut builder = phf_codegen::Map::new();

	write!(&mut file, "pub static DATABASE: ::phf::Map<[u8; 3], Producer> = ").unwrap();

	let mut lines   = content.lines();
	let mut entries = HashSet::new();
	let     start   = Regex::new(r#"^(?P<prefix>[[:xdigit:]]{6})\s+\(base 16\)\s*(?P<name>.+)$"#).unwrap();

	while let Some(line) = lines.next() {
		if let Some(captures) = start.captures(line) {
			let prefix = captures["prefix"].to_owned();
			let prefix = [
				u8::from_str_radix(&prefix[0 .. 2], 16).unwrap(),
				u8::from_str_radix(&prefix[2 .. 4], 16).unwrap(),
				u8::from_str_radix(&prefix[4 .. 6], 16).unwrap() ];

			let name    = captures["name"].to_owned();
			let street  = lines.next().unwrap().trim().to_owned();
			let city    = lines.next().unwrap().trim().to_owned();
			let country = lines.next().unwrap().trim().to_owned();

			if !entries.contains(&prefix) {
				entries.insert(prefix);
				builder.entry(prefix, &format!(r#"
					Producer {{
						prefix:  {:?},
						name:    {:?},
						address: Address {{
							street:  {:?},
							city:    {:?},
							country: {:?},
						}},
					}}
				"#, prefix, name, street, city, country));
			}
		}
	}

	builder.build(&mut file).unwrap();
	write!(&mut file, ";\n").unwrap();
}
