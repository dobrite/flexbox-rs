
use rect::Rect;
use rgb::RGB;

#[derive(Debug)]
pub struct Command {
    pub bg: RGB<u8>,
    pub fg: RGB<u8>,
    pub rect: Rect,
}

impl Command {
    pub fn new(bg: RGB<u8>, fg: RGB<u8>, rect: Rect) -> Self {
        Command {
            bg: bg,
            fg: fg,
            rect: rect,
        }
    }

    pub fn fg(&self) -> RGB<u8> {
        self.fg
    }

    pub fn bg(&self) -> RGB<u8> {
        self.bg
    }

    pub fn top(&self) -> i32 {
        self.rect.top
    }

    pub fn left(&self) -> i32 {
        self.rect.left
    }

    pub fn width(&self) -> u32 {
        self.rect.width
    }

    pub fn height(&self) -> u32 {
        self.rect.height
    }
}
