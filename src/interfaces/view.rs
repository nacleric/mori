pub trait View {
    fn clear(&mut self) -> Result<(), std::io::Error>;
    fn print(&mut self);
}
