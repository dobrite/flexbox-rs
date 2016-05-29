
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use lib::Layout;

pub struct Renderer { }

pub fn render(renderer: &mut sdl2::render::Renderer, layout: &[Layout]) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    let _ = renderer.draw_rect(Rect::new(100, 200, 78, 20));
    renderer.present();
}
