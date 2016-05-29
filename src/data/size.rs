/// a generic struct for something with width and height
/// can be any units i.e. not just "tiles"
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]

// TODO get rid of this if it isn't used

pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Size {
            width: width,
            height: height,
        }
    }

    pub fn perimeter(&self) -> i32 {
        self.width * 2 + self.height * 2 - 4
    }

    pub fn len(&self) -> i32 {
        self.width * self.height
    }
}
