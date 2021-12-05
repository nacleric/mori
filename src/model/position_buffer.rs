use std::cmp::min;

use crate::model::cursor_position::CursorPosition;
use crate::{
    interfaces::RowBuffer,
    model::utf8_buffer::{direction::Direction, Utf8Buffer},
};

#[cfg(test)]
mod unit_tests;

#[derive(Debug, Eq, PartialEq)]
pub struct PositionBuffer {
    data: Utf8Buffer,
    position: CursorPosition,
}

impl PositionBuffer {
    pub fn new(buffer: Utf8Buffer, position: CursorPosition) -> Self {
        Self {
            data: buffer,
            position,
        }
    }

    pub fn delete_grapheme(&mut self, direction: Direction) -> Option<char> {
        let (col, row) = self.position.as_tuple();
        let eol = self.data.col_count(row);
        match direction {
            Direction::Backward => {
                if col != 0 {
                    self.move_left();
                    let deleted_char = self
                        .data
                        .edit_row(row)
                        .expect("could not get row contents")
                        .chars()
                        .nth(col.saturating_sub(1));
                    self.data
                        .edit_row(row)
                        .expect("could not get row contents")
                        .remove(col.saturating_sub(1));
                    deleted_char
                } else {
                    None
                }
            }
            Direction::Forward => {
                if col != eol {
                    let deleted_char = self
                        .data
                        .edit_row(row)
                        .expect("could not get row contents")
                        .chars()
                        .nth(col);
                    self.data
                        .edit_row(row)
                        .expect("could not get row contents")
                        .remove(col);
                    deleted_char
                } else {
                    None
                }
            }
        }
    }

    pub fn index(&mut self) -> &mut Self {
        // TODO: returns index of grapheme indices
        unimplemented!()
    }

    pub fn insert_grapheme(&mut self, grapheme: char) -> &mut Self {
        let (col, row) = self.position.as_tuple();
        // let index = self.index();
        // self.rows[row].insert(index, grapheme);
        self.data
            .edit_row(row)
            .expect("could not get row")
            .insert(col, grapheme);
        self.move_right()
    }

    pub fn move_down(&mut self) -> &mut Self {
        let (col, row) = self.position.as_tuple();
        let max_row = self.data.row_count() - 1;

        let new_pos: CursorPosition;
        if row == max_row {
            new_pos = self.position;
        } else {
            let new_row = row + 1;
            new_pos = CursorPosition::new(min(self.data.col_count(new_row), col), new_row);
        }
        self.position = new_pos;
        self
    }

    pub fn move_left(&mut self) -> &mut Self {
        let (col, row) = self.position.as_tuple();
        let eol = self.data.col_count(row);

        let new_pos: CursorPosition;
        if col == 0 {
            if row == 0 {
                new_pos = CursorPosition::new(col, row);
            } else {
                new_pos = CursorPosition::new(eol, row - 1);
            }
        } else {
            new_pos = CursorPosition::new(col - 1, row);
        };
        self.position = new_pos;
        self
    }

    pub fn move_right(&mut self) -> &mut Self {
        let (col, row) = self.position.as_tuple();
        let max_row = self.data.row_count() - 1;
        let eol = self.data.col_count(row);

        let new_pos: CursorPosition;
        if col == eol {
            if row == max_row {
                new_pos = CursorPosition::new(col, row);
            } else {
                new_pos = CursorPosition::new(0, row + 1);
            }
        } else {
            new_pos = CursorPosition::new(col + 1, row);
        };
        self.position = new_pos;
        self
    }

    pub fn move_up(&mut self) -> &mut Self {
        let (col, row) = self.position.as_tuple();

        let new_pos: CursorPosition;
        if row == 0 {
            new_pos = self.position;
        } else {
            let new_row = row - 1;
            new_pos = CursorPosition::new(min(self.data.col_count(new_row), col), new_row);
        }
        self.position = new_pos;
        self
    }
}

impl Default for PositionBuffer {
    fn default() -> Self {
        Self {
            data: Utf8Buffer::default(),
            position: CursorPosition::default(),
        }
    }
}
