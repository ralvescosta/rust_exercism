use std::usize;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();

        for row in 0..row_count {
            if row == 0 {
                triangle.push(vec![1 as u32]);
                continue;
            }
            triangle.push(vec![1 as u32]);
            if row == 1 {
                triangle[row as usize].push(1 as u32);
                continue;
            }
            for column in 1..row {
                let a = triangle[row as usize - 1][column as usize - 1];
                let b = triangle[row as usize - 1][column as usize];
                triangle[row as usize].push(a + b);
            }
            triangle[row as usize].push(1 as u32);
        }

        return PascalsTriangle { rows: triangle };
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        return self.rows;
    }
}
