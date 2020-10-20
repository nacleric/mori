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

    pub fn update(&mut self, col: usize, row: usize) -> Result<&mut Self> {
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

    pub fn right(&mut self) -> Result<&mut Self> {
        self.col += 1;
        Ok(self)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    data: String,
    pos: Position,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            pos: Position::new(),
        }
    }

    pub fn contents(&self) -> String {
        self.data.clone()
    }

    // Note: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
    // TODO: Might need to account for glyphs that take up 2+ characters
    pub fn delete_glyph(&mut self) -> &mut Self {
        self.data.clear();
        self
    }

    pub fn insert_glyph(&mut self, glyph: char) -> Result<&mut Self> {
        // Note ask Brad about accessors again:
        self.data.insert(self.pos().col, glyph);
        // vs
        // self.data.insert(self.pos.col, glyph);
        self.pos.right()?;

        Ok(self)
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

}
