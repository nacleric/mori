#[cfg(test)]
mod unit_tests;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

/*
impl Position for CursorPosition {
    fn move_down(&self, pos: CursorPosition) -> CursorPosition {
        let (col, row) = pos.as_tuple();
        let pos = CursorPosition::new(col, row + 1);
        pos
    }

    fn move_left(&self, pos: CursorPosition) -> CursorPosition {
        let (col, row) = pos.as_tuple();
        let pos = CursorPosition::new(col - 1, row);
        pos
    }

    fn move_right(&self, pos: CursorPosition) -> CursorPosition {
        let (col, row) = pos.as_tuple();
        let pos = CursorPosition::new(col + 1, row);
        pos
    }

    fn move_up(&self, pos: CursorPosition) -> CursorPosition {
        let (col, row) = pos.as_tuple();
        let pos = CursorPosition::new(col, row - 1);
        pos
    }
}
*/
