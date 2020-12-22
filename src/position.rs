#[cfg(test)]
mod unit_tests;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Position {
    col: usize,
    row: usize,
}

impl Position {
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

    // Movements will need policy
    // Lol this is confusing down is up. up is down in the view of texteditor
    pub fn move_down(&self) -> Position {
        let pos = Position::new(self.col(), self.row() - 1);
        pos
    }

    pub fn move_left(&self) -> Position {
        let pos = Position::new(self.col().saturating_sub(1), self.row());
        pos
    }

    pub fn move_right(&self) -> Position {
        let pos = Position::new(self.col().saturating_add(1), self.row());
        pos
    }

    pub fn move_up(&self) -> Position {
        let pos = Position::new(self.col(), self.row() + 1);
        pos
    }
}

// How default is derived
// impl Default for Position {
//     fn default() -> Self {
//         Self { usize::default(), usize::default() };
//     }
// }