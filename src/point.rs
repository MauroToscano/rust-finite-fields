/// Curve Point<const A: u128, B: u128> implementation
///
use std::ops;

// TODO: change i128 for a generic implementing prime field
#[derive(Debug, Copy, Clone)]
pub enum Point<const A: i128, const B: i128> {
    Infinity,
    Finite { x: i128, y: i128 },
}

impl<const A: i128, const B: i128> Point<A, B> {
    pub fn new(x: i128, y: i128) -> Self {
        if y * y != x * x * x + A * x + B {
            panic!("({}, {}) is not on the curve", x, y);
        }
        Self::Finite { x, y }
    }
}

impl<const A: i128, const B: i128> ops::Add for Point<A, B> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Infinity, Self::Infinity) => self.clone(),
            (Self::Infinity, Self::Finite { .. }) => other.clone(),
            (Self::Finite { .. }, Self::Infinity) => self.clone(),
            (Self::Finite { x: x1, y: y1 }, 
             Self::Finite { x: x2, y: y2 }) if x1 == x2 && y1 == y2 => {
                if y1 == 0 {
                    Self::Infinity
                } else {
                    let s = (3 * (x1 * x1) + A) / (2 * y1);
                    let x3 = (s * s) - (2 * x1);
                    let y3 = (s * (x1 - x3)) - y1;
                    Self::new(x3, y3)
                }
            }
            (Self::Finite { x: x1, y: y1 }, 
             Self::Finite { x: x2, y: y2 }) if x1 == x2 && y1 != y2 => {
                    Self::Infinity
             }
            (Self::Finite { x: x1, y: y1 }, 
             Self::Finite { x: x2, y: y2 }) => {
                let s = (y2 - y1) / (x2 - x1);
                let x3 = (s * s) - x1 - x2;
                let y3 = (s * (x1 - x3)) - y1;
                Self::new(x3, y3)
            }
        }
    }
}

impl<const A: i128, const B: i128> PartialEq for Point<A, B> {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Infinity, Self::Infinity) => true,
            (Self::Infinity, Self::Finite { .. }) => false,
            (Self::Finite { .. }, Self::Infinity) => false,
            (Self::Finite { x: x1, y: y1 }, Self::Finite { x: x2, y: y2 }) => x1 == x2 && y1 == y2,
        }
    }

    fn ne(&self, rhs: &Self) -> bool {
        return !self.eq(rhs);
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

    // secp256k1 parameters
    const A: i128 = 0;
    const B: i128 = 8;
    type TestPoint = Point<A, B>;
    // const INFINITY: TestPoint = TestPoint::Infinity;

    #[test]
    fn addition_different_x() {
        let result = TestPoint::new(1, 3) + TestPoint::new(2, 4);
        assert_eq!(result, Point::new(-2, 0));
    }

    #[test]
    fn addition_opposite_points() {
        let result = TestPoint::new(1, 3) + TestPoint::new(1, -3);
        assert_eq!(result, Point::Infinity);
    }

    #[test]
    fn addition_same_x_and_y_zero() {
        let result = Point::<0, 8>::new(-2, 0) + Point::<0, 8>::new(-2, 0);
        assert_eq!(result, Point::Infinity);
    }

    #[test]
    fn addition_same_points_y_not_zero() {
        let result = Point::<0, 0>::new(4, 8) + Point::<0, 0>::new(4, 8);
        assert_eq!(result, Point::<0, 0>::new(1, 1));
    }

    #[test]
    fn equal() {
        assert!(TestPoint::new(1, 3) == TestPoint::new(1, 3))
    }

    #[test]
    fn not_equal() {
        assert!(TestPoint::new(1, 3) != TestPoint::new(2, 4))
    }
}
