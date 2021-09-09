//! Diamond-Square algorithm implementation.
//! radynje@gmail.com

use std::cmp;

/// Cut map to specified size.
fn clip(
	map: Vec<Vec<f64>>,
	width: usize,
	height: usize
) -> Vec<Vec<f64>> {
	let mut clipped_map: Vec<Vec<f64>> = vec![
		vec![
			std::f64::NAN;
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
	average: f64,
	scale: f64
) {
	let random = rand::random::<f64>() - 0.5;
	let height = average + random * scale as f64;

	map[x][y] = if height <= 1.0 && height >= 0.0 {
		height
	} else {
		if height > 1.0 { 1.0 } else { 0.0 }
	}
}

/// Get average height of square corners and set
/// height of it's middle-point.
fn square(
	map: &mut Vec<Vec<f64>>,
	max_index: usize,
	x: usize,
	y: usize,
	_: usize,
	half_chunk: usize,
	scale: f64
) {
	let mut num: usize = 0;
	let mut sum: f64 = 0.0;

	if 0 <= x as i32 - half_chunk as i32 {
		if 0 <= y as i32 - half_chunk as i32 {
			sum += map[x - half_chunk][y - half_chunk];
			num += 1;
		}

		if y + half_chunk <= max_index {
			sum += map[x - half_chunk][y + half_chunk];
			num += 1;
		}
	}

	if x + half_chunk <= max_index {
		if 0 <= y as i32 - half_chunk as i32 {
			sum += map[x + half_chunk][y - half_chunk];
			num += 1;
		}

		if y + half_chunk <= max_index {
			sum += map[x + half_chunk][y + half_chunk];
			num += 1;
		}
	}

	let average = sum / num as f64;

	set_height(map, x, y, average, scale);
}

/// Get average height of diamon corners and set
/// height of it's middle-point.
fn diamond(
	map: &mut Vec<Vec<f64>>,
	max_index: usize,
	x: usize,
	y: usize,
	half_chunk: usize,
	scale: f64
) {
	let mut num: usize = 0;
	let mut sum: f64 = 0.0;

	if 0 <= x as i32 - half_chunk as i32 {
		sum += map[x - half_chunk][y];
		num += 1;
	};

	if x as i32 + half_chunk as i32 <= max_index as i32 {
		sum += map[x + half_chunk][y];
		num += 1;
	};

	if 0 <= y as i32 - half_chunk as i32 {
		sum += map[x][y - half_chunk];
		num += 1;
	};

	if y as i32 + half_chunk as i32 <= max_index as i32 {
		sum += map[x][y + half_chunk];
		num += 1;
	};

	let average = sum / num as f64;

	set_height(map, x, y, average, scale);
}

fn diamond_square(map_size: usize) -> Vec<Vec<f64>> {
	let max_index = map_size - 1;
	let center = max_index >> 1;
	let mut map: Vec<Vec<f64>> = vec![vec![std::f64::NAN; map_size]; map_size];
	let mut chunk: usize = max_index >> 1;
	let mut half_chunk: usize = chunk >> 1;
	let mut scale: f64 = 1.0;

	map[0][0] = rand::random::<f64>();
	map[max_index][0] = rand::random::<f64>();
	map[0][max_index] = rand::random::<f64>();
	map[max_index][max_index] = rand::random::<f64>();

	square(&mut map, max_index, center, center, max_index, chunk, scale);
	diamond(&mut map, max_index, 0, center, chunk, scale);
	diamond(&mut map, max_index, max_index, center, chunk, scale);
	diamond(&mut map, max_index, center, 0, chunk, scale);
	diamond(&mut map, max_index, center, max_index, chunk, scale);

	while 1 <= chunk {
		for x in (half_chunk..map_size).step_by(chunk) {
			for y in (half_chunk..map_size).step_by(chunk) {
				if (map[x][y]).is_nan() {
					square(
						&mut map,
						max_index,
						x,
						y,
						chunk,
						half_chunk,
						scale
					);

					diamond(
						&mut map,
						max_index,
						x + half_chunk,
						y,
						half_chunk,
						scale
					);

					diamond(
						&mut map,
						max_index,
						x - half_chunk,
						y,
						half_chunk,
						scale
					);

					diamond(
						&mut map,
						max_index,
						x,
						y + half_chunk,
						half_chunk,
						scale
					);

					diamond(
						&mut map,
						max_index,
						x,
						y - half_chunk,
						half_chunk,
						scale
					);
				}

			}
		}

		chunk >>= 1;
		half_chunk >>= 1;
		scale /= 2.0;
	}

	map
}

pub fn generate(
	width: usize,
	height: usize
) -> Vec<Vec<f64>> {
	clip(diamond_square(get_suitable_size(width, height)), width, height)
}
