#[cfg(test)]
mod unit_tests;
use crate::error::{Error, Result};


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
}

// How default is derived
// impl Default for Position {
//     fn default() -> Self {
//         Self { usize::default(), usize::default() };
//     }
// }
