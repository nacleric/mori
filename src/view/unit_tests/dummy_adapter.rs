use crate::interfaces::TtyControl;

#[derive(Debug, Default)]
pub struct TtyControlDummyAdapter {}

impl TtyControlDummyAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TtyControl for TtyControlDummyAdapter {
    fn clear(&mut self) {}

    fn draw(&mut self) {}

    fn resize(&mut self) {}

    fn set_color(&mut self) {}
}
