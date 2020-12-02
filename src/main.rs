#![allow(dead_code)]
pub mod buffer;
mod cli_args;
mod error;
pub mod interfaces;
mod position;

use cli_args::CliArgs;
use std::io;
use structopt::StructOpt;
pub use {
    error::{Error, Result},
    position::Position,
};
use crate::{
    buffer::*,
    interfaces::*,
};

// TODO: Read filepath and insert contents into buffer
// Step 2: decouple it in a way that makes sense. Maybe put it into a folder called views
fn main() {
    let filepath = CliArgs::from_args();
    let data = std::fs::read_to_string(filepath);
    let mut buf = Buffer::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Couldn't read line");

        match input.as_str() {
            "#db\n" => {
                buf.delete_glyph(direction::Direction::Backward);
            },
            "#df\n" => {
                buf.delete_glyph(direction::Direction::Forward);
            },
            "#l\n" => {
                buf.move_left();
            },
            "#r\n" => {
                buf.move_right();
            },
            _ => {
                buf.insert_glyphs(input.chars());
            },
        }

        println!("{:?}", buf);
    }
}
