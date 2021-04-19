#[cfg(test)]
mod unit_tests;

pub mod mock_terminal;

use crate::{
    interfaces::UIActions,
};

use termion::{
    self,
    raw::{IntoRawMode, RawTerminal},
};

use std::io::Stdout;
use std::io::{stdin, stdout, Write};

use crate::{
    consts::{HEIGHT, WIDTH},
    interfaces::{View, ViewBuffer},
};

// TODO: Will have to be changed
pub struct TerminalBuffer {
    data: [[Option<char>; WIDTH]; HEIGHT],
}

impl ViewBuffer for TerminalBuffer {
    fn clear(&mut self) {
        unimplemented!()
    }
}

pub struct Terminal<B, UI> {
    view: B,
    ui: UI,
    // input:
    output: RawTerminal<Stdout>,
}

impl<B: ViewBuffer, UI: UIActions> Terminal<B, UI>
where
    B: Clone,
{
    pub fn new(view: B, ui: UI) -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        Self {
            view,
            ui,
            output: stdout,
        }
    }

    pub fn view(&self) -> &B {
        let view = &self.view;
        view
    }
}

impl<B: ViewBuffer, UI: UIActions> View for Terminal<B, UI> {
    fn clear(&mut self) {
        write!(self.output, "{}", termion::clear::All).unwrap();
        self.output.flush().unwrap();
    }

    fn print(&mut self) {
        unimplemented!();
    }
}
