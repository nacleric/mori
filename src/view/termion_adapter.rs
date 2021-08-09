use std::io::Write;

use crate::interfaces::TtyControl;
use termion::clear;

use super::Terminal;

#[derive(Debug)]
pub struct TermionAdapter {
    terminal: Terminal,
}

impl TermionAdapter {
    pub fn new(terminal: Terminal) -> Self {
        Self { terminal }
    }

    pub fn set_terminal(&mut self, terminal: Terminal) {
        self.terminal = terminal
    }
}

impl TtyControl for TermionAdapter {
    fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        match write!(self.terminal.output, "{}", clear::All) {
            Ok(_) => self.terminal.output.flush(),
            Err(e) => panic!("Problem clearing the screen: {:?}", e),
        }
    }

    fn draw(&mut self) {
        unimplemented!()
    }

    fn print(&mut self) {
        unimplemented!()
    }

    fn render_frame(&mut self) {
        match write!(self.terminal.output, "{}", self.terminal.position_buffer) {
            Ok(_) => self.terminal.output.flush(),
            Err(e) => panic!("Problem rendering the buffer: {:?}", e),
        }
    }

    fn resize(&mut self) {
        unimplemented!()
    }

    fn set_color(&mut self) {
        unimplemented!()
    }
}
