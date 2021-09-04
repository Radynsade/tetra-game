use rand::prelude::*;
use rand_pcg::{Pcg64, Lcg128Xsl64};

struct Generator {
	width: usize,
	height: usize,
	seed: u64,
	rng: Lcg128Xsl64,
}

impl Generator {
	pub fn new(
		width: usize,
		height: usize,
		seed: u64
	) -> Generator {
		let rng: Lcg128Xsl64 = Pcg64::seed_from_u64(seed);

		Generator{
			width,
			height,
			seed,
			rng
		}
	}

	pub fn seed(&self) -> u64 {
		self.seed
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn generate(&self) {
		self.rng.
	}

	fn diamond_square(map_size: usize) -> Vec<Vec<f64>> {

	}
}
