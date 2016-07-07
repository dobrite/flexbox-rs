
use std::default::Default;

use flexbox;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }

    pub fn into_offset(self) -> flexbox::Offset {
        flexbox::Offset::new(self.x, self.y)
    }
}
