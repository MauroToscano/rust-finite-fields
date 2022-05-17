/// Finite Field<const ORDER: u64>s F_p implementation
///
use std::ops;
use core::cmp;

#[derive(Debug)]
pub struct Field<const ORDER: u64> {
    element: u64,
}

impl <const ORDER: u64> Field<ORDER> {
    // TODO: this grows too fast, use modular exponentiation instead
    // TODO 2: change exponent to not cast it to u32
    pub fn pow(base: Self, exponent: u64) -> Self {
        Self { element: u64::pow(base.element, exponent as u32) % ORDER }
    }

    pub fn new(element: u64) -> Self {
        Self { element: element }
    }
}

impl <const ORDER: u64> ops::Add for Field<ORDER> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { element: (self.element + rhs.element) % ORDER }
    }
}

impl <const ORDER: u64> ops::Sub for Field<ORDER> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { element: (self.element - rhs.element) % ORDER }
    }

}

impl <const ORDER: u64> ops::Mul for Field<ORDER> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self { element: (self.element * rhs.element) % ORDER }
    }
}


impl <const ORDER: u64> ops::Div for Field<ORDER> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.element % ORDER == 0 {
            panic!("Cannot divide by 0");
        }

        let inverse = Self::pow(rhs, ORDER - 2);
        self * inverse
    }
}

impl <const ORDER: u64> cmp::PartialEq for Field<ORDER> {
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

    const ORDER: u64 = 10000000007;
    #[test]
    fn addition() {
        let result = Field::<ORDER>::new(2) + Field::<ORDER>::new(2);
        assert_eq!(result, Field::<ORDER>::new(4));
    }

    #[test]
    fn subtraction() {
        let result = Field::<ORDER>::new(3) - Field::<ORDER>::new(2);
        assert_eq!(result, Field::<ORDER>::new(1));
    }

    #[test]
    fn multiplication() {
        let result = Field::<ORDER>::new(2) * Field::<ORDER>::new(3);
        assert_eq!(result, Field::<ORDER>::new(6));
    }

    #[test]
    fn division() {
        let result = Field::<ORDER>::new(10) / Field::<ORDER>::new(2);
        assert_eq!(result, Field::<ORDER>::new(5));
    }
}
