
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

        cursor.cascade_style(&r.get_style());

        let (command, mut children) = match r {
            // TODO can we destructure view and children here?
            &Renderable::View(ref view) => {
                let mut child_cursor = cursor;
                child_cursor.flex_direction = view.style.flex_direction;
                child_cursor.width = 0;
                let mut children = vec![];
                for child in &view.children {
                    let (ref mut commands, nc) = self.recurse(child, child_cursor);

                    child_cursor.height = 0;
                    child_cursor.width = 0;

                    // move child_cursor to get ready to render the next child
                    //
                    // adjust cursor width/height as we  render children so that
                    // it has dimensions needed to enclose the children
                    if child_cursor.flex_direction == style::FlexDirection::Row {
                        child_cursor.x += nc.width;
                        cursor.width += nc.width;
                        cursor.height = nc.height;
                    } else {
                        child_cursor.y += nc.height;
                        cursor.width = nc.width;
                    }

                    children.append(commands);
                }

                // now that children are laid out; we know parent's dimensions and can push it on
                let command = {
                    // if parent has declared dimensions, use them, otherwise use the dimensions
                    // forced by children
                    let width = view.style.width.unwrap_or(cursor.width);
                    let height = view.style.height.unwrap_or(cursor.height);

                    let (x, y) = if view.style.position == style::Position::Fixed {
                        // fixed position is relative to the whole screen rather than parent.
                        // since fixed can be declared anywhere along the tree we have to
                        // continually check and deal with it.

                        // TODO When both top and bottom are specified, as long as height is
                        // unspecified, auto or 100%, both top and bottom distances will be
                        // respected. Otherwise, if height is constrained in any way, the top
                        // property takes precedence and the bottom property is ignored.
                        let x = view.style.left.unwrap_or(view.style
                            .right
                            .map_or(0, |right| cursor.root_width as i32 - right - width as i32));
                        let y = view.style.top.unwrap_or(view.style
                            .bottom
                            .map_or(0, |bot| cursor.root_height as i32 - bot - height as i32));
                        (x, y)
                    } else {
                        // offset is how far scrolled the parent is
                        (cursor.x as i32 + cursor.offset_x, cursor.y as i32 + cursor.offset_y)
                    };

                    let rect = Rect::new(x, y, width, height);
                    let bg = cursor.compute_bg(view.style.bg);
                    let fg = view.style.fg.unwrap_or(cursor.fg);
                    Command::new(bg, fg, None, rect)
                };

                if view.style.position == style::Position::Static {
                    if cursor.flex_direction == style::FlexDirection::Row {
                        cursor.width = view.style.width.unwrap_or(cursor.width);
                    } else {
                        cursor.height = view.style.height.unwrap_or(cursor.height);
                    }
                }

                (command, children)
            }
            // TODO can we destructure view and children here?
            &Renderable::Text(ref text) => {
                let measure::Dim { width, height } = self.measure.get_dim(text.children);

                let command = {
                    let x = cursor.x as i32;
                    let y = cursor.y as i32;
                    let rect = Rect::new(x, y, width, height);
                    let bg = cursor.compute_bg(text.style.bg);
                    let fg = text.style.fg.unwrap_or(cursor.fg);
                    Command::new(bg, fg, Some(text.children), rect)
                };

                cursor.width = width;
                cursor.height = height;

                (command, vec![])
            }
        };

        v.push(command);
        v.append(&mut children);

        (v, cursor)
    }
}
