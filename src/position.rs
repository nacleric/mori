#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Position {
    col: usize, // x-coord
    row: usize, // y-coord
}

impl Position {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn cr_coords(&self) -> (usize, usize) {
        (self.col(), self.row())
    }

    pub fn row(&self) -> usize {
        self.row
    }
}
