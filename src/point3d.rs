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
