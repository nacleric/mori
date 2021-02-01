#[cfg(test)]
mod unit_tests;

pub mod direction;

use crate::interfaces::Buffer;
use non_empty_vec::NonEmpty;
// use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
pub struct Utf8Buffer {
    rows: NonEmpty<<Self as Buffer>::Row>,
}

impl Utf8Buffer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Buffer for Utf8Buffer {
    type Row = String;

    fn append_row(&mut self) -> &mut Self {
        self.rows.push(String::new());
        self
    }

    fn delete_row(&mut self, row_index: usize) -> Option<Self::Row> {
        unimplemented!()
    }

    fn edit_row(&mut self, row_index: usize) -> Option<&mut Self::Row> {
        unimplemented!()
    }

    fn insert_row(&mut self, row_index: usize) -> &mut Self {
        unimplemented!()
    }

    fn row_count(&self) -> usize {
        self.rows.len().into()
    }
}

impl Default for Utf8Buffer {
    fn default() -> Self {
        Self {
            rows: NonEmpty::new(vec![String::new()])
                .expect("Internal error: Must provide non-empty `Vec` to `Default`"),
        }
    }
}

// impl Buffer for Utf8Buffer {
//     fn insert_row(&mut self, pos: CursorPosition) -> CursorPosition {
//         // Enter Key-event: Add a new empty buffer when pressing enter
//         // (Policy) If enter is pressed mid-string, data to the right of cursor is put into new line
//         let (_, row) = pos.as_tuple();
//         self.rows.insert(row + 1, String::new());
//         let new_pos = self.move_down(pos);
//         new_pos
//     }
//
//     fn delete_row(&mut self, pos: CursorPosition) -> CursorPosition {
//         // Backspace Key-event: Remove buffer if index[0] get's deleted
//         // (Policy) If elements still exist in buffer, move data to the row above it
//         unimplemented!()
//     }
// }
//
// impl GraphemeBuffer for Utf8Buffer {
//     type Error = Error;
//
//     fn content(&self) -> Vec<String> {
//         self.rows.clone()
//     }
//
//     fn delete_grapheme(
//         &mut self,
//         direction: Direction,
//         pos: CursorPosition,
//     ) -> (CursorPosition, Option<char>) {
//         let (col, row) = pos.as_tuple();
//         match direction {
//             Direction::Forward => {
//                 let mut graphemes = std::str::from_utf8(self.row_content(pos))
//                     .expect("Returns a &str")
//                     .to_owned();
//                 let opt_removed_grapheme = graphemes.chars().nth(col).map(|removed_grapheme| {
//                     graphemes.remove(col);
//
//                     self.set_row_content(pos, graphemes).unwrap_or_else(|_| {
//                         unreachable!(
//                             "`set_row_content()` is always expected to update the Buffer after grapheme is deleted"
//                         )
//                     });
//                     removed_grapheme
//                 });
//                 (CursorPosition::new(col, row), opt_removed_grapheme)
//             }
//             Direction::Backward => {
//                 let mut graphemes = std::str::from_utf8(self.row_content(pos))
//                     .expect("Returns a &str")
//                     .to_owned();
//                 let opt_removed_grapheme = graphemes.chars().nth(col.saturating_sub(1)).map(|removed_grapheme| {
//                     graphemes.remove(col.saturating_sub(1));
//
//                     self.set_row_content(pos, graphemes).unwrap_or_else(|_| {
//                         unreachable!(
//                             "`set_row_content()` is always expected to update the buffer after grapheme is deleted"
//                         )
//                     });
//                     self.move_left(pos);
//                     removed_grapheme
//                 });
//                 (
//                     CursorPosition::new(col.saturating_sub(1), row),
//                     opt_removed_grapheme,
//                 )
//             }
//         }
//     }
//
//     // WIP: needs to implement a range
//     fn delete_graphemes(&mut self) -> (CursorPosition, Vec<String>) {
//         unimplemented!()
//     }
//
//     // TODO: Will need policies for movement. Switch back to index grapheme eventually
//     fn insert_grapheme(&mut self, pos: CursorPosition, grapheme: char) -> CursorPosition {
//         let (col, row) = pos.as_tuple();
//         // let index = self.index();
//         // self.rows[row].insert(index, grapheme);
//         self.rows[row].insert(col, grapheme);
//         let new_pos = self.move_right(pos);
//         new_pos
//     }
//
//     fn insert_graphemes<I: Iterator<Item = char>>(
//         &mut self,
//         mut pos: CursorPosition,
//         graphemes: I,
//     ) -> CursorPosition {
//         graphemes.into_iter().for_each(|c| {
//             pos = self.insert_grapheme(pos, c);
//         });
//         pos
//     }
//
//     fn index(&self, pos: CursorPosition) -> usize {
//         let (col, row) = pos.as_tuple();
//         let index = match col {
//             0 => 0,
//             col => {
//                 dbg!(
//                     self.rows[row]
//                         .grapheme_indices(true)
//                         .nth(col)
//                         .expect("invalid position")
//                         .1
//                 );
//                 self.rows[row]
//                     .grapheme_indices(true)
//                     .nth(col)
//                     .expect("invalid position")
//                     .0
//             }
//         };
//         index
//     }
//
//     fn row_content(&self, pos: CursorPosition) -> &[u8] {
//         let (_, row) = pos.as_tuple();
//         self.rows[row].as_bytes()
//     }
//
//     fn set_row_content(
//         &mut self,
//         pos: CursorPosition,
//         data: String,
//     ) -> Result<&mut Self, Self::Error> {
//         let (_col, row) = pos.as_tuple();
//         self.rows[row] = data;
//         Ok(self)
//     }
// }
//
// // How default is derived
// // TODO: consider making row its own type (typestate)
// impl Default for Utf8Buffer {
//     fn default() -> Self {
//         Self {
//             rows: vec![String::new()],
//         }
//     }
// }
//
// impl From<Vec<String>> for Utf8Buffer {
//     fn from(data: Vec<String>) -> Self {
//         let mut buf = Utf8Buffer::new();
//         buf.rows = data;
//         buf
//     }
// }
//
// /*
// // TODO: replace ColumnState & RowState with newtype
// struct Column(usize); // newtype pattern
// impl Column {
//     // "predicate" special name for function that takes no params and returns bool
//     #[inline]
//     pub fn is_beginning_of_line(&self) -> bool {
//         self.0 == 0
//     }
// }
// */
//
// impl View for Utf8Buffer {
//     // Note: Passing a trait constrains type to types that implement the write Trait
//     fn show<W: Write>(&self, writer: &mut W) -> Result<&Self> {
//         // Note: byte smaller than pointer, better to copy than reference(&)
//         writer.write_all(
//             &self
//                 .content()
//                 .iter()
//                 .map(|s| s.as_bytes().iter().map(|b| *b))
//                 .flatten()
//                 .collect::<Vec<u8>>(),
//         )?;
//         Ok(self)
//     }
// }
