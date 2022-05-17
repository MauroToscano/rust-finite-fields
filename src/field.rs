/// Finite Field<const ORDER: u64>s F_p implementation
///
use std::ops;
use core::cmp;

/* Iterative Function to calculate (base^exponent)%modulo in O(log exponent) */
// Taken from https://www.geeksforgeeks.org/modular-exponentiation-power-in-modular-arithmetic/
pub fn mod_pow(mut base: u64, mut exponent: u64, modulo: u64) -> u64
{
    let mut res = 1;
 
    base = base % modulo; 
  
    if base == 0 { return 0; }
 
    while exponent > 0
    {
        if exponent & 1 != 0 { res = (res*base) % modulo; }
 
        exponent = exponent>>1;
        base = (base*base) % modulo;
    }
    res
}

#[derive(Debug, Copy, Clone)]
pub struct Field<const ORDER: u64> {
    element: u64,
}


impl <const ORDER: u64> Field<ORDER> {
    // TODO 2: change exponent to not cast it to u32
    pub fn pow(base: Self, mut exponent: i64) -> Self {
        while exponent < 0 {
            exponent += (ORDER - 1) as i64;
        }
        Field { element: mod_pow(base.element, exponent as u64, ORDER) }
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

        let inverse = Field::pow(rhs, -1);
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

    const ORDER: u64 = 1000000007;
    type TestField = Field::<ORDER>;
    #[test]
    fn addition() {
        let result = TestField::new(2) + Field::<ORDER>::new(2);
        assert_eq!(result, TestField::new(4));
    }

    #[test]
    fn subtraction() {
        let result = TestField::new(3) - Field::<ORDER>::new(2);
        assert_eq!(result, TestField::new(1));
    }

    #[test]
    fn multiplication() {
        let result = TestField::new(2) * Field::<ORDER>::new(3);
        assert_eq!(result, TestField::new(6));
    }

    #[test]
    fn division() {
        let result = TestField::new(10) / TestField::new(2);
        assert_eq!(result, TestField::new(5));
    }

    #[test]
    fn inverse_using_pow() {
        let inverse = TestField::pow(TestField::new(10), (ORDER - 2) as i64);
        let result = TestField::new(10) * inverse;
        assert_eq!(result, TestField::new(1));
    }
}
