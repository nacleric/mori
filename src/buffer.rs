pub mod direction;
#[cfg(test)]
mod unit_tests;
use crate::{
    error::{Error, Result},
    interfaces::GraphemeBuffer,
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

impl Bufferable for Buffer {

}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_row(&mut self, pos: Position) -> Position {
        // Enter Key-event: Add a new empty buffer when pressing enter
        // (Policy) If enter is pressed mid-string, data to the right of cursor is put into new line
        let (_, row) = pos.as_tuple();
        self.rows.insert(row + 1, String::new());
        let new_pos = self.move_down(pos);
        new_pos
    }

    pub fn delete_row() {
        // Backspace Key-event: Remove buffer if index[0] get's deleted
        // (Policy) If elements still exist in buffer, move data to the row above it
        unimplemented!()
    }

    pub fn move_down(&self, pos: Position) -> Position {
        let (col, row) = pos.as_tuple();
        let lower_bound = self.rows.len() - 1;

        let pos: Position;
        if row == lower_bound {
            pos = Position::new(col, row);
        } else {
            let eol_next_row = self.rows[row + 1].len();
            if col > eol_next_row {
                pos = Position::new(eol_next_row, row + 1);
            } else {
                pos = Position::new(col, row + 1);
            }
        }
        pos
    }

    pub fn move_left(&self, pos: Position) -> Position {
        let (col, row) = pos.as_tuple();
        let pos = match col {
            0 => {
                if row == 0 {
                    Position::new(col, row)
                } else {
                    let eol = self.rows[col].len();
                    Position::new(eol, row.saturating_sub(1))
                }
            }
            _ => Position::new(col.saturating_sub(1), row),
        };
        pos
    }

    pub fn move_right(&self, pos: Position) -> Position {
        let (col, row) = pos.as_tuple();
        let lower_bound = self.rows.len() - 1;
        let eol = self.rows[row].len();

        let pos: Position;
        if col == eol {
            if row == lower_bound {
                pos = Position::new(col, row);
            } else {
                pos = Position::new(0, row + 1);
            }
        } else {
            pos = Position::new(col + 1, row);
        }
        pos
    }

    pub fn move_up(&self, pos: Position) -> Position {
        let (col, row) = pos.as_tuple();

        let pos: Position;
        if row == 0 {
            pos = Position::new(col, row);
        } else {
            let eol_prev_row = self.rows[row - 1].len();
            if col > eol_prev_row {
                pos = Position::new(eol_prev_row, row - 1);
            } else {
                pos = Position::new(col, row - 1);
            }
        }
        pos
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
                    self.move_left(pos);
                    removed_grapheme
                });
                (
                    Position::new(col.saturating_sub(1), row),
                    opt_removed_grapheme,
                )
            }
        }
    }

    // WIP: needs to implement a range
    fn delete_graphemes(&mut self) -> (Position, Vec<String>) {
        unimplemented!()
    }

    // TODO: Will need policies for movement. Switch back to index grapheme eventually
    fn insert_grapheme(&mut self, pos: Position, grapheme: char) -> Position {
        let (col, row) = pos.as_tuple();
        // let index = self.index();
        // self.rows[row].insert(index, grapheme);
        self.rows[row].insert(col, grapheme);
        let new_pos = self.move_right(pos);
        new_pos
    }

    fn insert_graphemes<I: Iterator<Item = char>>(
        &mut self,
        mut pos: Position,
        graphemes: I,
    ) -> Position {
        graphemes.into_iter().for_each(|c| {
            pos = self.insert_grapheme(pos, c);
        });
        pos
    }

    fn index(&self, pos: Position) -> usize {
        let (col, row) = pos.as_tuple();
        let index = match col {
            0 => 0,
            col => {
                dbg!(
                    self.rows[row]
                        .grapheme_indices(true)
                        .nth(col)
                        .expect("invalid position")
                        .1
                );
                self.rows[row]
                    .grapheme_indices(true)
                    .nth(col)
                    .expect("invalid position")
                    .0
            }
        };
        index
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

/*
// TODO: replace ColumnState & RowState with newtype
struct Column(usize); // newtype pattern
impl Column {
    // "predicate" special name for function that takes no params and returns bool
    #[inline]
    pub fn is_beginning_of_line(&self) -> bool {
        self.0 == 0
    }
}
*/

impl View for Buffer {
    // Note: Passing a trait constrains type to types that implement the write Trait
    fn show<W: Write>(&self, writer: &mut W) -> Result<&Self> {
        // Note: byte smaller than pointer, better to copy than reference(&)
        writer.write_all(
            &self
                .content()
                .iter()
                .map(|s| s.as_bytes().iter().map(|b| *b))
                .flatten()
                .collect::<Vec<u8>>(),
        )?;
        Ok(self)
    }
}
