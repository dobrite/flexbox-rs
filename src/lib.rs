
#[cfg(feature = "sdl2")]
pub use backend::sdl2_backend; // maybe no sdl2?

pub use layout::{layout, Layout};
pub use rect::Rect;
pub use render::Render;
pub use renderable::Renderable;
pub use rgb::RGB;
pub use style::Style;
pub use view::View;

pub mod backend;
mod cursor;
mod layout;
mod rect;
pub mod render;
mod renderable;
mod rgb;
mod style;
mod view;
