mod hmap;

const MAP_WIDTH: usize = 400;
const MAP_HEIGHT: usize = 400;
const IMAGE_FILE: &str = "heightmap.png";

fn main() {
	println!("Diamond-square algorithm used.");

	let map: hmap::HeightMap = hmap::generate(
		MAP_WIDTH,
		MAP_HEIGHT,
		hmap::Algorithm::DiamondSquare
	);

	map.as_image().save(IMAGE_FILE).unwrap();
}
