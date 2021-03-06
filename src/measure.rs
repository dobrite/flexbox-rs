
#[derive(Debug)]
pub struct Dim {
    pub width: u32,
    pub height: u32,
}

impl Dim {
    pub fn new(width: u32, height: u32) -> Dim {
        Dim {
            width: width,
            height: height,
        }
    }
}

pub trait Measure {
    fn get_dim(&self, r: &str) -> Dim;
}
