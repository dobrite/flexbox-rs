
use renderable::Renderable;
use style;
use view::View;

/// conceptually equal to root returning a body tag with fixed width height e.g:
/// and the children being the inner elements.
///
/// <body style="width: Xpx; height: Ypx">
///   <!-- children -->
/// </body>

pub struct Root<'r> {
    pub width: u32,
    pub height: u32,
    root: Renderable<'r>,
}

impl<'r> Root<'r> {
    pub fn new(style: style::Style, children: Vec<Renderable<'r>>) -> Self {
        Root {
            width: style.width.expect("must pass width"),
            height: style.height.expect("must pass height"),
            root: Renderable::View(View::new(style, children)),
        }
    }

    pub fn root(&self) -> &Renderable<'r> {
        &self.root
    }
}
