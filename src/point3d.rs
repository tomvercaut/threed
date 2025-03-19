use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl From<(f64, f64, f64)> for Point3D {
    fn from(p: (f64, f64, f64)) -> Self {
        Self::new(p.0, p.1, p.2)
    }
}

impl Add<f64> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: f64) -> Self::Output {
        Point3D::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f64> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: f64) -> Self::Output {
        Point3D::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Point3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, rhs: f64) -> Self::Output {
        Point3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl AddAssign<f64> for Point3D {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl SubAssign<f64> for Point3D {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl MulAssign<f64> for Point3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Point3D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Point3D> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<Point3D> for Point3D {
    type Output = Point3D;

    fn div(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl AddAssign<Point3D> for Point3D {
    fn add_assign(&mut self, rhs: Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<Point3D> for Point3D {
    fn sub_assign(&mut self, rhs: Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<Point3D> for Point3D {
    fn mul_assign(&mut self, rhs: Point3D) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign<Point3D> for Point3D {
    fn div_assign(&mut self, rhs: Point3D) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_scalar() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p + 2.0, Point3D::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_add_point() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        assert_eq!(p1 + p2, Point3D::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn test_sub_scalar() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p - 1.0, Point3D::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn test_sub_point() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        assert_eq!(p1 - p2, Point3D::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_mul_scalar() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p * 2.0, Point3D::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_point() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        assert_eq!(p1 * p2, Point3D::new(2.0, 6.0, 12.0));
    }

    #[test]
    fn test_div_scalar() {
        let p = Point3D::new(2.0, 4.0, 6.0);
        assert_eq!(p / 2.0, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_div_point() {
        let p1 = Point3D::new(2.0, 6.0, 12.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        assert_eq!(p1 / p2, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add_assign_scalar() {
        let mut p = Point3D::new(1.0, 2.0, 3.0);
        p += 2.0;
        assert_eq!(p, Point3D::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_add_assign_point() {
        let mut p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        p1 += p2;
        assert_eq!(p1, Point3D::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn test_sub_assign_scalar() {
        let mut p = Point3D::new(3.0, 4.0, 5.0);
        p -= 2.0;
        assert_eq!(p, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_sub_assign_point() {
        let mut p1 = Point3D::new(3.0, 5.0, 7.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        p1 -= p2;
        assert_eq!(p1, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut p = Point3D::new(1.0, 2.0, 3.0);
        p *= 2.0;
        assert_eq!(p, Point3D::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_assign_point() {
        let mut p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        p1 *= p2;
        assert_eq!(p1, Point3D::new(2.0, 6.0, 12.0));
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut p = Point3D::new(2.0, 4.0, 6.0);
        p /= 2.0;
        assert_eq!(p, Point3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_div_assign_point() {
        let mut p1 = Point3D::new(2.0, 6.0, 12.0);
        let p2 = Point3D::new(2.0, 3.0, 4.0);
        p1 /= p2;
        assert_eq!(p1, Point3D::new(1.0, 2.0, 3.0));
    }
}
