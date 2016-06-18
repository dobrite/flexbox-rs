
use std::default::Default;

use rgb;
use style;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub fg: rgb::RGB<u8>,
    pub bg: style::BackgroundColor,
    pub flex_direction: style::FlexDirection,
}

impl Cursor {
    pub fn new(width: u32, height: u32) -> Self {
        Cursor {
            width: width,
            height: height,
            ..Default::default()
        }
    }

    pub fn apply_style(&mut self, style: &style::Style) {
        if let Some(bg) = style.bg {
            self.bg = bg;
        }
        // TODO prob more
    }

    pub fn compute_bg(&self, bg_style: Option<style::BackgroundColor>) -> Option<rgb::RGB<u8>> {
        match bg_style {
            None => {
                match self.bg {
                    style::BackgroundColor::Transparent => None,
                    style::BackgroundColor::Color(color) => Some(color),
                }
            }
            Some(bg_color) => {
                match bg_color {
                    style::BackgroundColor::Transparent => None,
                    style::BackgroundColor::Color(color) => Some(color),
                }
            }
        }
    }

    // TODO compute_fg
}

#[cfg(test)]
mod tests {
    use rgb;
    use style;
    use super::Cursor;

    #[test]
    fn it_computes_bg_when_given_bg_style_None_and_cursor_style_Transparent() {
        let mut cursor = Cursor::new(0, 0);
        cursor.bg = style::BackgroundColor::Transparent;
        assert_eq!(None, cursor.compute_bg(None));
    }

    fn it_computes_bg_when_given_bg_style_None_and_cursor_style_Color() {
        let mut cursor = Cursor::new(0, 0);
        let color = rgb::RGB::new(0, 0, 0);
        cursor.bg = style::BackgroundColor::Color(color);
        assert_eq!(Some(color), cursor.compute_bg(None));
    }

    fn it_computes_bg_when_given_bg_style_Some_Transparent() {
        let mut cursor = Cursor::new(0, 0);
        let style = style::Style::new().with_bg(style::BackgroundColor::Transparent);
        assert_eq!(None, cursor.compute_bg(style.bg));
    }

    fn it_computes_bg_when_given_bg_style_Some_Color() {
        let mut cursor = Cursor::new(0, 0);
        let color = rgb::RGB::new(0, 0, 0);
        let style = style::Style::new().with_bg(style::BackgroundColor::Color(color));
        assert_eq!(Some(color), cursor.compute_bg(style.bg));
    }
}
