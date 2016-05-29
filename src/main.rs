extern crate sdl2;

mod data;
mod events;
mod lib;
mod render;

use events::Events;
use lib::{layout, Renderable, Style, View};
use render::render;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let width: u32 = 800;
    let height: u32 = 600;

    let window = video.window("flexbox-rs", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let root = Renderable::View(View::new(Style::new().with_width(width).with_height(height),
                                          vec![
        Renderable::View(View::new(Style::new().with_width(50).with_height(100), vec![])),
        Renderable::View(View::new(Style::new().with_width(50).with_height(100), vec![])),
    ]));

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        render(&mut renderer, &layout(&root));
    }
}
