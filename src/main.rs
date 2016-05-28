extern crate sdl2;
mod events;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use events::Events;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("flexbox-rs", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        let _ = renderer.draw_point(Point::new(50, 50));
        let _ = renderer.draw_rect(Rect::new(100, 200, 78, 20));
        renderer.present();
    }
}
