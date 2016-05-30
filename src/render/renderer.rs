
use sdl2;

use lib::Layout;

pub fn render(r: &mut sdl2::render::Renderer, layout: &[Layout]) {
    for l in layout {
        r.set_draw_color(l.bg);
        let _ = r.fill_rect(l.rect);
    }

    r.present();
}
