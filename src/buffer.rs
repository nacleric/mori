pub mod direction;
#[cfg(test)]
mod unit_tests;
use crate::{
    error::{Error, Result},
    interfaces::GraphemeBuffer,
    interfaces::Movement,
    position::Position,
};
use crate::{interfaces::View, position};
use direction::Direction;
use std::io::Write;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Eq, PartialEq)]
pub struct Buffer {
    rows: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    // WIP: needs to implement a range
    pub fn delete_graphemes(&mut self) -> (Position, Vec<String>) {
        unimplemented!()
    }

    pub fn insert_graphemes<I: Iterator<Item = char>>(
        &mut self,
        mut pos: Position,
        graphemes: I,
    ) -> Position {
        graphemes.into_iter().for_each(|c| {
            pos = self.insert_grapheme(pos, c);
        });
        pos
    }

    pub fn insert_row(&mut self, pos: Position) -> Position {
        // Enter Key-event: Add a new empty buffer when pressing enter
        // (Policy) If enter is pressed mid-string, data to the right of cursor is put into new line
        let (_, row) = pos.as_tuple();
        self.rows.insert(row + 1, String::new());
        let new_pos = pos.move_down();
        new_pos
    }

    pub fn delete_row() {
        // Backspace Key-event: Remove buffer if index[0] get's deleted
        // (Policy) If elements still exist in buffer, move data to the row above it
        unimplemented!()
    }
}

impl GraphemeBuffer for Buffer {
    type Error = Error;

    fn content(&self) -> Vec<String> {
        self.rows.clone()
    }

    // Semantically guarantees something gets deleted but if theres nothing to delete than innacurate name
    // TODO: add Position as an argument
    fn delete_grapheme(&mut self, direction: Direction, pos: Position) -> (Position, Option<char>) {
        let (col, row) = pos.as_tuple();
        match direction {
            Direction::Forward => {
                let mut graphemes = std::str::from_utf8(self.row_content(pos))
                    .expect("Returns a &str")
                    .to_owned();
                let opt_removed_grapheme = graphemes.chars().nth(col).map(|removed_grapheme| {
                    graphemes.remove(col);

                    self.set_row_content(pos, graphemes).unwrap_or_else(|_| {
                        unreachable!(
                            "`set_row_content()` is always expected to update the buffer after grapheme is deleted"
                        )
                    });
                    // Note: Only move_left() if at the end of line (will probably be in a policy function)
                    // pos.move_left();
                    removed_grapheme
                });
                (Position::new(col, row), opt_removed_grapheme)
            }
            Direction::Backward => {
                let mut graphemes = std::str::from_utf8(self.row_content(pos))
                    .expect("Returns a &str")
                    .to_owned();
                let opt_removed_grapheme = graphemes.chars().nth(col.saturating_sub(1)).map(|removed_grapheme| {
                    graphemes.remove(col.saturating_sub(1));

                    self.set_row_content(pos, graphemes).unwrap_or_else(|_| {
                        unreachable!(
                            "`set_row_content()` is always expected to update the buffer after grapheme is deleted"
                        )
                    });
                    if col != 0 {
                        // Note: If at beginning of line *don't* move left (will probably be in a policy function)
                        pos.move_left();
                    }
                    removed_grapheme
                });
                (
                    Position::new(col.saturating_sub(1), row),
                    opt_removed_grapheme,
                )
            }
        }
    }

    // TODO: Will need policies for movement. Switch back to index grapheme eventually
    fn insert_grapheme(&mut self, pos: Position, grapheme: char) -> Position {
        let (col, row) = pos.as_tuple();
        // let index = self.index();
        // self.rows[row].insert(index, grapheme);
        self.rows[row].insert(col, grapheme);
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

    fn row_content(&self, pos: Position) -> &[u8] {
        let (_, row) = pos.as_tuple();
        self.rows[row].as_bytes()
    }

    fn set_row_content(&mut self, pos: Position, data: String) -> Result<&mut Self, Self::Error> {
        let (_col, row) = pos.as_tuple();
        self.rows[row] = data;
        Ok(self)
    }
}

// How default is derived
// TODO: consider making row its own type (typestate)
impl Default for Buffer {
    fn default() -> Self {
        Self {
            rows: vec![String::new()],
        }
    }
}

impl From<Vec<String>> for Buffer {
    fn from(data: Vec<String>) -> Self {
        let mut buf = Buffer::new();
        buf.rows = data;
        buf
    }
}

impl View for Buffer {
    // Note: Passing a trait constrains type to types that implement the write Trait
    fn show<W: Write>(&self, writer: &mut W) -> Result<&Self> {
        // Note: byte smaller than pointer, better to copy than reference(&)
        writer.write_all(
            &self
                .content()
                .iter()
                .map(|s| s
                    .as_bytes()
                    .iter()
                    .map(|b| *b))
                .flatten()
                .collect::<Vec<u8>>(),
        )?;
        Ok(self)
    }
}

