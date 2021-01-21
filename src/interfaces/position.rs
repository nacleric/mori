use crate::cursor_position::CursorPosition;

pub trait Position {
    fn move_down(&self, pos: CursorPosition) -> CursorPosition;
    fn move_left(&self, pos: CursorPosition) -> CursorPosition;
    fn move_right(&self, pos: CursorPosition) -> CursorPosition;
    fn move_up(&self, pos: CursorPosition) -> CursorPosition;
}
