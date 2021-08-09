use crate::{interfaces::TtyControl, view::Terminal};

#[derive(Debug)]
pub struct TtyControlDummyAdapter {
    terminal: Terminal,
}

impl TtyControlDummyAdapter {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::default(),
        }
    }
}

impl TtyControl for TtyControlDummyAdapter {
    fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        unimplemented!();
    }

    fn draw(&mut self) {}

    fn print(&mut self) {}

    fn render_frame(&mut self) {}
    
    fn resize(&mut self) {}

    fn set_color(&mut self) {}
}

fn test_tty_commands<T: TtyControl>(display: &mut T) {
    display.clear_screen();
    display.draw();
    display.print();
    display.render_frame();
    display.resize();
    display.set_color();
}

// Test is kinda useless for now, mostly wanted to see how the adapter would work
#[test]
fn dummy_adapter_allows_terminal_access_to_tty_interface() {
    // Given
    let terminal = &mut TtyControlDummyAdapter::new();
    let expected_res = test_tty_commands(terminal);
    let sut = terminal;

    // When
    let res = test_tty_commands(sut);
    
    // Then
    assert_eq!(res, expected_res);
}