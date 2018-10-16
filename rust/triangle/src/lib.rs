use std::collections::BTreeSet;

pub struct Triangle {
    sides: BTreeSet<u64>,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] + sides[1] > sides[2]
            && sides[1] + sides[2] > sides[0]
            && sides[2] + sides[0] > sides[1]
        {
            Some(Triangle {
                sides: sides.into_iter().cloned().collect(),
            })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.len() <= 2
    }
}
