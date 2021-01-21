#[cfg(test)]
mod unit_tests;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CursorPosition {
    col: usize,
    row: usize,
}

impl CursorPosition {
    // type_constructor
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
