pub trait Bufferable {
    fn insert_row(&mut self, pos: Position) -> Position;
    fn delete_row(&mut self, pos: Position) -> Position;
}