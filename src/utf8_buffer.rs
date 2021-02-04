#[cfg(test)]
mod unit_tests;

pub mod direction;

use crate::{consts::*, interfaces::Buffer};
use non_empty_vec::NonEmpty;

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

    fn col_count(&self, row_index: usize) -> usize {
        self.rows[row_index].len()
    }

    fn delete_row(&mut self, row_index: usize) -> Option<Self::Row> {
        let recorded_string = self.rows[row_index].clone();
        let new_rows = {
            let mut tmp_vec = self.rows.to_vec();
            tmp_vec.remove(row_index);
            tmp_vec
        };
        self.rows = NonEmpty::new(new_rows)
            .unwrap_or_else(|| unreachable!(ERR_INTERNAL_NON_EMPTY_VEC_REQUIRED));

        Some(recorded_string)
    }

    fn edit_row(&mut self, row_index: usize) -> Option<&mut Self::Row> {
        Some(&mut self.rows[row_index])
    }

    fn insert_row(&mut self, row_index: usize) -> &mut Self {
        let new_rows = {
            let mut tmp_vec = self.rows.to_vec();
            tmp_vec.insert(row_index, String::new());
            tmp_vec
        };
        self.rows = NonEmpty::new(new_rows)
            .unwrap_or_else(|| unreachable!(ERR_INTERNAL_NON_EMPTY_VEC_REQUIRED));
        self
    }

    fn row_count(&self) -> usize {
        self.rows.len().into()
    }
}

impl Default for Utf8Buffer {
    fn default() -> Self {
        Self {
            rows: NonEmpty::new(vec![String::new()])
                .unwrap_or_else(|| unreachable!(ERR_INTERNAL_NON_EMPTY_VEC_REQUIRED)),
        }
    }
}

impl From<Vec<String>> for Utf8Buffer {
    fn from(data: Vec<String>) -> Self {
        let mut buf = Utf8Buffer::new();
        buf.rows = NonEmpty::new(data)
            .unwrap_or_else(|| unreachable!(ERR_INTERNAL_NON_EMPTY_VEC_REQUIRED));
        buf
    }
}
