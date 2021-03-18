use crate::interfaces::Terminal;

#[cfg(test)]
mod unit_tests;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

// Note: Declare an array [T; arr_len] Declare 2darray: [[T; arr_len]; arr_len]
struct MockTerminal {
    data: [[Option<char>; WIDTH]; HEIGHT],
}

impl MockTerminal {
    fn new() -> Self {
        Self {
            data: [[None; WIDTH]; HEIGHT],
        }
    }

    fn content(&self) -> [[Option<char>; WIDTH]; HEIGHT] {
        self.data
    }
}

impl Terminal for MockTerminal {
    fn clear(&mut self) {
        unimplemented!()
    }
}
