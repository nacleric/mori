pub mod direction;
#[cfg(test)]
mod unit_tests;
use crate::{
    error::{Error, Result},
    interfaces::GlyphBuffer,
    position::Position,
};
use direction::Direction;

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

    // TODO: Add policies to this
    // Note: Move
    // Semantically guarantees something gets deleted but if theres nothing to delete than innacurate name
    fn delete_glyph(&mut self, direction: Direction) -> Option<char> {
        //  hello
        //  12345
        let cursor_position= self.pos().col();
        let glyph = match direction {
            Direction::Forward => {
                let mut glyphs = std::str::from_utf8(self.contents()).expect("Returns a &str").to_owned();
                let removed_glyph = glyphs.chars().nth(cursor_position).unwrap();
                glyphs.remove(cursor_position);

                // Note: Only move_left() if at the end of line (will probably be in a policy function)
                self.move_left().unwrap_or_else(|| {
                    unreachable!(
                        "`move_left()` expected to always suceed immediately following `delete()`."
                    )
                });

                removed_glyph
            }
            Direction::Backward => {
                let mut glyphs = std::str::from_utf8(self.contents()).expect("Returns a &str").to_owned();
                let removed_glyph = glyphs.chars().nth(cursor_position - 1).unwrap();
                glyphs.remove(cursor_position - 1);

                if cursor_position != 0 {
                    // Note: If at beginning of line *don't* move left (will probably be in a policy function)
                    self.move_left().unwrap_or_else(|| {
                        unreachable!(
                            "`move_left()` expected to always suceed immediately following `delete()`."
                        )
                    });
                }
                removed_glyph
            }
        };
        Some(glyph)
    }

    // TODO: Add policies to this
    fn insert_glyph(&mut self, glyph: char) -> &mut Self {
        self.data.insert(self.pos().col(), glyph);
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
        // TODO: add saturating_sub
        self.pos = Position::new(self.pos().col() - 1, self.pos().row());
        Some(self)
    }

    fn move_right(&mut self) -> Option<&mut Self> {
        // TODO: add saturating_add
        self.pos = Position::new(self.pos().col() + 1, self.pos().row());
        Some(self)
    }

    fn move_up(&mut self) -> Option<&mut Self> {
        self.pos = Position::new(self.pos().col(), self.pos().row().saturating_sub(1));
        Some(self)
    }

    fn pos(&self) -> Position {
        self.pos
    }

    fn set_contents(&mut self, content: &[u8]) -> Result<&mut Self, Self::Error> {
        let data = std::str::from_utf8(content).expect("utf8 error").to_owned();
        self.data = data;
        Ok(self)
    }

    // TODO: Currently only worries about a single line, no concept of verticality yet
    fn set_pos(&mut self, pos: Position) -> Result<&mut Self, Self::Error> {
        let content_length = self.contents().len();

        let pos = match pos.col() {
            _col if _col < content_length => {
                self.pos = pos;
                Ok(self)
            }
            _ => Err(Error::InvalidPosition(pos)),
        };

        pos
    }
}

// TODO: Change the name of this later
struct BigBuffer {
    lines: Vec<Buffer>,
    pos: Position,
}

impl BigBuffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            pos: Position::default(),
        }
    }
}
