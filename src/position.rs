// Default is also a constructor
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Position {
    col: usize, // x-coord
    row: usize, // y-coord
}

impl Position {
    // type_constructor
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.col(), self.row())
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

// How default is derived
// impl Default for Position {
//     fn default() -> Self {
//         Self { usize::default(), usize::default() };
//     }
// }