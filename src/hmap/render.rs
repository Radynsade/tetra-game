const MAX_SMOOTH: f64 = 1275.0;
const MAX_RGB: f64 = 1020.0;
const MAX_COLOR: f64 = 255.0;

pub fn grayscale(height: f64) -> (u8, u8, u8) {
	let rgb = (height * 255.0) as u8;

	(rgb, rgb, rgb)
}

pub fn rgb(height: f64) -> (u8, u8, u8) {
	let mut h = height * MAX_RGB;
	let red: u8;
	let green: u8;
	let blue: u8;

	blue = if h > MAX_COLOR {
		if h / MAX_COLOR >= 2.0 {
			0
		} else {
			255 - ((h % MAX_COLOR) as u8)
		}
	} else {
		h as u8
	};

	h -= MAX_COLOR;
	if h < 0.0 { h = 0.0 };

	green = if h > MAX_COLOR {
		if h / MAX_COLOR >= 2.0 {
			0
		} else {
			255 - ((h % MAX_COLOR) as u8)
		}
	} else {
		h as u8
	};

	h -= MAX_COLOR;
	if h < 0.0 { h = 0.0 };

	red = if h > MAX_COLOR {
		if h / MAX_COLOR >= 2.0 {
			0
		} else {
			255 - ((h % MAX_COLOR) as u8)
		}
	} else {
		h as u8
	};

	(red, green, blue)
}

pub fn smooth(height: f64) -> (u8, u8, u8) {
	let mut h = height * MAX_SMOOTH;
	let mut division: f64;
	let red: u8;
	let green: u8;
	let blue: u8;

	blue = if h > MAX_COLOR {
		division = h / MAX_COLOR;

		if division >= 3.0 {
			0
		} else {
			if division < 2.0 {
				255
			} else {
				255 - ((h % MAX_COLOR) as u8)
			}
		}
	} else {
		h as u8
	};

	h -= MAX_COLOR;
	if h < 0.0 { h = 0.0 };

	green = if h > MAX_COLOR {
		division = h / MAX_COLOR;

		if division >= 3.0 {
			0
		} else {
			if division < 2.0 {
				255
			} else {
				255 - ((h % MAX_COLOR) as u8)
			}
		}
	} else {
		h as u8
	};

	h -= MAX_COLOR;
	if h < 0.0 { h = 0.0 };

	red = h as u8;

	(red, green, blue)
}
