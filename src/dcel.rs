use std::fmt;
use point::Point;

const NIL: usize = !0;

#[derive(Debug)]
pub struct DCEL {
	pub vertices: Vec<Vertex>,
	pub halfedges: Vec<HalfEdge>,
}

impl DCEL {
	pub fn new() -> Self {
		DCEL {vertices: vec![],
			halfedges: vec![]}
	}
	pub fn add_twins(&mut self) -> (usize, usize) {
		let mut he1 = HalfEdge::new();
		let mut he2 = HalfEdge::new();

		let start_index = self.halfedges.len();
		he1.twin = start_index + 1;
		he2.twin = start_index;
		self.halfedges.push(he1);
		self.halfedges.push(he2);
		(start_index, start_index + 1)
	}
}

impl fmt::Display for DCEL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut vertices_disp = String::new();

        for (index, node) in self.vertices.iter().enumerate() {
            vertices_disp.push_str(format!("{}: {}\n", index, node).as_str());
        }

        let mut halfedges_disp = String::new();

        for (index, node) in self.halfedges.iter().enumerate() {
            halfedges_disp.push_str(format!("{}: {}\n", index, node).as_str());
        }

        write!(f, "Vertices:\n{}\nHalfedges:\n{}", vertices_disp, halfedges_disp)
    }
}

#[derive(Debug)]
pub struct Vertex {
	pub coordinates: Point,
	pub incident_edge: usize, // index of halfedge
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, edge: {}", self.coordinates, self.incident_edge)
    }
}

#[derive(Debug)]
pub struct HalfEdge {
	pub origin: usize, // index of vertex
	pub twin: usize, // index of halfedge
	pub next: usize, // index of halfedge
}

impl fmt::Display for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "origin: {}, twin: {}, next: {}", self.origin, self.twin, self.next)
    }
}

impl HalfEdge {
	pub fn new() -> Self {
		HalfEdge {origin: NIL, twin: NIL, next: NIL}
	}
}

pub fn make_line_segments(dcel: &DCEL) -> Vec<(Point, Point)> {
	let mut result = vec![];
	for halfedge in &dcel.halfedges {
		if halfedge.origin != NIL && halfedge.next != NIL {
			if dcel.halfedges[halfedge.next].origin != NIL {
				result.push((dcel.vertices[halfedge.origin].coordinates,
					dcel.vertices[dcel.halfedges[halfedge.next].origin].coordinates))
			}
		}
	}
	result
}