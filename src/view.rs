#[cfg(test)]
mod unit_tests;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

struct MockTerminal {
    data: [[None; WIDTH]; HEIGHT],
}

impl MockTerminal {
    fn new() -> Self {
        Self {
            data: [[None; WIDTH]; HEIGHT],
        }
    }
}

struct Terminal {}

impl Terminal {
    fn new(mockterminal: MockTerminal) -> Self {
        unimplemented!()
    }
}
