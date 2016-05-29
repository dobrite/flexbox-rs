
use std::default::Default;

use sdl2;

// border-box: content-box -- border and padding included in height width
// flexbox
//   alignItems -- laid out along the cross axis
//   alignSelf -- alignItems override for individual flex items
//   borderWidth (top, left, right, buttom)
//   bottom, left, right, top
//   flex (shorthand for flex-grow, flex-shrink, and flex-basis)
//   flexDirection ('row', 'column')
//   flexWrap ('wrap', 'nowrap')
//   height, width
//   justifyContent ('flex-start', 'flex-end', 'center', 'space-between', 'space-around')
//   margin (bottom, horizontal, vertical, top, right, left)
//   padding (bottom, horizontal, vertical, top, right, left)
//   position ('absolute', 'relative')
// bg color
// border color (top, left, right, bottom)
// border style (..)
// border width (..)
// border radius? (..)
// border style (dashed, dotted, solid)?
// opacity ?
// overflow 'visible', 'hidden' (wrap?)
// elevation (shadows to lower layer)

// only dim in flex direction applies e.g. row only width, col only height
pub struct Style {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fg: Option<sdl2::pixels::Color>,
    pub bg: Option<sdl2::pixels::Color>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            width: Default::default(),
            height: Default::default(),
            fg: Default::default(),
            bg: Default::default(),
        }
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_fg(mut self, fg: sdl2::pixels::Color) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn with_bg(mut self, bg: sdl2::pixels::Color) -> Self {
        self.bg = Some(bg);
        self
    }
}
