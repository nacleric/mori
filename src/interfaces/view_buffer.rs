use crate::consts::{HEIGHT, WIDTH};

pub trait ViewBuffer {
    fn clear(&mut self);
    fn contents(&self) -> [[Option<char>; WIDTH]; HEIGHT];
}
