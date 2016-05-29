
use sdl2;

use super::renderable::Renderable;

pub struct Layout {
    bg: sdl2::pixels::Color,
    fg: sdl2::pixels::Color,
    rect: sdl2::rect::Rect,
}

pub fn layout<'a>(root: &Renderable<'a>) -> Vec<Layout> {
    vec![]
}
