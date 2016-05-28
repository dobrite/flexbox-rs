
use data::PixelLocOffset;

pub struct Layout {
    offset: PixelLocOffset,
}

impl Layout {
    pub fn new(offset: PixelLocOffset) -> Self {
        Layout { offset: offset }
    }

    pub fn get_offset(&self) -> &PixelLocOffset {
        &self.offset
    }
}
