pub trait TtyControl {
    fn clear(&mut self);
    fn draw(&mut self);
    fn resize(&mut self);
    fn set_color(&mut self);
}
