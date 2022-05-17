/// Finite Fields F_p implementation
///
use std::ops;

struct Field {
    element: u64,
}

const PRIME : u64 = 1000000007;

impl Field {
    // TODO: this grows too fast, use modular exponentiation instead
    // TODO 2: change exponent to not cast it to u32
    pub fn pow(base: Self, exponent: u64) -> Field {
        Field { element: u64::pow(base.element, exponent as u32) % PRIME }
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

        let inverse = Field::pow(rhs, PRIME - 2);
        self * inverse
    }
}
