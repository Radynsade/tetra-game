use image;
use colored::*;

mod diamond;
// mod generator;
mod render;

pub struct HeightMap {
	matrix: Vec<Vec<f64>>,
	width: usize,
	height: usize
}

impl HeightMap {
	pub fn as_image(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
		let mut image = image::RgbImage::new(
			self.width as u32,
			self.height as u32
		);

		for (x, y, pixel) in image.enumerate_pixels_mut() {
			let (red, green, blue) = render::smooth(self.matrix[x as usize][y as usize]);

			*pixel = image::Rgb([red, green, blue]);
		}

		image
	}

	pub fn print(&self) {
		for x in 0..self.width {
			for y in 0..self.height {
				let (red, green, blue) = render::smooth(self.matrix[x as usize][y as usize]);

				print!("{}", "██".truecolor(red, green, blue));
			}

			println!("");
		}
	}

	pub fn scale(&self, to: f64) -> Vec<Vec<u64>> {
		let mut scaled_map: Vec<Vec<u64>> = vec![vec![0; self.height]; self.width];

		for x in 0..self.width {
			for y in 0..self.height {
				scaled_map[x][y] = (self.matrix[x][y] * to) as u64;
			}
		}

		return scaled_map;
	}
}

/// Generation algorithms.
pub enum Algorithm {
	DiamondSquare,
	Perlin
}

/// Generate height-map using selected algorithm.
pub fn generate(
	width: usize,
	height: usize,
	algorithm: Algorithm
) -> HeightMap {
	let matrix = match algorithm {
		Algorithm::DiamondSquare => diamond::generate(
			width,
			height
		),
		Algorithm::Perlin => vec![vec![0.0; 10]; 10]
	};

	HeightMap {
		matrix,
		width,
		height
	}
}
