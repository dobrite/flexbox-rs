
use data::Size;

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

pub struct Style {
    pub size: Size,
}

impl Style {
    pub fn new(size: Size) -> Self {
        Style { size: size }
    }
}
