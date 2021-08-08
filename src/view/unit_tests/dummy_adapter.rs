use crate::{interfaces::TtyControl, view::Terminal};

#[derive(Debug)]
pub struct TtyControlDummyAdapter {
    terminal: Terminal,
}

impl TtyControlDummyAdapter {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(),
        }
    }
}

impl TtyControl for TtyControlDummyAdapter {
    fn clear(&mut self) {}

    fn draw(&mut self) {}

    fn render_frame(&mut self) {}
    
    fn resize(&mut self) {}

    fn set_color(&mut self) {}
}

fn test_tty_commands<T: TtyControl>(terminal: &mut T) {
    terminal.clear();
    terminal.draw();
    terminal.render_frame();
    terminal.resize();
    terminal.set_color();
}

// Test is kinda useless for now, mostly wanted to see how the adapter would work
// Maybe doesn't need &mut?
#[test]
fn dummy_adapter_allows_terminal_access_to_tty_interface() {
    // Given
    let dummy_terminal = Terminal::new();
    let dummy_tty_adapter = &mut TtyControlDummyAdapter {
        terminal: dummy_terminal
    };
    let expected_res = test_tty_commands(dummy_tty_adapter);
    let sut = dummy_tty_adapter;

    // When
    let res = test_tty_commands(sut);
    
    // Then
    assert_eq!(res, expected_res);
}