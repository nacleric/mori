#[cfg(test)]
mod unit_tests;
use crate::error::Result;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Position {
    col: u32, // x-coord
    row: u32, // y-coord
}

impl Position {
    pub fn new() -> Self {
        Self { col: 0, row: 0 }
    }

    pub fn update(&mut self, col: u32, row: u32) {
        self.col = col;
        self.row = row;
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    data: String,
    pos: Position,
}

impl Buffer {
    pub fn new() -> Self {
        Self { data: String::new(), pos: Position::new() }
    }

    pub fn insert_glyph(&mut self, glyph: char) -> Result<&mut Self> {
        self.data.push(glyph);
        Ok(self)
    }

    // Note: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
    // TODO: Might need to account for glyphs that take up 2+ characters
    pub fn delete_glyph(&mut self) -> Result<&mut Self> {
        let data = self.data.chars().nth(self.pos.col as usize).unwrap();
        // let data = self.data.chars();
        self.pos.col -= 1;
        Ok(self)
    }

    pub fn contents(&self) -> String {
        self.data.clone()
    }
}
