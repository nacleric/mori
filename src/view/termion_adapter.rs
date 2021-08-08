use termion::clear;
use crate::interfaces::TtyControl;

use super::Terminal;

#[derive(Debug)]
pub struct TermionAdapter {
    terminal: Terminal,
}

impl TermionAdapter {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(),
        }
    }
}

impl TtyControl for TermionAdapter {
    fn clear(&mut self) {
        println!("{}", clear::All);
    }

    fn draw(&mut self) {
        unimplemented!()
    }

    fn render_frame(&mut self) {
        unimplemented!()    
    }

    fn resize(&mut self) {
        unimplemented!()
    }

    fn set_color(&mut self) {
        unimplemented!()
    }
}
