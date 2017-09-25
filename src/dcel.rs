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

// does not handle the case where line goes through dcel vertex
pub fn add_line(seg: Segment, mut dcel: DCEL) -> DCEL {
	let mut intersections = get_line_intersections(seg, &dcel);
	intersections.sort_by(|a, b| a.0.cmp(&b.0));
	let start_pt = if seg[0] < seg[1] { seg[0] } else { seg[1] };
	let end_pt   = if seg[0] < seg[1] { seg[1] } else { seg[0] };

	let (mut line_needs_next, mut line_needs_prev, _) = add_twins_from_pt(start_pt, &mut dcel);
	dcel.halfedges[line_needs_prev].next = line_needs_next;
	let prev_pt = start_pt;

	for (int_pt, this_cut_edge) in intersections {
		let (new_line_needs_next, new_line_needs_prev, new_pt_ind) = add_twins_from_pt(int_pt, &mut dcel);
		dcel.halfedges[line_needs_prev].origin = new_pt_ind;

		let mut cut_edge = this_cut_edge;
		if makes_left_turn(prev_pt, int_pt, dcel.get_origin(this_cut_edge)) {
			cut_edge = dcel.halfedges[cut_edge].twin;
		}

		let old_cut_next = dcel.halfedges[cut_edge].next;
		let old_cut_twin = dcel.halfedges[cut_edge].twin;
		dcel.halfedges[cut_edge].next = line_needs_prev;
		
		let cut_ext_ind = dcel.halfedges.len();
		let cut_ext_he = HalfEdge { origin: new_pt_ind, next: old_cut_next, twin: old_cut_twin, face: NIL, prev: NIL, alive: true };
		dcel.halfedges.push(cut_ext_he);
		dcel.halfedges[line_needs_next].next = cut_ext_ind;

		let old_twin_next = dcel.halfedges[old_cut_twin].next;
		dcel.halfedges[old_cut_twin].next = new_line_needs_next;

		let twin_ext_ind = dcel.halfedges.len();
		let twin_ext_he = HalfEdge { origin: new_pt_ind, next: old_twin_next, twin: cut_edge, face: NIL, prev: NIL, alive: true };
		dcel.halfedges.push(twin_ext_he);
		dcel.halfedges[new_line_needs_prev].next = twin_ext_ind;

		dcel.halfedges[cut_edge].twin = twin_ext_ind;
		dcel.halfedges[old_cut_twin].twin = cut_ext_ind;

		line_needs_next = new_line_needs_next;
		line_needs_prev = new_line_needs_prev;
	}

	dcel.halfedges[line_needs_next].next = line_needs_prev;
	let end_vertex_ind = dcel.vertices.len();
	let end_vertex = Vertex { coordinates: end_pt, incident_edge: line_needs_prev, alive: true };
	dcel.vertices.push(end_vertex);
	dcel.halfedges[line_needs_prev].origin = end_vertex_ind;
	return dcel;
}

fn makes_left_turn(pt1: Point, pt2: Point, pt3: Point) -> bool {
	let x1 = pt1.x();
	let x2 = pt2.x();
	let x3 = pt3.x();
	let y1 = pt1.y();
	let y2 = pt2.y();
	let y3 = pt3.y();

	(x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1) > 0.
}

fn add_twins_from_pt(start_pt: Point, dcel: &mut DCEL) -> (usize, usize, usize) {
	let (twin1, twin2) = dcel.add_twins();

	let start_vertex = Vertex { coordinates: start_pt, incident_edge: twin1, alive: true };
	let start_vertex_ind = dcel.vertices.len();
	dcel.vertices.push(start_vertex);

	dcel.halfedges[twin1].origin = start_vertex_ind;

	(twin1, twin2, start_vertex_ind)
}

fn get_line_intersections(seg: Segment, dcel: &DCEL) -> Vec<(Point, usize)> {
	let mut intersections = vec![];
	let mut seen_halfedges = HashSet::new();
	for (index, halfedge) in dcel.halfedges.iter().enumerate() {
		let twin = halfedge.twin;
		if seen_halfedges.contains(&index) || seen_halfedges.contains(&twin) || !halfedge.alive { continue; }
		let this_seg = [dcel.get_origin(index), dcel.get_origin(twin)];
		let this_intersection = segment_intersection(seg, this_seg);
		if let Some(int_pt) = this_intersection { intersections.push((int_pt, index)); }
		seen_halfedges.insert(index);
		seen_halfedges.insert(twin);
	}
	return intersections;
}

pub fn make_line_segments(dcel: &DCEL) -> Vec<Segment> {
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