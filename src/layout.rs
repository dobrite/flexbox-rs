
use cursor::Cursor;
use rect::Rect;
use renderable::Renderable;
use rgb::RGB;

// TODO wrap these or abtract them
// TODO keep these pub or do some chaining builder thingy?
#[derive(Debug)]
pub struct Layout {
    pub bg: RGB<u8>,
    pub fg: RGB<u8>,
    pub rect: Rect,
}

pub fn layout<'a>(width: u32, height: u32, r: &Renderable<'a>) -> Vec<Layout> {
    recurse(r, Cursor::new(width, height)).0
}

// TODO some sort of From or Into would be nice to not have to wrap everythign in the enum
fn recurse<'a>(r: &Renderable<'a>, cursor: Cursor) -> (Vec<Layout>, Cursor) {
    let mut v = vec![];
    let mut new_cursor = cursor;

    match r {
        &Renderable::View(ref view) => {
            v.push(Layout {
                bg: view.style.bg.unwrap_or(cursor.bg),
                fg: view.style.fg.unwrap_or(cursor.fg),
                rect: Rect::new(cursor.x as i32,
                                cursor.y as i32,
                                view.style.width.unwrap_or(cursor.width),
                                view.style.height.unwrap_or(0)),
            });

            for child in view.children.iter() {
                let (ref mut l, _) = recurse(child, new_cursor);
                // TODO this is `row`, do `column`
                new_cursor.x += match child {
                    &Renderable::View(ref view) => view.style.width.unwrap(), // TODO ugh
                    &Renderable::Text(_) => 0u32, // TODO
                };

                v.append(l);
            }
        }
        &Renderable::Text(_) => {
            println!("text");
        }
    }

    (v, new_cursor)
}

// let root = Renderable::View(View::new(Style::new().with_width(width).with_height(height),
//    vec![
//        Renderable::View(View::new(Style::new().with_width(50)
//          .with_height(100).with_bg(Color::RGB(255, 0, 0)), vec![])),
//        Renderable::View(View::new(Style::new().with_width(50)
//          .with_height(100).with_bg(Color::RGB(0, 255, 0)), vec![])),
//    ]
// ));

// [
//   Layout { bg: Color::RGB(0, 0, 0), Rect::new(0, 0, 800, 600) },
//   Layout { bg: Color::RGB(255, 0, 0), Rect::new(0, 0, 50, 100) },
//   Layout { bg: Color::RGB(0, 255, 0), Rect::new(50, 0, 50, 100) },
// ]
