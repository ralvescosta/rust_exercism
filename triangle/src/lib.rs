use std::ops::Add;
pub struct Triangle<T>(T, T, T);

impl<T: Copy + PartialOrd + Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let a = sides[0];
        let b = sides[1];
        let c = sides[2];

        if a + b > c && b + c > a && a + c > b {
            return Some(Triangle(a, b, c));
        }
        return None;
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.0 != self.2 && self.1 != self.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.0 == self.2 || self.1 == self.2
    }
}
