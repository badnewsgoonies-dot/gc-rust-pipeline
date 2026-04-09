#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("denominator cannot be zero");
        }

        let mut numerator = numerator;
        let mut denominator = denominator;

        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }

        let gcd = Self::gcd(numerator, denominator);
        Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a.abs()
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn numerator(&self) -> i64 {
        self.numerator
    }

    pub fn denominator(&self) -> i64 {
        self.denominator
    }

    pub fn add(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }

    pub fn multiply(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;

    #[test]
    fn normalize_simplifies() {
        let f = Fraction::new(4, 8);
        assert_eq!(f.numerator(), 1);
        assert_eq!(f.denominator(), 2);
    }

    #[test]
    fn negative_in_denom() {
        let f = Fraction::new(3, -6);
        assert_eq!(f.numerator(), -1);
        assert_eq!(f.denominator(), 2);
    }

    #[test]
    fn add_unlike() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 3);
        assert_eq!(a.add(&b), Fraction::new(5, 6));
    }

    #[test]
    fn multiply() {
        let a = Fraction::new(2, 3);
        let b = Fraction::new(3, 4);
        assert_eq!(a.multiply(&b), Fraction::new(1, 2));
    }
}
