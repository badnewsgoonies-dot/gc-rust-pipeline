#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self { coefficients }
    }

    pub fn degree(&self) -> usize {
        if self.coefficients.is_empty() {
            0
        } else {
            self.coefficients.len() - 1
        }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;
        for &c in self.coefficients.iter().rev() {
            result = result * x + c;
        }
        result
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = self.coefficients.len().max(other.coefficients.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.coefficients.get(i).copied().unwrap_or(0.0);
            let b = other.coefficients.get(i).copied().unwrap_or(0.0);
            result.push(a + b);
        }

        Polynomial::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn evaluate_constant() {
        let p = Polynomial::new(vec![5.0]);
        assert_eq!(p.evaluate(10.0), 5.0);
    }

    #[test]
    fn evaluate_linear() {
        let p = Polynomial::new(vec![3.0, 2.0]);
        assert_eq!(p.evaluate(4.0), 11.0);
    }

    #[test]
    fn evaluate_quadratic() {
        let p = Polynomial::new(vec![1.0, 0.0, 1.0]);
        assert_eq!(p.evaluate(3.0), 10.0);
    }

    #[test]
    fn add_unequal_lengths() {
        let a = Polynomial::new(vec![1.0, 2.0]);
        let b = Polynomial::new(vec![3.0, 4.0, 5.0]);
        assert_eq!(a.add(&b), Polynomial::new(vec![4.0, 6.0, 5.0]));
    }
}
