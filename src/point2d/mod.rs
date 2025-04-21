use crate::eps::is_zero_f64;
use crate::traits::{Abs, Length, Norm};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn x(&self) -> &f64 {
        &self.x
    }

    #[inline]
    pub fn y(&self) -> &f64 {
        &self.y
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }
}

impl AsRef<Point2D> for Point2D {
    fn as_ref(&self) -> &Point2D {
        self
    }
}

impl Abs for Point2D {
    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Neg for Point2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Neg for &Point2D {
    type Output = Point2D;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y)
    }
}

impl From<(f64, f64)> for Point2D {
    fn from(p: (f64, f64)) -> Self {
        Self::new(p.0, p.1)
    }
}

impl Add for Point2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<f64> for Point2D {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<f64> for Point2D {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub for Point2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<f64> for Point2D {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl SubAssign<f64> for Point2D {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul for Point2D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Point2D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f64> for Point2D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Point2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Point2D {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign for Point2D {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<f64> for Point2D {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f64> for Point2D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Length for Point2D {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Norm for Point2D {
    fn norm(&self) -> Point2D {
        let l = self.length();
        if is_zero_f64(l) {
            Point2D::new(0.0, 0.0)
        } else {
            Point2D::new(self.x / l, self.y / l)
        }
    }

    fn norm_mut(&mut self) {
        let l = self.length();
        if is_zero_f64(l) {
            self.x = 0.0;
            self.y = 0.0;
        } else {
            self.x /= l;
            self.y /= l;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_abs() {
        let p = Point2D::new(-1.0, -2.0);
        let result = p.abs();
        assert_eq!(result, Point2D::new(1.0, 2.0));
    }

    #[test]
    fn test_point_getters() {
        let p = Point2D::new(1.0, 2.0);
        assert_eq!(*p.x(), 1.0);
        assert_eq!(*p.y(), 2.0);
    }

    #[test]
    fn test_point_mut_getters() {
        let mut p = Point2D::new(1.0, 2.0);
        *p.x_mut() = 3.0;
        *p.y_mut() = 4.0;
        assert_eq!(*p.x(), 3.0);
        assert_eq!(*p.y(), 4.0);
    }

    #[test]
    fn test_point_neg() {
        let p = Point2D::new(1.0, 2.0);
        let result = -p;
        assert_eq!(result, Point2D::new(-1.0, -2.0));
    }

    #[test]
    fn test_point_neg_ref() {
        let p = Point2D::new(1.0, 2.0);
        let result = -&p;
        assert_eq!(result, Point2D::new(-1.0, -2.0));
    }

    #[test]
    fn test_point_add() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(3.0, 4.0);
        let result = p1 + p2;
        assert_eq!(result, Point2D::new(4.0, 6.0));
    }

    #[test]
    fn test_point_add_scalar() {
        let p = Point2D::new(1.0, 2.0);
        let result = p + 2.0;
        assert_eq!(result, Point2D::new(3.0, 4.0));
    }

    #[test]
    fn test_point_add_assign() {
        let mut p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(3.0, 4.0);
        p1 += p2;
        assert_eq!(p1, Point2D::new(4.0, 6.0));
    }

    #[test]
    fn test_point_add_assign_scalar() {
        let mut p = Point2D::new(1.0, 2.0);
        p += 2.0;
        assert_eq!(p, Point2D::new(3.0, 4.0));
    }

    #[test]
    fn test_point_sub() {
        let p1 = Point2D::new(4.0, 6.0);
        let p2 = Point2D::new(1.0, 2.0);
        let result = p1 - p2;
        assert_eq!(result, Point2D::new(3.0, 4.0));
    }

    #[test]
    fn test_point_sub_scalar() {
        let p = Point2D::new(3.0, 4.0);
        let result = p - 2.0;
        assert_eq!(result, Point2D::new(1.0, 2.0));
    }

    #[test]
    fn test_point_sub_assign() {
        let mut p1 = Point2D::new(4.0, 6.0);
        let p2 = Point2D::new(1.0, 2.0);
        p1 -= p2;
        assert_eq!(p1, Point2D::new(3.0, 4.0));
    }

    #[test]
    fn test_point_sub_assign_scalar() {
        let mut p = Point2D::new(3.0, 4.0);
        p -= 2.0;
        assert_eq!(p, Point2D::new(1.0, 2.0));
    }

    #[test]
    fn test_point_mul() {
        let p1 = Point2D::new(2.0, 3.0);
        let p2 = Point2D::new(3.0, 4.0);
        let result = p1 * p2;
        assert_eq!(result, Point2D::new(6.0, 12.0));
    }

    #[test]
    fn test_point_mul_scalar() {
        let p = Point2D::new(2.0, 3.0);
        let result = p * 2.0;
        assert_eq!(result, Point2D::new(4.0, 6.0));
    }

    #[test]
    fn test_point_mul_assign() {
        let mut p1 = Point2D::new(2.0, 3.0);
        let p2 = Point2D::new(3.0, 4.0);
        p1 *= p2;
        assert_eq!(p1, Point2D::new(6.0, 12.0));
    }

    #[test]
    fn test_point_mul_assign_scalar() {
        let mut p = Point2D::new(2.0, 3.0);
        p *= 2.0;
        assert_eq!(p, Point2D::new(4.0, 6.0));
    }

    #[test]
    fn test_point_div() {
        let p1 = Point2D::new(6.0, 12.0);
        let p2 = Point2D::new(2.0, 3.0);
        let result = p1 / p2;
        assert_eq!(result, Point2D::new(3.0, 4.0));
    }

    #[test]
    fn test_point_div_scalar() {
        let p = Point2D::new(4.0, 6.0);
        let result = p / 2.0;
        assert_eq!(result, Point2D::new(2.0, 3.0));
    }

    #[test]
    fn test_point_div_assign() {
        let mut p1 = Point2D::new(6.0, 12.0);
        let p2 = Point2D::new(2.0, 3.0);
        p1 /= p2;
        assert_eq!(p1, Point2D::new(3.0, 4.0));
    }

    #[test]
    fn test_point_div_assign_scalar() {
        let mut p = Point2D::new(4.0, 6.0);
        p /= 2.0;
        assert_eq!(p, Point2D::new(2.0, 3.0));
    }

    #[test]
    fn test_point_length() {
        let p = Point2D::new(3.0, 4.0);
        assert_eq!(p.length(), 5.0);
    }

    #[test]
    fn test_point_norm() {
        let p = Point2D::new(3.0, 4.0);
        let result = p.norm();
        assert_eq!(result, Point2D::new(0.6, 0.8));
    }

    #[test]
    fn test_point_norm_mut() {
        let mut p = Point2D::new(3.0, 4.0);
        p.norm_mut();
        assert_eq!(p, Point2D::new(0.6, 0.8));
    }

    #[test]
    fn test_point_norm_zero() {
        let p = Point2D::new(0.0, 0.0);
        let result = p.norm();
        assert_eq!(result, Point2D::new(0.0, 0.0));
    }

    #[test]
    fn test_point_norm_mut_zero() {
        let mut p = Point2D::new(0.0, 0.0);
        p.norm_mut();
        assert_eq!(p, Point2D::new(0.0, 0.0));
    }
}

pub mod ops {
    use super::*;

