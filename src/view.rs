#[cfg(test)]
mod unit_tests;

struct TerminalSpy {}

impl TerminalSpy {
    fn new(width: usize, height: usize) -> Self {
        unimplemented!()
    }
}
struct View {}

impl View {
    fn new(terminalSpy: TerminalSpy) -> Self {
        unimplemented!()
    }
}
