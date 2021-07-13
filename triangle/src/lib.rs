pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if let Some(i) = sides.iter().find(|&&s| s < 0) {
            None
        }
        return None;
    }

    pub fn is_equilateral(&self) -> bool {
        unimplemented!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        unimplemented!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        unimplemented!("Determine if the Triangle is isosceles.");
    }
}
