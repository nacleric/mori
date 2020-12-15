pub mod direction;
#[cfg(test)]
mod unit_tests;
use crate::interfaces::View;
use crate::{
    error::{Error, Result},
    interfaces::GlyphBuffer,
    position::Position,
};
use direction::Direction;
use std::io::Write;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    rows: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rows: Vec::default(),
        }
    }

    /// Removes data from the buffer but does not remove the entire buffer
    pub fn delete_glyphs(&mut self) -> &mut Self {
        // self.data.clear();
        // self
        unimplemented!();
    }

    pub fn insert_glyphs<I: Iterator<Item = char>>(&mut self, glyphs: I) -> &mut Self {
        glyphs.into_iter().for_each(|c| {
            self.insert_glyph(c);
        });
        self
    }

    pub fn add_row() {
        // Enter Key-event: Add a new empty buffer when pressing enter
        // (Policy) If enter is pressed mid-string, data to the right of cursor is put into new line
        unimplemented!()
    }

    pub fn delete_row() {
        // Backspace Key-event: Remove buffer if index[0] get's deleted
        // (Policy) If elements still exist in buffer, move data to the row above it
        unimplemented!()
    }
}

impl GlyphBuffer for Buffer {
    type Error = Error;

    fn content(&self, pos: Position) -> Vec<String> {
        self.rows
    }

    // Semantically guarantees something gets deleted but if theres nothing to delete than innacurate name
    // TODO: add Position as an argument
    fn delete_glyph(&mut self, direction: Direction, pos: Position) -> Option<char> {
        let (col, row) = pos.as_tuple(); 
        let glyph = match direction {
            Direction::Forward => {
                let mut glyphs = std::str::from_utf8(self.row_content(pos))
                    .expect("Returns a &str")
                    .to_owned();
                let removed_glyph = glyphs.chars().nth(col)?;
                glyphs.remove(col);

                self.set_row_content(glyphs).unwrap_or_else(|_| {
                    unreachable!(
                        "`set_row_content()` is always expected to update the buffer after glyph is deleted"
                    )
                });
                // Note: Only move_left() if at the end of line (will probably be in a policy function)
                self.move_left().unwrap_or_else(|| {
                    unreachable!(
                        "`move_left()` expected to always suceed immediately following `delete()`."
                    )
                });
                removed_glyph
            }
            Direction::Backward => {
                let mut glyphs = std::str::from_utf8(self.row_content(pos))
                    .expect("Returns a &str")
                    .to_owned();
                let removed_glyph = glyphs.chars().nth(col.saturating_sub(1))?;
                glyphs.remove(col.saturating_sub(1));

                self.set_row_content(glyphs).unwrap_or_else(|_| {
                    unreachable!(
                        "`set_row_content()` is always expected to update the buffer after glyph is deleted"
                    )
                });
                if col != 0 {
                    // Note: If at beginning of line *don't* move left (will probably be in a policy function)
                    self.move_left().unwrap_or_else(|| {
                        unreachable!(
                            "`move_left()` expected to always succeed immediately following `delete()`."
                        )
                    });
                }
                removed_glyph
            }
        };
        Some(glyph)
    }

    fn insert_glyph(&mut self, glyph: char, pos: Position) -> Position {
        // TODO: From Rust traceback
        let (_col, row) = pos.as_tuple(); 
        self.rows[row].insert(self.index(), glyph);
        self.move_right(pos).unwrap_or_else(|| {
            unreachable!(
                "`move_right()` expected to always succeed immediately following an `insert()`."
            )
        });
        pos
    }

    // TODO: pos state removed. Return Position instead?
    fn move_down(&mut self) -> Option<&mut Self> {
        unimplemented!();
    }

    fn move_left(&mut self) -> Option<&mut Self> {
        unimplemented!();
    }

    fn move_right(&mut self, pos: Position) -> Option<Position> {
        // unimplemented!();
        // Create new Position or reference old pos?
        // v1: pos.col() + 1;
        let (col, row) = pos.as_tuple();
        let new_pos = Position::new(col + 1, row);
        Some(new_pos)
    }

    fn move_up(&mut self) -> Option<&mut Self> {
        unimplemented!();
    }

    // TODO: WIP needs to account for rows and columns. Vec of String
    // Might not need information of what row it belongs to
    fn index(&self) -> usize {
        // match self.pos().col() {
        //     0 => 0,
        //     pos => self
        //         .data[self.pos().row()] // .data is for Row Buffer? Assumes Vector
        //         .grapheme_indices(true)
        //         .nth(self.pos().col() - 1)
        //         .expect("Invalid position") // usize is index &str utf8 representation of the char
        //         .0,
        // }
        unimplemented!();
    }

    // show row_content for the entire buffer or just a line?
    // !Most likely accessing content of a single line
    // make a separate accessor for the ENTIRE buffer
    // TODO: make content only handle a single line
    fn row_content(&self, pos: Position) -> &[u8] {
        let (_col, row) = pos.as_tuple();
        self.rows[row].as_bytes()
    }

    // Might not need this
    fn set_row_content(&mut self, data: String) -> Result<&mut Self, Self::Error> {
        // self.data = data;
        // Ok(self)
        unimplemented!();
    }
}

impl View for Buffer {
    // Note: Constrains type to types that implement the write Trait
    fn show<W: Write>(&self, writer: &mut W) -> Result<&Self> {
        // writer.write_all(self.content())?;
        // Ok(self)
        unimplemented!();
    }
}
