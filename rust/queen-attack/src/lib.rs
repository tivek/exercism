#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if 0 <= rank && rank <= 7 && 0 <= file && file <= 7 {
            Some(ChessPosition {
                rank: rank,
                file: file,
            })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let delta_rank = self.position.rank - other.position.rank;
        let delta_file = self.position.file - other.position.file;
        delta_rank == 0 || delta_file == 0 || delta_rank.abs() == delta_file.abs()
    }
}
