///
pub struct Square {
    rank: usize,
    file: usize,
}

impl Square {
    ///
    pub fn from_idx(i: usize) -> Self {
        Square { rank: i / 8, file: i % 8 }
    }

    ///
    pub fn rank(&self) -> usize {
        self.rank
    }

    ///
    pub fn file(&self) -> usize {
        self.file
    }

    ///
    pub fn index(&self) -> usize {
        self.rank * 8 + self.file
    }
}
