
use super::Renderable;
use super::style::Style;

pub struct View<'a> {
    style: Style,
    children: Vec<Renderable<'a>>,
}

impl<'a> View<'a> {
    pub fn new(style: Style, children: Vec<Renderable<'a>>) -> Self {
        View {
            style: style,
            children: children,
        }
    }
}
