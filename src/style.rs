
use std::default::Default;

use rgb::RGB;
use style;

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
// bg color - transparent
// border color (top, left, right, bottom)
// border style (..)
// border width (..)
// border radius? (..)
// border style (dashed, dotted, solid)?
// opacity ?
// overflow 'visible', 'hidden' (wrap?)
// elevation (shadows to lower layer)

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Row
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackgroundColor {
    Color(RGB<u8>),
    Transparent,
}

impl Default for BackgroundColor {
    fn default() -> BackgroundColor {
        BackgroundColor::Transparent
    }
}

// visible | scroll | hidden | auto
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Overflow {
    Visible,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Overflow {
        Overflow::Visible
    }
}

// static | relative | absolute | sticky | fixed
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Static,
    Fixed,
}

impl Default for Position {
    fn default() -> Position {
        Position::Static
    }
}

// only dim in flex direction applies e.g. row only width, col only height
#[derive(Debug)]
pub struct Style {
    // TODO <D, C> dim, color generic types
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub top: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
    pub right: Option<i32>,
    pub fg: Option<RGB<u8>>, // TODO rename to color?
    pub bg: Option<BackgroundColor>, // TODO rename to background_color?
    pub flex_direction: FlexDirection,
    pub overflow: Overflow,
    pub position: Position,
}

impl Style {
    pub fn new() -> Self {
        Style {
            width: Default::default(),
            height: Default::default(),
            fg: Default::default(),
            bg: Default::default(),
            flex_direction: Default::default(),
            overflow: Default::default(),
            position: Default::default(),
            top: Default::default(),
            bottom: Default::default(),
            left: Default::default(),
            right: Default::default(),
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

    pub fn with_fg(mut self, fg: RGB<u8>) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn with_bg(mut self, bg: style::BackgroundColor) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn with_flex_direction(mut self, fd: FlexDirection) -> Self {
        self.flex_direction = fd;
        self
    }

    pub fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }
}
