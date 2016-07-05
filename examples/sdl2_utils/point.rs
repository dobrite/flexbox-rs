
use std::default::Default;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
}
