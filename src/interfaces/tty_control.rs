// Handles the actual window screen
// TODO: WindowControl might be better name
pub trait TtyControl {
    fn clear_screen(&mut self) -> Result<(), std::io::Error>;
    fn draw(&mut self);
    fn print(&mut self);
    fn render_frame(&mut self);
    fn resize(&mut self);
    fn set_color(&mut self);
}
