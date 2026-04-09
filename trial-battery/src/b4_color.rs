#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        ColorRgb::new(0, 0, 0)
    }

    pub fn white() -> Self {
        ColorRgb::new(255, 255, 255)
    }

    pub fn mix(&self, other: &ColorRgb, t: f64) -> ColorRgb {
        let t = t.clamp(0.0, 1.0);
        let r = (self.r as f64 * (1.0 - t) + other.r as f64 * t).round() as u8;
        let g = (self.g as f64 * (1.0 - t) + other.g as f64 * t).round() as u8;
        let b = (self.b as f64 * (1.0 - t) + other.b as f64 * t).round() as u8;
        ColorRgb::new(r, g, b)
    }

    pub fn brightness(&self) -> u32 {
        self.r as u32 + self.g as u32 + self.b as u32
    }
}

#[cfg(test)]
mod tests {
    use super::ColorRgb;

    #[test]
    fn black_brightness() {
        assert_eq!(ColorRgb::black().brightness(), 0);
    }

    #[test]
    fn white_brightness() {
        assert_eq!(ColorRgb::white().brightness(), 765);
    }

    #[test]
    fn mix_zero_returns_self() {
        let a = ColorRgb::new(100, 200, 50);
        let b = ColorRgb::new(0, 0, 0);
        assert_eq!(a.mix(&b, 0.0), a);
    }

    #[test]
    fn mix_one_returns_other() {
        let a = ColorRgb::new(100, 200, 50);
        let b = ColorRgb::new(0, 0, 0);
        assert_eq!(a.mix(&b, 1.0), b);
    }

    #[test]
    fn mix_half_averages() {
        let a = ColorRgb::new(100, 200, 50);
        let b = ColorRgb::new(200, 0, 150);
        assert_eq!(a.mix(&b, 0.5), ColorRgb::new(150, 100, 100));
    }
}
