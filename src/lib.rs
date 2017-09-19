#[macro_use]
extern crate log;
extern crate rand;

mod geometry;

pub use geometry::{Point, voronoi, make_line_segments};