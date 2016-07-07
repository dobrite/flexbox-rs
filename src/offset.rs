
#[derive(Clone, Copy, Debug, Default)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Offset {
    pub fn new(x: i32, y: i32) -> Self {
        Offset { x: x, y: y }
    }
}
