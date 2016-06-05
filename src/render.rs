
use layout::Command;

pub trait Render {
    fn render(&mut self, layout: &[Command]);
}
