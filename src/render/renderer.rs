
use sdl2;

use lib::{Layout, Rect, RGB, Render};

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
    sdl2::rect::Rect::new(r.top, r.left, r.width, r.height)
}
