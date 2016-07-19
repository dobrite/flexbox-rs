
use command::Command;
use cursor::Cursor;
use measure;
use offset;
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

    pub fn layout(&'r self, root: &root::Root<'r>, offset: offset::Offset) -> Vec<Command> {
        let cursor = Cursor::new(root.width, root.height, offset);
        self.recurse(root.root(), cursor).0
    }

    // TODO some sort of From or Into would be nice to not have to wrap everythign in the enum
    // TODO pass vec down and back up. right now we're allocating a bunch
    // TODO background-color is not inherited -- check if this is true
    // TODO color IS inherited
    fn recurse(&'r self, r: &Renderable<'r>, mut cursor: Cursor) -> (Vec<Command>, Cursor) {
        let mut v = vec![];

        match r {
            // TODO can we destructure view and children here?
            &Renderable::View(ref view) => {
                cursor.cascade_style(&view.style);

                let mut parent_cursor = cursor;
                parent_cursor.flex_direction = view.style.flex_direction;
                let mut children = vec![];
                for child in &view.children {
                    parent_cursor.width = 0;
                    let (ref mut commands, nc) = self.recurse(child, parent_cursor);

                    if parent_cursor.flex_direction == style::FlexDirection::Row {
                        parent_cursor.x = nc.x;
                    } else {
                        parent_cursor.y = nc.y;
                    }

                    children.append(commands);
                }

                {
                    let width = view.style.width.unwrap_or(cursor.width);
                    let height = view.style.height.unwrap_or(cursor.height);
                    let (x, y) = if view.style.position == style::Position::Fixed {
                        // TODO When both top and bottom are specified, as long as height is
                        // unspecified, auto or 100%, both top and bottom distances will be
                        // respected. Otherwise, if height is constrained in any way, the top
                        // property takes precedence and the bottom property is ignored.
                        let x = view.style.left.unwrap_or(view.style
                            .right
                            .map_or(0, |right| cursor.root_width as i32 - right - width as i32));
                        let y = view.style.top.unwrap_or(view.style
                            .bottom
                            .map_or(0,
                                    |bottom| cursor.root_height as i32 - bottom - height as i32));
                        (x, y)
                    } else {
                        (cursor.x as i32 + cursor.offset_x, cursor.y as i32 + cursor.offset_y)
                    };
                    let rect = Rect::new(x, y, width, height);
                    let bg = cursor.compute_bg(view.style.bg);
                    let fg = view.style.fg.unwrap_or(cursor.fg);
                    let command = Command::new(bg, fg, None, rect);

                    v.push(command);
                }

                v.append(&mut children);

                if view.style.position == style::Position::Static {
                    if cursor.flex_direction == style::FlexDirection::Row {
                        cursor.x += view.style.width.unwrap_or(cursor.width);
                    } else {
                        cursor.y += view.style.height.unwrap_or(cursor.height);
                    }
                }
            }
            // TODO can we destructure view and children here?
            &Renderable::Text(ref text) => {
                cursor.cascade_style(&text.style);
                let measure::Dim { width, height } = self.measure.get_dim(text.children);
                {
                    let x = cursor.x as i32;
                    let y = cursor.y as i32;
                    let rect = Rect::new(x, y, width, height);
                    let bg = cursor.compute_bg(text.style.bg);
                    let fg = text.style.fg.unwrap_or(cursor.fg);
                    let command = Command::new(bg, fg, Some(text.children), rect);
                    v.push(command);
                }
                cursor.x += width;
                cursor.y += height;
            }
        }

        (v, cursor)
    }
}
