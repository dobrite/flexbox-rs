extern crate sdl2;
extern crate flexbox;

mod sdl2_utils;

use sdl2_utils::Events;
use flexbox::{layout, FlexDirection, Renderable, RGB, Style, View, Render};

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

    let root = Renderable::View(View::new(Style::new()
                                              .with_bg(RGB::new(0, 0, 0))
                                              .with_flex_direction(FlexDirection::Column),
                                          vec![
        Renderable::View(View::new(Style::new().with_width(50).with_height(100)
                                   .with_bg(RGB::new(255, 0, 0))
                                   .with_fg(RGB::new(0, 0, 0)), vec![])),
        Renderable::View(View::new(Style::new().with_width(50).with_height(100)
                                   .with_bg(RGB::new(0, 255, 0))
                                   .with_fg(RGB::new(0, 0, 0)), vec![])),
    ]));

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        renderer.render(&layout(width, height, &root));
    }
}
