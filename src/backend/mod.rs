#[cfg(feature = "web")]
mod web_canvas;
#[cfg(feature = "web")]
pub use web_canvas::*;

#[cfg(feature = "cairo")]
mod cairo_canvas;
#[cfg(feature = "cairo")]
pub use cairo_canvas::*;