use std::fmt;
use point::Point;
use std::collections::HashSet;

const NIL: usize = !0;

#[derive(Debug)]
pub struct DCEL {
	pub vertices: Vec<Vertex>,
	pub halfedges: Vec<HalfEdge>,
	faces: Vec<Face>,
}

impl DCEL {
	pub fn new() -> Self {
		DCEL {vertices: vec![],
			halfedges: vec![],
			faces: vec![]}
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

        let mut faces_disp = String::new();

        for (index, node) in self.faces.iter().enumerate() {
        	faces_disp.push_str(format!("{}: {}\n", index, node).as_str());
        }

        let mut halfedges_disp = String::new();

        for (index, node) in self.halfedges.iter().enumerate() {
            halfedges_disp.push_str(format!("{}: {}\n", index, node).as_str());
        }

        write!(f, "Vertices:\n{}\nFaces:\n{}\nHalfedges:\n{}", vertices_disp, faces_disp, halfedges_disp)
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
	face: usize, // index of face
}

impl fmt::Display for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "origin: {}, twin: {}, next: {}", self.origin, self.twin, self.next)
    }
}

impl HalfEdge {
	pub fn new() -> Self {
		HalfEdge {origin: NIL, twin: NIL, next: NIL, face: NIL}
	}
}

#[derive(Debug)]
pub struct Face {
	outer_component: usize // index of halfedge
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "outer: {}", self.outer_component)
    }
}

impl Face {
	pub fn new(edge: usize) -> Self {
		Face {outer_component: edge}
	}
}

pub fn add_faces(dcel: &mut DCEL) {
	if !dcel.faces.is_empty() { panic!("add_faces only works on DCELs with no faces");}
	let mut seen_edges = HashSet::new();
	let num_halfedges = dcel.halfedges.len();

	let mut processed_edges = 0;
	info!("Adding faces. There are {} halfedges.", num_halfedges);

	for edge_index in 0..num_halfedges {
		if seen_edges.contains(&edge_index) { continue; }
		processed_edges += 1;

		let face_index = dcel.faces.len();
		let new_face = Face::new(edge_index);
		dcel.faces.push(new_face);

		let mut current_edge = edge_index;
		loop {
			seen_edges.insert(current_edge);
			dcel.halfedges[current_edge].face = face_index;
			current_edge = dcel.halfedges[current_edge].next;
			if current_edge == edge_index { break; }
		}
	}
	info!("Generated faces for {} edges.", processed_edges);
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

pub fn make_polygons(dcel: &DCEL) -> Vec<Vec<Point>> {
	let mut result = vec![];
	for face in &dcel.faces {
		let mut this_poly = vec![];
		let start_edge = face.outer_component;
		let mut current_edge = start_edge;
		loop {
			let this_origin = dcel.halfedges[current_edge].origin;
			this_poly.push(dcel.vertices[this_origin].coordinates);
			current_edge = dcel.halfedges[current_edge].next;
			if current_edge == start_edge { break; }
		}
		result.push(this_poly);
	}
	return result;
}