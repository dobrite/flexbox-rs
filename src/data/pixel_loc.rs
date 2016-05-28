/// pixel based window location
///
/// e.g. `PixelLoc { x: 0, y: 0 }` is always the upper left pixel of the screen

use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default)]
pub struct PixelLoc {
    pub x: i32,
    pub y: i32,
}

impl PixelLoc {
    pub fn offset(self, other: &PixelLoc) -> Self {
        self +
        PixelLoc {
            x: other.x,
            y: other.y,
        }
    }
}

impl Add for PixelLoc {
    type Output = PixelLoc;

    fn add(self, other: PixelLoc) -> Self::Output {
        PixelLoc {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for PixelLoc {
    type Output = PixelLoc;

    fn sub(self, other: PixelLoc) -> Self::Output {
        PixelLoc {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PixelLocOffset {
    pub x: i32,
    pub y: i32,
}
