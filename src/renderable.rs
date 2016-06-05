
use view;

#[allow(dead_code)]
pub enum Renderable<'r> {
    View(view::View<'r>),
    Text(&'r str),
}
