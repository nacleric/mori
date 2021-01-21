use crate::position::Position;

pub trait Positionable {
    fn move_down(&self, pos: Position) -> Position;
    fn move_left(&self, pos: Position) -> Position;
    fn move_right(&self, pos: Position) -> Position;
    fn move_up(&self, pos: Position) -> Position;
}
