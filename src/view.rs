
use Renderable;
use style::Style;

#[derive(Debug)]
pub struct View<'r> {
    pub style: Style,
    pub children: Vec<Renderable<'r>>,
}

impl<'r> View<'r> {
    pub fn new(style: Style, children: Vec<Renderable<'r>>) -> Self {
        View {
            style: style,
            children: children,
        }
    }
}
