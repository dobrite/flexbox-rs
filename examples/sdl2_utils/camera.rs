
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
}
