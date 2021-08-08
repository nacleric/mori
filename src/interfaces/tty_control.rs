// Handles the actual window screen
// TODO: WindowControl might be better name
pub trait TtyControl {
    fn clear(&mut self);
    fn draw(&mut self);
    fn render_frame(&mut self);
    fn resize(&mut self);
    fn set_color(&mut self);
}
