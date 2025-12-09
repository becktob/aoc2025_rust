use std::ops::Mul;

// By the fundamental theorem of arithmetic, rational numbers in lowest
// terms are unique. So, by keeping `Rational`s in reduced form, we can
// derive `Eq` and `PartialEq`.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Zero is an invalid denominator!");
        }

        // Reduce to lowest terms by dividing by the greatest common
        // divisor.
        let gcd = gcd(numerator, denominator).abs();
        Self {
            // Ensure positive denominator by expanding with denominator.signum()
            numerator: numerator / gcd * denominator.signum(),
            denominator: denominator / gcd * denominator.signum(),
        }
    }
}

impl Mul for Rational {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[test]
fn test_rational_mul() {
    assert_eq!(Rational::new(1, 2), Rational::new(2, 4));
    assert_eq!(
        Rational::new(2, 3) * Rational::new(3, 4),
        Rational::new(1, 2)
    );
}

#[test]
fn test_positive_denom() {
    assert_eq!(Rational::new(-1, 1), Rational::new(1, -1));
}
