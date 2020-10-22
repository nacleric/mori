#[cfg(test)]
mod unit_tests;
use crate::error::Result;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Position {
    col: usize, // x-coord
    row: usize, // y-coord
}

impl Position {
    pub fn new() -> Self {
        Self { col: 0, row: 0 }
    }

    pub fn col(&mut self) -> usize {
        self.col
    }

    pub fn set(&mut self, col: usize, row: usize) -> Result<&mut Self> {
        self.col = col;
        self.row = row;
        Ok(self)
    }

    pub fn left(&mut self) -> Result<&mut Self> {
        if self.col != 0 {
            self.col -= 1;
        }
        Ok(self)
    }

    // Note: Might need to account for max size of string
    pub fn right(&mut self) -> Result<&mut Self> {
        self.col += 1;
        Ok(self)
    }

    pub fn row(&mut self) -> usize {
        self.row
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    data: String,
    // Use a reference here? &Position
    pos: Position,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: String::default(),
            pos: Position::default(),
        }
    }

    pub fn contents(&self) -> String {
        self.data.clone()
    }

    pub fn delete_glyph(&mut self) -> &mut Self {
        if self.pos().col() != 0 {
            self.data.drain(self.pos().col()-1..self.pos().col());
        }
        self
    }

    /// Removes data from the buffer but does not remove the entire buffer
    pub fn delete_glyphs(&mut self) -> &mut Self {
        self.data.clear();
        self
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn insert_glyph(&mut self, glyph: char) -> Result<&mut Self> {
        self.data.insert(self.pos().col(), glyph);
        self.pos.right()?;

        Ok(self)
    }

    pub fn insert_glyphs<I: Iterator<Item = char>>(&mut self, glyphs: I) -> Result<&mut Self> {
        // `try_for_each` wants `Result<(), E>` from each iteration (wants `Ok(())` or `Err(E)`)
        glyphs
            .into_iter()
            .try_for_each(|c| self.insert_glyph(c).map(|_| ()))?;
        Ok(self)
    }


    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn set_data<S: Into<String>>(&mut self, data: S) -> &mut Self {
        self.data = data.into();
        self
    }
}

// TODO: Change the name of this later
struct Big_Buffer {
    lines: Vec<Buffer>,
    pos: Position,
}

impl Big_Buffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            pos: Position::default(),
        }
    }
}