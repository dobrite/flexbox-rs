
#[derive(Clone, Copy, Debug, Default)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGB<T> {
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB { r: r, g: g, b: b }
    }
}
