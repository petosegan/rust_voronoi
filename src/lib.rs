#[macro_use]
extern crate log;
extern crate rand;
extern crate ordered_float;

mod geometry;
mod point;
mod dcel;
mod beachline;
mod event;
mod voronoi;

pub use voronoi::voronoi;
pub use point::Point;
pub use dcel::make_line_segments;