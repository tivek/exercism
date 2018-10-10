// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Robot {
    d: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        println!("{} {}", x, y);
        Robot { x: x, y: y, d: d }
    }

    pub fn turn_right(self) -> Self {
        Self::new(self.x, self.y, self.d.turn_right())
    }

    pub fn turn_left(self) -> Self {
        Self::new(self.x, self.y, self.d.turn_left())
    }

    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self::new(self.x, self.y + 1, self.d),
            Direction::East => Self::new(self.x + 1, self.y, self.d),
            Direction::South => Self::new(self.x, self.y - 1, self.d),
            Direction::West => Self::new(self.x - 1, self.y, self.d),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut state = self;
        for i in instructions.chars() {
            state = match i {
                'R' => state.turn_right(),
                'L' => state.turn_left(),
                'A' => state.advance(),
                _ => state,
            };
        }
        state
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
