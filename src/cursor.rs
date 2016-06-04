
use std::default::Default;

use rgb;
use style;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub fg: rgb::RGB<u8>,
    pub bg: rgb::RGB<u8>,
    pub flex_direction: style::FlexDirection,
}

impl Cursor {
    pub fn new(width: u32, height: u32) -> Self {
        Cursor {
            bg: rgb::RGB::new(255, 255, 255),
            width: width,
            height: height,
            ..Default::default()
        }
    }
}
