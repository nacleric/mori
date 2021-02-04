pub trait Position {
    fn move_down(&mut self) -> &mut Self;
    fn move_left(&self);
    fn move_right(&self);
    fn move_up(&self);
}
