#![cfg(feature = "sdl2")]

pub extern crate sdl2;

use layout::Layout;
use render::Render;
use rect::Rect;
use rgb::RGB;

impl<'a> Render for sdl2::render::Renderer<'a> {
    fn render(&mut self, layout: &[Layout]) {
        for l in layout {
            self.set_draw_color(to_sdl2_color(l.bg));
            let _ = self.fill_rect(to_sdl2_rect(l.rect));
        }

        self.present();
    }
}

fn to_sdl2_color(c: RGB<u8>) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(c.r, c.g, c.b)
}

fn to_sdl2_rect(r: Rect) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(r.left, r.top, r.width, r.height)
}
