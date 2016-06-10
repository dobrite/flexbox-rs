
use renderable::Renderable;
use style::Style;
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
    pub fn new(width: u32, height: u32, children: Vec<Renderable<'r>>) -> Self {
        let style = Style::new().with_width(width).with_height(height);

        Root {
            width: width,
            height: height,
            root: Renderable::View(View::new(style, children)),
        }
    }

    pub fn root(&self) -> &Renderable<'r> {
        &self.root
    }
}
