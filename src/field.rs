/// Finite Fields F_p implementation
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

#[derive(Debug, Clone, Copy)]
pub struct Field {
    element: u64,
}

pub const PRIME : u64 = 1000000007;

impl Field {
    // TODO 2: change exponent to not cast it to u32
    pub fn pow(base: Self, mut exponent: i64) -> Self {
        while exponent < 0 {
            exponent += (PRIME - 1) as i64;
        }
        Field { element: mod_pow(base.element, exponent as u64, PRIME) }
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

        let inverse = Field::pow(rhs, -1);
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
    use super::PRIME;

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
    fn division() {
        let result = Field::new(10) / Field::new(2);
        assert_eq!(result, Field::new(5));
    }

    #[test]
    fn inverse_using_pow() {
        let inverse = Field::pow(Field::new(10), (PRIME - 2) as i64);
        let result = Field::new(10) * inverse;
        assert_eq!(result, Field::new(1));
    }
}
