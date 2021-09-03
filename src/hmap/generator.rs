struct Generator {
	width: usize,
	height: usize,
	seed: u64,
}

impl Generator {
	pub fn new(
		width: usize,
		height: usize,
		seed: u64
	) -> Generator {
		Generator{
			width,
			height,
			seed
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
}
