//! Diamond-Square algorithm implementation.
//! radynje@gmail.com

use std::cmp;

const MAX_HEIGHT: f64 = 255.0;

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

fn normalize(num: f64, max: f64) -> f64 {
	num / max
}

/// Set new height to the point.
fn set_height(
	map: &mut Vec<Vec<f64>>,
	x: usize,
	y: usize,
	chunk: usize,
	average: f64,
	_: f64
) {
	let random = rand::random::<f64>() - 0.5;

	map[x][y] = average + random * chunk as f64;
}

/// Get average height of square corners and set
/// height of it's middle-point.
fn square(
	map: &mut Vec<Vec<f64>>,
	map_size: usize,
	x: usize,
	y: usize,
	chunk: usize,
	half_chunk: usize,
	noise: f64
) {
	if map[x][y] != 0.0 {
		return;
	}

	let mut num: usize = 0;
	let mut sum: f64 = 0.0;

	if 0 <= x as i32 - half_chunk as i32 {
		if 0 <= y as i32 - half_chunk as i32 {
			sum += map[x - half_chunk][y - half_chunk];
			num += 1;
		}

		if y + half_chunk <= map_size {
			sum += map[x - half_chunk][y + half_chunk];
			num += 1;
		}
	}

	if x + half_chunk <= map_size {
		if 0 <= y as i32 - half_chunk as i32 {
			sum += map[x + half_chunk][y - half_chunk];
			num += 1;
		}

		if y + half_chunk <= map_size {
			sum += map[x + half_chunk][y + half_chunk];
			num += 1;
		}
	}

	let average = sum / num as f64;

	set_height(map, x, y, chunk, average, noise);
}

/// Get average height of diamon corners and set
/// height of it's middle-point.
fn diamond(
	map: &mut Vec<Vec<f64>>,
	map_size: usize,
	x: usize,
	y: usize,
	chunk: usize,
	half_chunk: usize,
	noise: f64
) {
	if map[x][y] != 0.0 {
		return;
	}

	let mut num: usize = 0;
	let mut sum: f64 = 0.0;

	if 0 <= x as i32 - half_chunk as i32 {
		sum += map[x - half_chunk][y];
		num += 1;
	};

	if x as i32 + half_chunk as i32 <= map_size as i32 {
		sum += map[x + half_chunk][y];
		num += 1;
	};

	if 0 <= y as i32 - half_chunk as i32 {
		sum += map[x][y - half_chunk];
		num += 1;
	};


	if y as i32 + half_chunk as i32 <= map_size as i32 {
		sum += map[x][y + half_chunk];
		num += 1;
	};

	let average = sum / num as f64;

	set_height(map, x, y, chunk, average, noise);
}

fn diamond_square(map_size: usize) -> Vec<Vec<f64>> {
	let max_index = map_size - 1;
	let half_height = MAX_HEIGHT / 2.0;
	let mut map: Vec<Vec<f64>> = vec![vec![0.0; map_size]; map_size];
	let mut chunk: usize = max_index >> 1;
	let mut half_chunk: usize = chunk >> 1;
	let mut first = false;

	map[0][0] = half_height;
	map[max_index][0] = half_height;
	map[0][max_index] = half_height;
	map[max_index][max_index] = half_height;

	while 1 <= chunk {
		for x in (half_chunk..map_size).step_by(chunk) {
			for y in (half_chunk..map_size).step_by(chunk) {
				square(&mut map, map_size, x ,y, chunk, half_chunk, 20.0);

				if !first {
					// println!("x: {}, y: {}, h: {}", x, y, map[x][y]);

					first = true;
				}
			}
		}

		// for x in (0..max_index).step_by(chunk) {
		// 	for y in (half_chunk..max_index).step_by(chunk) {
		// 		diamond(&mut map, map_size, x, y, chunk, half_chunk, 20.0);
		// 	}
		// }

		// for x in (chunk..max_index).step_by(chunk) {
		// 	for y in (0..max_index).step_by(chunk) {
		// 		diamond(&mut map, map_size, x, y, chunk, half_chunk, 20.0);
		// 	}
		// }

		chunk >>= 1;
		half_chunk >>= 1;
	}

	map
}

pub fn generate(
	width: usize,
	height: usize
) -> Vec<Vec<f64>> {
	clip(diamond_square(get_suitable_size(width, height)), width, height)
}
