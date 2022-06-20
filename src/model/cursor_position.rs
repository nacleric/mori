#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CursorPosition {
    col: usize,
    row: usize,
}

impl CursorPosition {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.col(), self.row())
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self {
            col: usize::default(),
            row: usize::default(),
        }
    }
}
