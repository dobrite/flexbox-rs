extern crate sdl2;
extern crate sdl2_ttf;
extern crate flexbox;

mod sdl2_utils;

use std::path::Path;

use sdl2_utils::Events;
use flexbox::{Layout, FlexDirection, Renderable, RGB, Root, Style, View, Render, sdl2_backend};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let ttf_context = sdl2_ttf::init().unwrap();

    let width: u32 = 800;
    let height: u32 = 600;

    let window = video.window("flexbox-rs", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let sdl2_renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let root = Root::new(Style::new().with_width(width).with_height(height).with_bg(RGB::new(0, 0, 0)), vec![
        Renderable::View(View::new(Style::new().with_bg(RGB::new(0, 0, 0)).with_flex_direction(FlexDirection::Column), vec![
            Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(255, 0, 0)).with_fg(RGB::new(0, 0, 0)), vec![])),
            Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(0, 255, 0)).with_fg(RGB::new(0, 0, 0)), vec![
                Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(0, 125, 125)).with_fg(RGB::new(0, 0, 0)), vec![])),
                Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(125, 125, 0)).with_fg(RGB::new(0, 0, 0)), vec![])),
            ])),
        ]))
    ]);

    let font = ttf_context.load_font(Path::new("./examples/assets/fonts/Monospace.ttf"), 16).unwrap();
    let mut renderer = sdl2_backend::Renderer::new(sdl2_renderer, &font);
    let measurer = sdl2_backend::Measurer::new(ttf_context, &font);
    let layout = Layout::new(&measurer);

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        renderer.render(&layout.layout(&root));
    }
}
