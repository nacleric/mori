#[cfg(test)]
mod unit_tests;
use crate::error::Result;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    data: String,
}

impl Buffer {
    pub fn new() -> Self {
        Self { data: String::new() }
    }

    pub fn insert_glyph(&mut self, glyph: char) -> Result<&mut Self> {
        self.data.push(glyph);
        Ok(self)
    }

    pub fn contents(&self) -> String {
        self.data.clone()
    }
}
