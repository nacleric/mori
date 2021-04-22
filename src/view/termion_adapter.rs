use crate::interfaces::TtyControl;

#[derive(Debug, Default)]
pub struct TermionAdapter {}

impl TermionAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TtyControl for TermionAdapter {
    fn clear(&mut self) {
        unimplemented!()
    }

    fn draw(&mut self) {
        unimplemented!()
    }

    fn resize(&mut self) {
        unimplemented!()
    }

    fn set_color(&mut self) {
        unimplemented!()
    }
}
