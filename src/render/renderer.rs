
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use lib::Layout;

pub struct Renderer { }

pub fn render(r: &mut sdl2::render::Renderer, layout: &[Layout]) {
    for l in layout {
        r.set_draw_color(l.bg);
        let _ = r.fill_rect(l.rect);
    }

    r.present();
}
