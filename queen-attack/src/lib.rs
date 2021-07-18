#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank >= 8 || file < 0 || file >= 8 {
            return None;
        }
        Some(Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let mut rank_diff = self.0.rank - other.0.rank;
        if rank_diff < 0 {
            rank_diff *= -1
        }
        let mut file_diff = self.0.file - other.0.file;
        if file_diff < 0 {
            file_diff *= -1
        }
        if self.0.rank == other.0.rank || self.0.file == other.0.file || (rank_diff == file_diff) {
            return true;
        }
        false
    }
}
