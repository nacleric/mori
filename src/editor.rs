extern crate termion;

use std::io::{stdin, stdout, Write};
use std::fs;
// use std::path::Path;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Might not be able to use String
#[allow(dead_code)]
pub fn read_file(file: &str) -> String{
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    contents
}

#[allow(dead_code)]
pub fn render_file(contents: String) {
    // let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    // stdout.write(contents.as_bytes());
    // stdout.flush().unwrap();
    writeln!(stdout, "{}", contents).unwrap();

}

pub fn example() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
