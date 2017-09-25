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
mod intersect;
mod segment_queue;
mod sweepline;

pub use voronoi::voronoi;
pub use point::Point;
pub use dcel::{make_line_segments, make_polygons, add_faces, add_line};
pub use intersect::{all_intersections};