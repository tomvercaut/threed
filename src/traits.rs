
pub trait Abs {
    fn abs(&self) -> Self;
}

pub trait Dot {
    /// Calculates the dot product (scalar product) between two vectors.
    ///
    /// The dot product is a mathematical operation that takes two vectors of equal length
    /// and returns a single scalar value. For vectors a and b, it is calculated as:
    /// a·b = a₁b₁ + a₂b₂ + ... + aₙbₙ
    ///
    /// # Arguments
    ///
    /// * `other` - Another vector of the same type to compute the dot product with
    ///
    /// # Returns
    ///
    /// Returns the scalar dot product as an f64 value
    fn dot(&self, other: &Self) -> f64;
}