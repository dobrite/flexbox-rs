extern crate sdl2;

mod data;
mod events;
mod lib;
mod render;

use data::Size;
use events::Events;
use lib::{layout, Renderable, Style, View};
use render::render;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let size = Size::new(800, 600);

    // TODO assert this cast is ok
    let window = video.window("flexbox-rs", size.width as u32, size.height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let root = Renderable::View(View::new(Style::new(size),
                                          vec![
        Renderable::View(View::new(Style::new(Size::new(50, 100)), vec![])),
        Renderable::View(View::new(Style::new(Size::new(50, 100)), vec![])),
    ]));

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        render(&mut renderer, &layout(&root));
    }
}
