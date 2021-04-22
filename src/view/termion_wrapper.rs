use crate::interfaces::UIActions;

#[derive(Debug, Default)]
pub struct TermionWrapper {}

impl TermionWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}

impl UIActions for TermionWrapper {
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
