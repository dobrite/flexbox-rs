
use cursor::Cursor;
use rect::Rect;
use renderable::Renderable;
use rgb::RGB;

#[derive(Debug)]
pub struct Layout {
    pub bg: RGB<u8>,
    pub fg: RGB<u8>,
    pub rect: Rect,
}

impl Layout {
    pub fn new(bg: RGB<u8>, fg: RGB<u8>, rect: Rect) -> Self {
        Layout {
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

/// all views come with `display: flex` by default.

/// width, height reps div child inside body with w/h set.
/// i.e.
/// <body>
///   <div style="width: 800px; height: 600px;">
///   </div>
/// </body>

pub fn layout<'a>(width: u32, height: u32, r: &Renderable<'a>) -> Vec<Layout> {
    recurse(r, Cursor::new(width, height)).0
}

// TODO some sort of From or Into would be nice to not have to wrap everythign in the enum
fn recurse<'a>(r: &Renderable<'a>, mut cursor: Cursor) -> (Vec<Layout>, Cursor) {
    let mut v = vec![];

    match r {
        &Renderable::View(ref view) => {
            v.push(Layout::new(view.style.bg.unwrap_or(cursor.bg),
                               view.style.fg.unwrap_or(cursor.fg),
                               Rect::new(cursor.x as i32,
                                         cursor.y as i32,
                                         view.style.width.unwrap_or(cursor.width),
                                         view.style.height.unwrap_or(cursor.height))));

            let mut new_cursor = cursor;
            for child in &view.children {
                new_cursor.width = 0;
                let (ref mut ls, nc) = recurse(child, new_cursor);
                new_cursor.x += nc.x;
                v.append(ls);
            }

            // this happen in row (default)
            cursor.x = view.style.width.unwrap_or(cursor.width);
            // this happens in column
            // cursor.y = view.style.height.unwrap_or(cursor.height);
        }
        &Renderable::Text(_) => {
            println!("text");
        }
    }

    (v, cursor)
}
