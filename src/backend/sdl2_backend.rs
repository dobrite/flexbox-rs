#![cfg(feature = "sdl2")]

pub extern crate sdl2;
pub extern crate sdl2_ttf;

use std::path::Path;

use command::Command;
use measure;
use rect::Rect;
use render::Render;
use renderable;
use rgb::RGB;

// TODO fg should determine font color

pub struct Renderer<'f, 'r> {
    renderer: sdl2::render::Renderer<'r>,
    font: &'f sdl2_ttf::Font,
}

impl<'f, 'r> Renderer<'f, 'r> {
    pub fn new(renderer: sdl2::render::Renderer<'r>, font: &'f sdl2_ttf::Font) -> Self {
        Renderer {
            renderer: renderer,
            font: font,
        }
    }
}

impl<'f, 'r> Render for Renderer<'f, 'r> {
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

pub struct Measurer<'f> {
    ttf_context: sdl2_ttf::Sdl2TtfContext,
    font: &'f sdl2_ttf::Font,
}

impl<'f> Measurer<'f> {
    pub fn new(ttf_context: sdl2_ttf::Sdl2TtfContext, font: &'f sdl2_ttf::Font) -> Self {
        Measurer {
            ttf_context: ttf_context,
            font: font,
        }
    }
}

impl<'f, 'r> measure::Measure<'r> for Measurer<'f> {
    fn get_dim(&self, r: &renderable::Renderable<'r>) -> measure::Dim {
        match r {
            &renderable::Renderable::View(ref view) => measure::Dim::new(0, 0), // TODO
            &renderable::Renderable::Text(ref text) => measure::Dim::new(10, 10), // TODO
        }
    }
}

fn to_sdl2_color(c: RGB<u8>) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(c.r, c.g, c.b)
}

fn to_sdl2_rect(r: Rect) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(r.left, r.top, r.width, r.height)
}
