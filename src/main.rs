mod hmap;

use text_io::read;
use std::io::{self, Write};

const IMAGE_FILE: &str = "heightmap.png";

fn main() {
	let mut map: hmap::HeightMap;
	let mut width: usize;
	let mut height: usize;
	let mut operation: u8;
	let mut repeat: char;

	loop {
		print!("Enter map width: ");
		io::stdout().flush().unwrap();
		width = read!();

		print!("Enter map height: ");
		io::stdout().flush().unwrap();
		height = read!();

		println!("Generating...");
		map = hmap::generate(
			width,
			height,
			hmap::Algorithm::DiamondSquare
		);
		println!("Completed.");

		print!("Possible operations:\n1) Print in console;\n2) Save as image.\n");
		print!("Enter operation number: ");
		io::stdout().flush().unwrap();
		operation = read!();

		match operation {
			1 => map.print(),
			2 => map.as_image().save(IMAGE_FILE).unwrap(),
			_ => std::process::exit(1)
		}

		print!("Repeat? (y/n): ");
		io::stdout().flush().unwrap();
		repeat = read!();

		if repeat == 'n' {
			std::process::exit(1);
		}
	}
}
