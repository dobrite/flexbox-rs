
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(left: i32, top: i32, width: u32, height: u32) -> Self {
        Rect {
            left: left,
            top: top,
            width: width,
            height: height,
        }
    }
}
