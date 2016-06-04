extern crate sdl2;
extern crate sdl2_ttf;
extern crate flexbox;

mod sdl2_utils;

use std::path::Path;

// TODO fg should determine this
use sdl2::pixels::Color::RGBA;
// TODO put in renderer
use sdl2::render::TextureQuery;

use sdl2_utils::Events;
use flexbox::{layout, FlexDirection, Renderable, RGB, Style, View, Render};

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

    let mut renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let root = Renderable::View(View::new(Style::new().with_bg(RGB::new(0, 0, 0)).with_flex_direction(FlexDirection::Column), vec![
        Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(255, 0, 0)).with_fg(RGB::new(0, 0, 0)), vec![])),
        Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(0, 255, 0)).with_fg(RGB::new(0, 0, 0)), vec![
            Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(0, 125, 125)).with_fg(RGB::new(0, 0, 0)), vec![])),
            Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(125, 125, 0)).with_fg(RGB::new(0, 0, 0)), vec![])),
        ])),
    ]));

    let mut font = ttf_context.load_font(Path::new("./examples/assets/fonts/Monospace.ttf"), 12).unwrap();
    let surface = font.render("Hello Rust!").blended(RGBA(255, 0, 0, 255)).unwrap();
    let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    renderer.copy(&mut texture, None, Some(Rect::new(0, 0, 200, 100)));

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        renderer.render(&layout(width, height, &root));
    }
}
