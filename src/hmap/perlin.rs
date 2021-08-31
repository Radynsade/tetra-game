/// Linearly interpolate between num1 and num2,
/// weight must be in the range [0.0, 1.0].
fn lerp(
	num1: f64,
	num2: f64,
	weight: f64,
) -> f64 {
	(1.0 - weight) * num1 + weight * num2
}

pub fn generate(
	width: usize,
	height: usize
) -> Vec<Vec<f64>> {
	vec![vec![0.0; height]; width]
}
