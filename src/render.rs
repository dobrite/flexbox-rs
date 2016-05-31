
use layout::Layout;

pub trait Render {
    fn render(&mut self, layout: &[Layout]);
}
