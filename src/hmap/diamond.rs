//! Diamond-Square algorithm implementation.
//! radynje@gmail.com

use std::cmp;

/// Faster average with fixed amount of elements.
fn count_average(
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

/// Set new height to the point.
fn set_height(
	map: &mut Vec<Vec<f64>>,
	x: usize,
	y: usize,
	chunk: usize,
	average: f64,
	noise: f64
) {
	let random = rand::random::<f64>() - 0.5;

	map[x][y] = average + random * chunk as f64 * noise;
}

/// Get average height of square corners and set
/// height of it's middle-point.
fn square(
	map: &mut Vec<Vec<f64>>,
	x: usize,
	y: usize,
	chunk: usize,
	half_chunk: usize,
	noise: f64
) {
	let average = count_average(
		map[x - half_chunk][y - half_chunk], // left-top
		map[x - half_chunk][y + half_chunk], // left-bottom
		map[x + half_chunk][y - half_chunk], // right-top
		map[x + half_chunk][y + half_chunk] // right-bottom
	);

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
	let left = if x as i32 - half_chunk as i32 <= 0 { 0.0 } else { map[x - half_chunk][y] };
	let right = if x as i32 + half_chunk as i32 > map_size as i32 { 0.0 } else { map[x + half_chunk][y] };
	let top = if y as i32 - half_chunk as i32 <= 0 { 0.0 } else { map[x][y - half_chunk] };
	let bottom = if y as i32 + half_chunk as i32 > map_size as i32 { 0.0 } else { map[x][y + half_chunk] };

	let average = count_average(
		left,
		right,
		top,
		bottom
	);

	set_height(map, x, y, chunk, average, noise);
}

fn diamond_square(map_size: usize) -> Vec<Vec<f64>> {
	let max_index = map_size - 1;
	let mut map: Vec<Vec<f64>> = vec![vec![0.0; map_size]; map_size];
	let mut chunk: usize = max_index >> 1;
	let mut half_chunk: usize = chunk >> 1;

	while 1 <= chunk {
		println!("chunk: {}", chunk);

		for x in (half_chunk..map_size).step_by(chunk) {
			for y in (half_chunk..map_size).step_by(chunk) {
				square(&mut map, x ,y, chunk, half_chunk, 1.0);
			}
		}

		for x in (0..max_index).step_by(chunk) {
			for y in (half_chunk..max_index).step_by(chunk) {
				diamond(&mut map, map_size, x, y, chunk, half_chunk, 10.0);
			}
		}

		for x in (chunk..max_index).step_by(chunk) {
			for y in (0..max_index).step_by(chunk) {
				diamond(&mut map, map_size, x, y, chunk, half_chunk, 10.0);
			}
		}

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
