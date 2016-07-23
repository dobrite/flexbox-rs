
use style;
use text;
use view;

#[derive(Debug)]
pub enum Renderable<'r> {
    View(view::View<'r>),
    Text(text::Text<'r>),
}

impl<'r> Renderable<'r> {
    pub fn get_style(&self) -> &style::Style {
        match *self {
            Renderable::View(ref view) => &view.style,
            Renderable::Text(ref text) => &text.style,
        }
    }
}