    /// Calculates the dot product of two 2D vectors.
    ///
    /// The dot product is the sum of the products of the corresponding components
    /// of the two vectors.
    ///
    /// # Arguments
    /// * `a` - The first vector
    /// * `b` - The second vector
    ///
    /// # Returns
    /// The dot product of vectors `a` and `b`
    ///
    /// # Example
    /// ```
    /// use threed::point2d::{Point2D, ops::dot};
    /// let v1 = Point2D::new(1.0, 2.0);
    /// let v2 = Point2D::new(3.0, 4.0);
    /// assert_eq!(dot(v1, v2), 11.0); // 1.0 * 3.0 + 2.0 * 4.0
    /// ```
    pub fn dot<P: AsRef<Point2D>>(a: P, b: P) -> f64 {
        let a = a.as_ref();
        let b = b.as_ref();
        a.x * b.x + a.y * b.y
    }

    /// Calculates the cross product (z-component) of two 2D vectors.
    ///
    /// The cross product of two 2D vectors results in a scalar value representing
    /// the signed area of the parallelogram formed by these vectors.
    /// A positive result indicates a counter-clockwise orientation from vector a to b,
    /// while a negative result indicates a clockwise orientation.
    ///
    /// # Arguments
    /// * `a` - The first vector
    /// * `b` - The second vector
    ///
    /// # Returns
    /// The z-component of the cross product (a.x * b.y - a.y * b.x)
    ///
    /// # Example
    /// ```
    /// use threed::point2d::{Point2D, ops::cross};
    /// let v1 = Point2D::new(1.0, 2.0);
    /// let v2 = Point2D::new(3.0, 4.0);
    /// assert_eq!(cross(v1, v2), -2.0);
    /// assert_eq!(cross(v2, v1), 2.0);
    /// ```
    ///
    pub fn cross<P: AsRef<Point2D>>(a: P, b: P) -> f64 {
        let a = a.as_ref();
        let b = b.as_ref();
        a.x * b.y - a.y * b.x
    }

    /// Calculates the Euclidean distance between two points in 2D space.
    ///
    /// This function computes the straight-line distance between two points
    /// using the Pythagorean theorem: sqrt((x₂-x₁)² + (y₂-y₁)²)
    ///
    /// # Arguments
    /// * `a` - The first point that i2yyjmplements AsRef<Point3D>
    /// * `b` - The second point that implements AsRef<Point3D>
    ///
    /// # Returns
    /// The distance between points `a` and `b`
    ///
    /// # Example
    /// ```
    /// use threed::point2d::{Point2D, ops::distance};
    /// let p1 = Point2D::new(1.0, 2.0);
    /// let p2 = Point2D::new(4.0, 6.0);
    /// assert_eq!(distance(p1, p2), 5.0); // Forms a 3-4-5 triangle
    /// ```
    pub fn distance<P: AsRef<Point2D>>(a: P, b: P) -> f64 {
        let a = a.as_ref();
        let b = b.as_ref();
        ((b.x() - a.x()).powi(2) + (b.y() - a.y()).powi(2)).sqrt()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_point_distance() {
            let p1 = Point2D::new(1.0, 1.0);
            let p2 = Point2D::new(4.0, 5.0);
            assert_eq!(distance(p1, p2), 5.0);
        }

        #[test]
        fn test_point_dot() {
            let p1 = Point2D::new(1.0, 2.0);
            let p2 = Point2D::new(3.0, 4.0);
            let result = dot(p1, p2);
            assert_eq!(result, 11.0); // 1.0 * 3.0 + 2.0 * 4.0 = 11.0
        }

        #[test]
        fn test_point_cross() {
            let p1 = Point2D::new(1.0, 2.0);
            let p2 = Point2D::new(3.0, 4.0);
            let result = cross(p1, p2);
            assert_eq!(result, -2.0); // 1.0 * 4.0 - 2.0 * 3.0 = -2.0
        }
    }
}
