#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;
extern crate ordered_float;

mod geometry;
mod point;
mod dcel;
mod beachline;
mod event;
mod voronoi;
mod lloyd;

pub use voronoi::voronoi;
pub use point::Point;
pub use dcel::{make_line_segments, make_polygons};
pub use lloyd::{lloyd_relaxation};