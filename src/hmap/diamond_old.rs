use std::cmp;

/// Faster average with fixed amount of elements.
fn average(
	num1: f64,
	num2: f64,
	num3: f64,
	num4: f64
) -> f64 {
	(num1 + num2 + num3 + num4) / 4.0
}

/// Cut map to specified size.
fn clip(
	map: Vec<Vec<f64>>,
	width: usize,
	height: usize
) -> Vec<Vec<f64>> {
	let mut clipped_map: Vec<Vec<f64>> = vec![
		vec![
			0.0;
			height
		];
		width
	];

	for x in 0..width {
		for y in 0..height {
			clipped_map[x][y] = map[x][y];
		}
	}

	clipped_map
}

/// Get suitable map size for diamond-square algorithm (2^n + 1)
fn get_suitable_size(
	width: usize,
	height: usize
) -> usize {
	let biggest = cmp::max(width, height);
	let mut size = 2;

	loop {
		if biggest <= size {
			return size + 1;
		};
		size <<= 1;
	}
}

/// Get new height of the pixel.
fn set_height(
	map: &mut Vec<Vec<f64>>,
	x: usize,
	y: usize,
	chunk: usize,
	avg: f64,
	noise: f64
) {
	map[x][y] = avg + (rand::random::<f64>() - 0.5) * noise as f64;
}

/// Get average height of square corners and set
/// height of it's middle-point.
fn square(
	map: &mut Vec<Vec<f64>>,
	x: usize,
	y: usize,
	chunk: usize,
	noise: f64
) {
	let offset: usize = chunk >> 1;

	let avg = average(
		map[x - offset][y - offset], // left-top
		map[x - offset][y + offset], // left-bottom
		map[x + offset][y - offset], // right-top
		map[x + offset][y + offset] // right-bottom
	);

	set_height(map, x, y, chunk, avg, noise);
}

/// Get average height of diamond corners and set
/// set height of it's middle-point.
fn diamond(
	map: &mut Vec<Vec<f64>>,
	x: usize,
	y: usize,
	chunk: usize,
	noise: f64
) {
	let offset: usize = chunk >> 1;

	let avg = average(
		map[x - offset][y], // left
		map[x][y - offset], // top
		map[x + offset][y], // right
		map[x][y + offset] // bottom
	);

	set_height(map, x, y, chunk, avg, noise);
}

fn diamond_square(size: usize) -> Vec<Vec<f64>> {
	let max = size - 1;
	let mut map: Vec<Vec<f64>> = vec![vec![0.0; size]; size];
	let mut chunk: usize = size;
	let mut half_chunk: usize = chunk << 1;
	let mut noise: f64 = chunk as f64 / size as f64;

	map[0][0] = 0.5 + (rand::random::<f64>() - 0.5);
	map[0][1] = 0.5 + (rand::random::<f64>() - 0.5);
	map[1][0] = 0.5 + (rand::random::<f64>() - 0.5);
	map[0][0] = 0.5 + (rand::random::<f64>() - 0.5);

	while 1 <= chunk {
		for x in (half_chunk..max).step_by(chunk) {
			for y in (half_chunk..max).step_by(chunk) {
				square(&mut map, x, y, chunk, noise);
			}
		}

		chunk >>= 1;
		half >>= 1;
		noise /= 2.0;
	}

	map
}

pub fn generate(
	width: usize,
	height: usize
) -> Vec<Vec<f64>> {
	clip(diamond_square(get_suitable_size(width, height)), width, height)
}
