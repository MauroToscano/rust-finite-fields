/// Finite Fields F_p implementation
///
use std::ops;
use core::cmp;

#[derive(Debug)]
pub struct Field {
    element: u64,
}

const PRIME : u64 = 1000000007;

impl Field {
    // TODO: this grows too fast, use modular exponentiation instead
    // TODO 2: change exponent to not cast it to u32
    pub fn pow(base: Self, exponent: u64) -> Self {
        Self { element: u64::pow(base.element, exponent as u32) % PRIME }
    }

    pub fn new(element: u64) -> Self {
        Self { element: element }
    }
}

impl ops::Add for Field {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { element: (self.element + rhs.element) % PRIME }
    }
}

impl ops::Sub for Field {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { element: (self.element - rhs.element) % PRIME }
    }

}

impl ops::Mul for Field {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self { element: (self.element * rhs.element) % PRIME }
    }
}


impl ops::Div for Field {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.element % PRIME == 0 {
            panic!("Cannot divide by 0");
        }

        let inverse = Self::pow(rhs, PRIME - 2);
        self * inverse
    }
}

impl cmp::PartialEq for Field {
    fn eq(&self, rhs: &Self) -> bool {
        self.element == rhs.element
    }

    fn ne(&self, rhs: &Self) -> bool {
        self.element != rhs.element
    }
}


#[cfg(test)]
mod tests {
    use crate::field::Field;

    #[test]
    fn addition() {
        let result = Field::new(2) + Field::new(2);
        assert_eq!(result, Field::new(4));
    }

    #[test]
    fn subtraction() {
        let result = Field::new(3) - Field::new(2);
        assert_eq!(result, Field::new(1));
    }

    #[test]
    fn multiplication() {
        let result = Field::new(2) * Field::new(3);
        assert_eq!(result, Field::new(6));
    }

    #[test]
    fn divition() {
        let result = Field::new(10) / Field::new(2);
        assert_eq!(result, Field::new(5));
    }
}
