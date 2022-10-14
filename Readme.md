# voronoi
This is a Rust implementation of Fortune's Linesweep algorithm for computing Voronoi diagrams.

[Online Documentation](https://docs.rs/voronoi/0.1.4/voronoi/)

## Usage
To use, add the following line to `Cargo.toml` under `[dependencies]`:
```toml
voronoi = "0.1.4"
```
or alternatively,
```toml
voronoi = { git = "https://github.com/petosegan/rust_voronoi.git" }
```

## Example
```rust
extern crate voronoi;
use voronoi::{voronoi, Point, make_polygons};
const BOX_SIZE: f64 = 800.;
// ...
let vor_pts = vec![Point::new(0.0, 1.0), Point::new(2.0, 3.0), Point::new(10.0, 12.0)];
let vor_diagram = voronoi(vor_pts, (BOX_SIZE, BOX_SIZE));
let vor_polys = make_polygons(&vor_diagram);
```

## TODO
* Handle degeneracies in geometry.rs
* Match DCEL faces to input points
* Reimplement the data structures with memory management
* Balance the trees
* Benchmark against other implementations