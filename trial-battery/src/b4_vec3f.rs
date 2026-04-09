#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn add(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn scale(&self, s: f64) -> Vec3f {
        Vec3f {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn dot(&self, other: &Vec3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3f;

    #[test]
    fn add_correct() {
        let a = Vec3f::new(1.0, 2.0, 3.0);
        let b = Vec3f::new(4.0, 5.0, 6.0);
        assert_eq!(a.add(&b), Vec3f::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn dot_correct() {
        let a = Vec3f::new(1.0, 2.0, 3.0);
        let b = Vec3f::new(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn length_pythagorean() {
        let v = Vec3f::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }
}
