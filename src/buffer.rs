pub mod direction;
#[cfg(test)]
mod unit_tests;
use crate::interfaces::View;
use crate::{
    error::{Error, Result},
    interfaces::GlyphBuffer,
    position::Position,
};
use direction::Direction;
use std::io::Write;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    data: String,
    pos: Position,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: String::default(),
            pos: Position::default(),
        }
    }

    /// Removes data from the buffer but does not remove the entire buffer
    pub fn delete_glyphs(&mut self) -> &mut Self {
        self.data.clear();
        self
    }

    pub fn insert_glyphs<I: Iterator<Item = char>>(&mut self, glyphs: I) -> &mut Self {
        glyphs.into_iter().for_each(|c| {
            self.insert_glyph(c);
        });
        self
    }
}

impl GlyphBuffer for Buffer {
    type Error = Error;

    fn contents(&self) -> &[u8] {
        self.data.as_bytes()
    }

    // Semantically guarantees something gets deleted but if theres nothing to delete than innacurate name
    fn delete_glyph(&mut self, direction: Direction) -> Option<char> {
        let cursor_position = self.pos().col();
        let glyph = match direction {
            Direction::Forward => {
                let mut glyphs = std::str::from_utf8(self.contents())
                    .expect("Returns a &str")
                    .to_owned();
                let removed_glyph = glyphs.chars().nth(cursor_position)?;
                glyphs.remove(cursor_position);

                self.set_contents(glyphs).unwrap_or_else(|_| {
                    unreachable!(
                        "`set_contents()` is always expected to update the buffer after glyph is deleted"
                    )
                });
                // Note: Only move_left() if at the end of line (will probably be in a policy function)
                self.move_left().unwrap_or_else(|| {
                    unreachable!(
                        "`move_left()` expected to always suceed immediately following `delete()`."
                    )
                });
                removed_glyph
            }
            Direction::Backward => {
                let mut glyphs = std::str::from_utf8(self.contents())
                    .expect("Returns a &str")
                    .to_owned();
                let removed_glyph = glyphs.chars().nth(cursor_position.saturating_sub(1))?;
                glyphs.remove(cursor_position.saturating_sub(1));

                self.set_contents(glyphs).unwrap_or_else(|_| {
                    unreachable!(
                        "`set_contents()` is always expected to update the buffer after glyph is deleted"
                    )
                });
                if cursor_position != 0 {
                    // Note: If at beginning of line *don't* move left (will probably be in a policy function)
                    self.move_left().unwrap_or_else(|| {
                        unreachable!(
                            "`move_left()` expected to always succeed immediately following `delete()`."
                        )
                    });
                }
                removed_glyph
            }
        };
        Some(glyph)
    }

    fn insert_glyph(&mut self, glyph: char) -> &mut Self {
        // TODO: From Rust traceback
        self.data.insert(self.index(), glyph);
        self.move_right().unwrap_or_else(|| {
            unreachable!(
                "`move_right()` expected to always succeed immediately following an `insert()`."
            )
        });
        self
    }

    fn move_down(&mut self) -> Option<&mut Self> {
        self.pos = Position::new(self.pos().col(), self.pos().row().saturating_add(1));
        Some(self)
    }

    fn move_left(&mut self) -> Option<&mut Self> {
        self.pos = Position::new(self.pos().col().saturating_sub(1), self.pos().row());
        Some(self)
    }

    fn move_right(&mut self) -> Option<&mut Self> {
        self.pos = Position::new(self.pos().col().saturating_add(1), self.pos().row());
        Some(self)
    }

    fn move_up(&mut self) -> Option<&mut Self> {
        self.pos = Position::new(self.pos().col(), self.pos().row().saturating_sub(1));
        Some(self)
    }

    fn pos(&self) -> Position {
        self.pos
    }

    // TODO: WIP needs to account for rows and columns. Vec of String
    fn index(&self) -> usize {
        match self.pos().col() {
            0 => 0,
            pos => self
                .data[self.pos().row()]
                .grapheme_indices(true)
                .nth(self.pos().col() - 1)
                .expect("Invalid position") // usize is index &str utf8 representation of the char
                .0,
        }
    }

    fn set_contents(&mut self, data: String) -> Result<&mut Self, Self::Error> {
        self.data = data;
        Ok(self)
    }

    // TODO: Currently only worries about a single line, no concept of verticality yet
    fn set_pos(&mut self, pos: Position) -> Result<&mut Self, Self::Error> {
        let content_length = self.contents().len();

        match pos.col() {
            col if col <= content_length => {
                self.pos = pos;
                Ok(self)
            }
            _ => Err(Error::InvalidPosition(pos)),
        }
    }
}

impl View for Buffer {
    // Note: Constrains type to types that implement the write Trait
    fn show<W: Write>(&self, writer: &mut W) -> Result<&Self> {
        writer.write_all(self.contents())?;
        Ok(self)
    }
}
