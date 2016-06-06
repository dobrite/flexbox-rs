
use renderable;

pub struct Dim {
    width: u32,
    height: u32,
}

impl Dim {
    pub fn new(width: u32, height: u32) -> Dim {
        Dim {
            width: width,
            height: height,
        }
    }
}

pub trait Measure<'r> {
    fn get_dim(&self, r: &renderable::Renderable<'r>) -> Dim;
}
