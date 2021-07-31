use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::ViewBuffer,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MockTerminalView {
    data: [[Option<char>; WIDTH]; HEIGHT],
}

impl MockTerminalView {
    pub fn new() -> Self {
        Self {
            data: [[None; WIDTH]; HEIGHT],
        }
    }

    pub fn get_data(&self) -> [[Option<char>; WIDTH]; HEIGHT] {
        self.data
    }

    pub fn set_data(&mut self, data: [[Option<char>; WIDTH]; HEIGHT]) {
        self.data = data;
    }
}
