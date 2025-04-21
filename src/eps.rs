pub fn is_zero_f64(x: f64) -> bool {
    x.abs() < f64::EPSILON
}