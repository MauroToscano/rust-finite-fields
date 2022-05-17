/// Finite Fields F_p implementation
///
use std::ops;

#[inline]
/* Iterative Function to calculate (base^exponent)%modulo in O(log exponent) */
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
struct Field {
    element: u64,
}

const PRIME : u64 = 1000000007;

impl Field {
    // TODO 2: change exponent to not cast it to u32
    pub fn pow(base: Self, mut exponent: i64) -> Field {
        while exponent < 0 {
            exponent += (PRIME - 1) as i64;
        }
        Field { element: mod_pow(base.element, exponent as u64, PRIME) }
    }
}

impl ops::Add<Field> for Field {
    type Output = Field;

    fn add(self, rhs: Field) -> Field {
        Field { element: (self.element + rhs.element) % PRIME }
    }
}

impl ops::Sub<Field> for Field {
    type Output = Field;

    fn sub(self, rhs: Field) -> Field {
        Field { element: (self.element - rhs.element) % PRIME }
    }

}

impl ops::Mul<Field> for Field {
    type Output = Field;

    fn mul(self, rhs: Field) -> Field {
        Field { element: (self.element * rhs.element) % PRIME }
    }
}


impl ops::Div<Field> for Field {
    type Output = Field;

    fn div(self, rhs: Field) -> Field {
        if rhs.element % PRIME == 0 {
            panic!("Cannot divide by 0");
        }

        let inverse = Field::pow(rhs, -1);
        self * inverse
    }
}
