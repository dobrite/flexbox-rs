
use cursor::Cursor;
use rect::Rect;
use renderable::Renderable;
use rgb::RGB;
use style;

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

/// all views come with `display: flex` by default.

/// width, height reps div child inside body with w/h set.
/// i.e.
/// <body>
///   <div style="width: 800px; height: 600px;">
///   </div>
/// </body>

pub fn layout<'r>(width: u32, height: u32, r: &Renderable<'r>) -> Vec<Command> {
    recurse(r, Cursor::new(width, height)).0
}

// TODO some sort of From or Into would be nice to not have to wrap everythign in the enum
// TODO pass vec down and back up. right now we're allocating a bunch
fn recurse<'r>(r: &Renderable<'r>, mut cursor: Cursor) -> (Vec<Command>, Cursor) {
    let mut v = vec![];

    match r {
        &Renderable::View(ref view) => {
            v.push(Command::new(view.style.bg.unwrap_or(cursor.bg),
                                view.style.fg.unwrap_or(cursor.fg),
                                Rect::new(cursor.x as i32,
                                          cursor.y as i32,
                                          view.style.width.unwrap_or(cursor.width),
                                          view.style.height.unwrap_or(cursor.height))));

            let mut parent_cursor = cursor;
            parent_cursor.flex_direction = view.style.flex_direction;
            for child in &view.children {
                parent_cursor.width = 0;
                let (ref mut ls, nc) = recurse(child, parent_cursor);

                if parent_cursor.flex_direction == style::FlexDirection::Row {
                    parent_cursor.x += nc.x;
                } else {
                    parent_cursor.y += nc.y;
                }

                v.append(ls);
            }

            if cursor.flex_direction == style::FlexDirection::Row {
                cursor.x = view.style.width.unwrap_or(cursor.width);
            } else {
                cursor.y = view.style.height.unwrap_or(cursor.height);
            }
        }
        &Renderable::Text(_) => {
            println!("text");
        }
    }

    (v, cursor)
}
