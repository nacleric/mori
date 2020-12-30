use crate::{
    buffer::Actions,
    buffer::direction::Direction,
    error::{Error, Result},
    position::{Position, ColumnState, RowState},
};

pub trait MovementPolicy {
    // Calculates valid positions
    fn check_col_state(&self, pos: Position) -> ColumnState;
    fn check_row_state(&self, pos: Position) -> RowState;
    fn check_lower_row(&self, pos: Position) -> bool;
    fn check_upper_row(&self, pos: Position) -> bool;
    fn invalid_actions(&self, col_state: ColumnState, row_state: RowState) -> Vec<Actions>;
}


/* List of cases
backspace BOL goes to EOL of upper row (delete_row) and moves content
(enter) add_row EOL goes to BOL of lower row and moves content
*/