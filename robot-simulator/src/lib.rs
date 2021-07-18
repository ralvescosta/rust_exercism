#[derive(PartialEq, Debug, Clone, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct Robot(i32, i32, Direction);

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self(x, y, d)
    }

    pub fn turn_right(self) -> Self {
        match self.2 {
            Direction::North => Self(self.0, self.1, Direction::East),
            Direction::East => Self(self.0, self.1, Direction::South),
            Direction::South => Self(self.0, self.1, Direction::West),
            Direction::West => Self(self.0, self.1, Direction::North),
        }
    }

    pub fn turn_left(self) -> Self {
        match self.2 {
            Direction::North => Self(self.0, self.1, Direction::West),
            Direction::East => Self(self.0, self.1, Direction::North),
            Direction::South => Self(self.0, self.1, Direction::East),
            Direction::West => Self(self.0, self.1, Direction::South),
        }
    }

    pub fn advance(self) -> Self {
        match self.2 {
            Direction::North => Self(self.0, self.1 + 1, self.2),
            Direction::East => Self(self.0 + 1, self.1, self.2),
            Direction::South => Self(self.0, self.1 - 1, self.2),
            Direction::West => Self(self.0 - 1, self.1, self.2),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut path = self.clone();
        for c in instructions.chars() {
            match c {
                'R' => path = path.turn_right(),
                'L' => path = path.turn_left(),
                'A' => path = path.advance(),
                _ => (),
            }
        }
        path
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
