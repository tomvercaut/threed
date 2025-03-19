use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
