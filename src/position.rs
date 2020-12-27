use crate::interfaces::Movement;
#[cfg(test)]
mod unit_tests;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Position {
    col: usize,
    row: usize,
    // policy: PositionPolicy
}

impl Position {
    // type_constructor
    pub fn new(col: usize, row: usize, /*policy: PositionPolicy*/) -> Self {
        Self { col, row, /*policy*/ }
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

impl Movement for Position {
    // TODO: Movements will need policy injected (constructor dependency injection) maybe try composition
    fn move_down(&self) -> Position {
        let pos = Position::new(self.col(), self.row() + 1);
        pos
    }

    fn move_left(&self) -> Position {
        let pos = Position::new(self.col() - 1, self.row());
        pos
    }

    fn move_right(&self) -> Position {
        let pos = Position::new(self.col() + 1, self.row());
        pos
    }

    fn move_up(&self) -> Position {
        let pos = Position::new(self.col(), self.row() - 1);
        pos
    }
}

// How default is derived
// impl Default for Position {
//     fn default() -> Self {
//         Self { usize::default(), usize::default() };
//     }
// }
