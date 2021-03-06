const MAX_SMOOTH: f64 = 1400.0;
const MAX_COLOR: f64 = 255.0;

pub fn grayscale(height: f64) -> (u8, u8, u8) {
	let rgb = (height * 255.0) as u8;

	(rgb, rgb, rgb)
}

pub fn smooth(height: f64) -> (u8, u8, u8) {
	let mut parts: [u8; 6] = [0, 0, 0, 0, 0, 0];
	let h = height * MAX_SMOOTH;
	let division: f64;
	let remainder: u8;
	let wholes: usize;
	let red: u8;
	let green: u8;
	let blue: u8;

	division = h / MAX_COLOR;
	wholes = division.floor() as usize;

	if wholes == 6 {
		return (130, 0, 0);
	}

	for i in 0..wholes {
		parts[i] = 255;
	}

	remainder = (h % MAX_COLOR) as u8;
	parts[wholes] = remainder;
	blue = parts[0] - parts[2];
	green = parts[1] - parts[4];
	red = parts[3] - parts[5];

	(red, green, blue)
}
