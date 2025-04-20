
pub trait Abs {
    /// Returns the absolute value of a number or vector.
    ///
    /// This method calculates the absolute value of the implementing type.
    /// For vectors, it typically returns a new vector with the same direction
    /// but with a positive magnitude (length).
    ///
    /// # Returns
    ///
    /// Returns a new instance of the implementing type representing
    /// the absolute value of the original.
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