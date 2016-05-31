
use Renderable;
use style::Style;

pub struct View<'a> {
    pub style: Style,
    pub children: Vec<Renderable<'a>>,
}

impl<'a> View<'a> {
    pub fn new(style: Style, children: Vec<Renderable<'a>>) -> Self {
        View {
            style: style,
            children: children,
        }
    }
}
