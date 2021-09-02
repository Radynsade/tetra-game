use image;
use colored::*;

mod diamond;
mod perlin;

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

		let mut height: u8;

		for (x, y, pixel) in image.enumerate_pixels_mut() {
			height = (self.matrix[x as usize][y as usize] * 255.0) as u8;

			*pixel = image::Rgb([height, height, height]);
		}

		image
	}

	pub fn print(&self) {
		let mut height: u8;

		for x in 0..self.width {
			for y in 0..self.height {
				height = (self.matrix[x][y] * 255.0) as u8;

				print!("{}", "██".truecolor(height, height, height));
			}

			println!("");
		}
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
		Algorithm::Perlin => perlin::generate(
			width,
			height
		)
	};

	HeightMap {
		matrix,
		width,
		height
	}
}
