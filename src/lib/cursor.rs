
use std::default::Default;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    pub x: u32,
    pub y: u32,
}

impl Cursor {
    pub fn new(x: u32, y: u32) -> Self {
        Cursor { x: x, y: y }
    }

    pub fn default() -> Self {
        Default::default()
    }
}
