
use std::default::Default;

use super::point::Point;

#[derive(Default)]
pub struct Camera {
    loc: Point,
}

impl Camera {
    pub fn new() -> Self {
        Camera { ..Default::default() }
    }

    pub fn up(&mut self) {
        self.loc.y += 1
    }

    pub fn down(&mut self) {
        self.loc.y -= 1
    }

    pub fn left(&mut self) {
        self.loc.x -= 1
    }

    pub fn right(&mut self) {
        self.loc.x += 1
    }
}
