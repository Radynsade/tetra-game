use image;
use colored::*;

mod diamond;
mod perlin;
mod generator;
mod render;

// 255x4 = 1020 - max combination of colours.
// 1020 - 100x3 = 720 - with min color brightness.
const MAX_HEIGHT: f64 = 1020.0;
const MAX_COLOR: f64 = 255.0;

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
