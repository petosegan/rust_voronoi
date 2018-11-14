#![deny(missing_docs,
        missing_debug_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

//! A Rust implementation of Fortune's Linesweep algorithm for computing Voronoi diagrams.

#[macro_use]
extern crate log;
extern crate rand;
extern crate ordered_float;
extern crate fnv;

#[cfg(feature = "serde_support")]
extern crate serde;

#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde_derive;

mod geometry;
mod point;
mod dcel;
mod beachline;
mod event;
mod voronoi;
mod lloyd;

pub use voronoi::voronoi;
pub use point::Point;
pub use dcel::{DCEL, make_line_segments, make_polygons};
pub use lloyd::{lloyd_relaxation, polygon_centroid};
