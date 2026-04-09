#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexNum {
    pub re: f64,
    pub im: f64,
}

impl ComplexNum {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn add(&self, other: &ComplexNum) -> ComplexNum {
        ComplexNum {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn sub(&self, other: &ComplexNum) -> ComplexNum {
        ComplexNum {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    pub fn mul(&self, other: &ComplexNum) -> ComplexNum {
        ComplexNum {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn conjugate(&self) -> ComplexNum {
        ComplexNum {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

#[cfg(test)]
mod tests {
    use super::ComplexNum;

    #[test]
    fn add_simple() {
        let a = ComplexNum::new(1.0, 2.0);
        let b = ComplexNum::new(3.0, 4.0);
        assert_eq!(a.add(&b), ComplexNum::new(4.0, 6.0));
    }

    #[test]
    fn mul_distribution() {
        let a = ComplexNum::new(1.0, 2.0);
        let b = ComplexNum::new(3.0, 4.0);
        assert_eq!(a.mul(&b), ComplexNum::new(-5.0, 10.0));
    }

    #[test]
    fn conjugate_negates_im() {
        let z = ComplexNum::new(2.0, 5.0);
        assert_eq!(z.conjugate(), ComplexNum::new(2.0, -5.0));
    }
}
