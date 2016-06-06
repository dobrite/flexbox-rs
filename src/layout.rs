
use command::Command;
use cursor::Cursor;
use measure;
use rect::Rect;
use renderable::Renderable;
use style;

/// all views come with `display: flex` by default.

/// width, height reps div child inside body with w/h set.
/// i.e.
/// <body>
///   <div style="width: 800px; height: 600px;">
///   </div>
/// </body>

pub struct Layout<'m> {
    measure: &'m measure::Measure,
}

impl<'m, 'r> Layout<'m> {
    pub fn new(measure: &'m measure::Measure) -> Self {
        Layout { measure: measure }
    }

    pub fn layout(&self, width: u32, height: u32, r: &Renderable<'r>) -> Vec<Command> {
        self.recurse(r, Cursor::new(width, height)).0
    }

    // TODO some sort of From or Into would be nice to not have to wrap everythign in the enum
    // TODO pass vec down and back up. right now we're allocating a bunch
    fn recurse(&self, r: &Renderable<'r>, mut cursor: Cursor) -> (Vec<Command>, Cursor) {
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
                    let (ref mut ls, nc) = self.recurse(child, parent_cursor);

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
            &Renderable::Text(ref text) => {
                let measure::Dim { width, height } = self.measure.get_dim(text.children);
                v.push(Command::new(text.style.bg.unwrap_or(cursor.bg),
                                    text.style.fg.unwrap_or(cursor.fg),
                                    Rect::new(cursor.x as i32, cursor.y as i32, width, height)));
            }
        }

        (v, cursor)
    }
}
