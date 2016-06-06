#![cfg(feature = "sdl2")]

pub extern crate sdl2;
pub extern crate sdl2_ttf;

use command::Command;
use measure;
use rect::Rect;
use render::Render;
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

#[allow(dead_code)]
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

impl<'f> measure::Measure for Measurer<'f> {
    fn get_dim(&self, s: &str) -> measure::Dim {
        let (width, height) = self.font.size_of_latin1(s.as_bytes()).unwrap();
        measure::Dim::new(width, height)
    }
}

fn to_sdl2_color(c: RGB<u8>) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(c.r, c.g, c.b)
}

fn to_sdl2_rect(r: Rect) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(r.left, r.top, r.width, r.height)
}

#[cfg(test)]
mod tests {
    extern crate sdl2_ttf;

    use std::path::Path;
    use super::Measurer;
    use Measure;

    #[test]
    fn it_measures() {
        let ttf_context = sdl2_ttf::init().unwrap();
        let path = Path::new("./examples/assets/fonts/Monospace.ttf");
        let font = ttf_context.load_font(path, 16).unwrap();
        let dim = Measurer::new(ttf_context, &font).get_dim("yo dawg!");
        assert_eq!(79, dim.width);
        assert_eq!(17, dim.height);
    }
}
