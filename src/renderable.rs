
use view;

#[allow(dead_code)]
pub enum Renderable<'a> {
    View(view::View<'a>),
    Text(&'a str),
}
