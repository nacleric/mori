pub mod direction;
#[cfg(test)]
mod unit_tests;
use crate::{interfaces::View, position};
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
            // rows: Vec::default(),
            rows: vec![String::new()],
        }
    }

    /// Removes data from the buffer but does not remove the entire buffer
    pub fn clear(&mut self) -> (Position, Vec<String>) {
        let pos = Position::new(0, 0 );
        let orig_value = std::mem::replace(&mut self.rows, Vec::new());
        (pos, orig_value)
    }

    // TODO insert might need to be recursive
    pub fn insert_glyphs<I: Iterator<Item = char>>(&mut self, glyphs: I, pos: Position) {
        glyphs.into_iter().for_each(|c| {
            let pos = self.insert_glyph(pos, c);
        });
    }

    pub fn insert_row(&mut self, pos: Position) -> Position { 
        // Enter Key-event: Add a new empty buffer when pressing enter
        // (Policy) If enter is pressed mid-string, data to the right of cursor is put into new line
        let (_, row) = pos.as_tuple();
        self.rows.insert(row + 1, String::new());
        let new_pos = pos.move_up();
        new_pos
    }

    pub fn delete_row() {
        // Backspace Key-event: Remove buffer if index[0] get's deleted
        // (Policy) If elements still exist in buffer, move data to the row above it
        unimplemented!()
    }
}

impl GlyphBuffer for Buffer {
    type Error = Error;

    fn content(&self) -> Vec<String> {
        self.rows.clone()
    }

    // Semantically guarantees something gets deleted but if theres nothing to delete than innacurate name
    // TODO: add Position as an argument
    fn delete_glyph(&mut self, direction: Direction, pos: Position) -> (Position, Option<char>) {
        let (col, row) = pos.as_tuple(); 
        match direction {
            Direction::Forward => {
                let mut glyphs = std::str::from_utf8(self.row_content(pos))
                    .expect("Returns a &str")
                    .to_owned();
                let opt_removed_glyph = glyphs.chars().nth(col).map(|removed_glyph| {
                    glyphs.remove(col);

                    self.set_row_content(pos, glyphs).unwrap_or_else(|_| {
                        unreachable!(
                            "`set_row_content()` is always expected to update the buffer after glyph is deleted"
                        )
                    });
                    // Note: Only move_left() if at the end of line (will probably be in a policy function)
                    // pos.move_left();
                    removed_glyph
                });
                (Position::new(col, row), opt_removed_glyph)
            }
            Direction::Backward => {
                let mut glyphs = std::str::from_utf8(self.row_content(pos))
                    .expect("Returns a &str")
                    .to_owned();
                let opt_removed_glyph = glyphs.chars().nth(col.saturating_sub(1)).map(|removed_glyph| {
                    glyphs.remove(col.saturating_sub(1));

                    self.set_row_content(pos, glyphs).unwrap_or_else(|_| {
                        unreachable!(
                            "`set_row_content()` is always expected to update the buffer after glyph is deleted"
                        )
                    });
                    if col != 0 {
                        // Note: If at beginning of line *don't* move left (will probably be in a policy function)
                        pos.move_left();
                    }
                    removed_glyph
                });
                (Position::new(col.saturating_sub(1), row), opt_removed_glyph)
            }
        }
    }

    // TODO: Will need policies for movement. Switch back to index grapheme eventually
    fn insert_glyph(&mut self, pos: Position, glyph: char) -> Position {
        let (col, row) = pos.as_tuple(); 
        // let index = self.index();
        self.rows[row].insert(col, glyph);
        let new_pos = pos.move_right();
        new_pos
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
        let (_ , row) = pos.as_tuple();
        self.rows[row].as_bytes()
    }

    // Might not need this
    fn set_row_content(&mut self, pos: Position, data: String) -> Result<&mut Self, Self::Error> {
        let (col, row) = pos.as_tuple();
        self.rows[row] = data;
        Ok(self)
    }
}

// How default is derived
// impl Default for Buffer {
//     fn default() -> Self {
//         Self { 
//             rows: vec![String::new()],
//         }
//     }
// }

impl From<Vec<String>> for Buffer {
    fn from(data: Vec<String>) -> Self {
        let mut buf = Buffer::new();
        buf.rows = data;
        buf
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
