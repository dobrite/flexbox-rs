extern crate sdl2;
extern crate sdl2_ttf;
extern crate flexbox;

mod sdl2_utils;

use std::path::Path;

use sdl2_utils::{Camera, Events};
use flexbox::{BackgroundColor, Layout, FlexDirection, Renderable, RGB, Root, Position, Style,
              View, Render, sdl2_backend};

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

    let root = Root::new(Style::new()
                             .with_width(width)
                             .with_height(height),
                         vec![
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                  .with_height(50)
                                  .with_width(100), vec![])),
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(200, 100, 255)))
                                  .with_height(50)
                                  .with_width(100)
                                  .with_right(20)
                                  .with_position(Position::Fixed), vec![]))
   ]);

    let font = ttf_context.load_font(Path::new("./examples/assets/fonts/Monospace.ttf"), 16)
        .unwrap();
    let mut renderer = sdl2_backend::Renderer::new(sdl2_renderer, &font);
    let measurer = sdl2_backend::Measurer::new(ttf_context, &font);
    let layout = Layout::new(&measurer);
    let mut camera = Camera::new();

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        if events.key_j == Some(true) {
            camera.down();
        }

        if events.key_k == Some(true) {
            camera.up();
        }

        if events.key_l == Some(true) {
            camera.right();
        }

        if events.key_h == Some(true) {
            camera.left();
        }

        renderer.render(&layout.layout(&root, camera.loc.into_offset()));
    }
}
