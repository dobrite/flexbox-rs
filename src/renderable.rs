
use text;
use view;

#[derive(Debug)]
pub enum Renderable<'r> {
    View(view::View<'r>),
    Text(text::Text<'r>),
}
