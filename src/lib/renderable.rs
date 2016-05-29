
use super::view;

pub enum Renderable<'a> {
    View(view::View<'a>),
    Text(&'a str),
}
