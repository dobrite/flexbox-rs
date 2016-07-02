
use command::Command;
use cursor::Cursor;
use measure;
use rect::Rect;
use renderable::Renderable;
use root;
use style;

pub struct Layout<'m> {
    measure: &'m measure::Measure,
}

impl<'m, 'r> Layout<'m> {
    pub fn new(measure: &'m measure::Measure) -> Self {
        Layout { measure: measure }
    }

    pub fn layout(&'r self, root: &root::Root<'r>) -> Vec<Command> {
        let cursor = Cursor::new(root.width, root.height);
        self.recurse(root.root(), cursor).0
    }

    // TODO some sort of From or Into would be nice to not have to wrap everythign in the enum
    // TODO pass vec down and back up. right now we're allocating a bunch
    // TODO background-color is not inherited -- check if this is true
    // TODO color IS inherited
    fn recurse(&'r self, r: &Renderable<'r>, mut cursor: Cursor) -> (Vec<Command>, Cursor) {
        let mut v = vec![];

        match r {
            &Renderable::View(ref view) => {
                cursor.cascade_style(&view.style);

                {
                    let x = cursor.x as i32;
                    let y = cursor.y as i32;
                    let rect = Rect::new(x,
                                         y,
                                         view.style.width.unwrap_or(cursor.width),
                                         view.style.height.unwrap_or(cursor.height));
                    let command = Command::new(cursor.compute_bg(view.style.bg),
                                               view.style.fg.unwrap_or(cursor.fg),
                                               None,
                                               rect);
                    v.push(command);
                }

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
                cursor.cascade_style(&text.style);
                let measure::Dim { width, height } = self.measure.get_dim(text.children);
                {
                    let x = cursor.x as i32;
                    let y = cursor.y as i32;
                    let rect = Rect::new(x, y, width, height);
                    let command = Command::new(cursor.compute_bg(text.style.bg),
                                               text.style.fg.unwrap_or(cursor.fg),
                                               Some(text.children),
                                               rect);
                    v.push(command);
                }
                cursor.x += width;
                cursor.y += height;
            }
        }

        (v, cursor)
    }
}
