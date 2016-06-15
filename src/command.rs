
use rect::Rect;
use rgb::RGB;

#[derive(Debug)]
pub struct Command<'s> {
    // None for Transparent
    pub bg: Option<RGB<u8>>,
    pub fg: RGB<u8>,
    pub text: Option<&'s str>,
    pub rect: Rect,
}

// TODO rgba(0, 0, 0, 0) for transparent ?
impl<'s> Command<'s> {
    pub fn new(bg: Option<RGB<u8>>, fg: RGB<u8>, text: Option<&'s str>, rect: Rect) -> Self {
        Command {
            bg: bg,
            fg: fg,
            text: text,
            rect: rect,
        }
    }

    pub fn fg(&self) -> RGB<u8> {
        self.fg
    }

    pub fn bg(&self) -> Option<RGB<u8>> {
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
