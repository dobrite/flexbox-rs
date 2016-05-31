
use std::default::Default;

use rgb::RGB;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub fg: RGB<u8>,
    pub bg: RGB<u8>,
}

impl Cursor {
    pub fn new(width: u32, height: u32) -> Self {
        Cursor {
            bg: RGB::new(255, 255, 255),
            width: width,
            height: height,
            ..Default::default()
        }
    }
}
