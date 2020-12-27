use crate::position::Position;

pub trait Movement {
    fn move_down(&self) -> Position;
    fn move_left(&self) -> Position;
    fn move_right(&self) -> Position;
    fn move_up(&self) -> Position;
}