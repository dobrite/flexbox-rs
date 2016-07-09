
use style::Style;

#[derive(Debug)]
pub struct Text<'a> {
    pub style: Style,
    pub children: &'a str,
}

impl<'a> Text<'a> {
    pub fn new(style: Style, children: &'a str) -> Self {
        Text {
            style: style,
            children: children,
        }
    }
}
