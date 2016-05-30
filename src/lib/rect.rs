
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub top: i32,
    pub left: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(top: i32, left: i32, width: u32, height: u32) -> Self {
        Rect {
            top: top,
            left: left,
            width: width,
            height: height,
        }
    }
}
