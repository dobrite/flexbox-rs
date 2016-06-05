#![cfg(feature = "sdl2")]

pub extern crate sdl2;
pub extern crate sdl2_ttf;

use std::path::Path;

use layout::Command;
use render::Render;
use rect::Rect;
use rgb::RGB;

// TODO fg should determine font color

pub struct Renderer<'a> {
    renderer: sdl2::render::Renderer<'a>,
    ttf_context: sdl2_ttf::Sdl2TtfContext,
    font: sdl2_ttf::Font,
}

impl<'a> Renderer<'a> {
    pub fn new(renderer: sdl2::render::Renderer<'a>,
               ttf_context: sdl2_ttf::Sdl2TtfContext,
               font_path: &Path)
               -> Self {

        let font = ttf_context.load_font(font_path, 16).unwrap();

        Renderer {
            renderer: renderer,
            ttf_context: ttf_context,
            font: font,
        }
    }
}

impl<'a> Render for Renderer<'a> {
    fn render(&mut self, layout: &[Command]) {
        self.renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.renderer.clear();

        for l in layout {
            self.renderer.set_draw_color(to_sdl2_color(l.bg));
            let _ = self.renderer.fill_rect(to_sdl2_rect(l.rect));
        }

        let surface = self.font
            .render("Stuffs")
            .blended(sdl2::pixels::Color::RGBA(255, 0, 0, 255))
            .expect("blah");

        let mut texture = self.renderer.create_texture_from_surface(&surface).unwrap();

        let sdl2::render::TextureQuery { width, height, .. } = texture.query();

        self.renderer.copy(&mut texture,
                           None,
                           Some(sdl2::rect::Rect::new(100, 100, width, height)));

        self.renderer.present();
    }
}

fn to_sdl2_color(c: RGB<u8>) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(c.r, c.g, c.b)
}

fn to_sdl2_rect(r: Rect) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(r.left, r.top, r.width, r.height)
}
