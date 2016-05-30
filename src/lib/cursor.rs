
use std::default::Default;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    pub x: u32,
    pub y: u32,
}

impl Cursor {
    pub fn default() -> Self {
        Default::default()
    }
}
