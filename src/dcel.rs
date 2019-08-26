use std::fmt;
use point::Point;
use geometry::{Segment, segment_intersection};

const NIL: usize = !0;

/// Doubly Connected Edge List representation of a subdivision of the plane.
pub struct DCEL {
    /// Vertices
    pub vertices: Vec<Vertex>,
    /// Halfedges
    pub halfedges: Vec<HalfEdge>,
    /// Faces
    pub faces: Vec<Face>,
}

impl DCEL {
    /// Construct an empty DCEL
    pub fn new() -> Self {
        DCEL {vertices: vec![],
            halfedges: vec![],
            faces: vec![]}
    }

    /// Add two halfedges that are twins
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

    /// Get the origin of a halfedge by index
    pub fn get_origin(&self, edge: usize) -> Point {
        let origin_ind = self.halfedges[edge].origin;
        return self.vertices[origin_ind].coordinates;
    }

    /// Set the previous edge of all halfedges
    /// Assumes that the DCEL is well-formed.
    pub fn set_prev(&mut self) {
        let mut seen_edges = vec![false; self.halfedges.len()];
        for edge_ind in 0..self.halfedges.len() {
            if seen_edges[edge_ind] { continue; }
            let mut current_ind = edge_ind;
            seen_edges[current_ind];
            loop {
                let next_edge = self.halfedges[current_ind].next;
                self.halfedges[next_edge].prev = current_ind;
                current_ind = next_edge;
                seen_edges[current_ind] = true;
                if current_ind == edge_ind { break; }
            }
        }
    }

    fn remove_edge(&mut self, edge: usize) {
        let edge_prev = self.halfedges[edge].prev;
        let edge_next = self.halfedges[edge].next;
        let twin = self.halfedges[edge].twin;
        let twin_prev = self.halfedges[twin].prev;
        let twin_next = self.halfedges[twin].next;

        self.halfedges[edge_prev].next = twin_next;
        self.halfedges[edge_next].prev = twin_prev;
        self.halfedges[twin_prev].next = edge_next;
        self.halfedges[twin_next].prev = edge_prev;

        self.halfedges[edge].alive = false;
        self.halfedges[twin].alive = false;
    }

    fn get_edges_around_vertex(&self, vertex: usize) -> Vec<usize> {
        let mut result = vec![];
        let start_edge = self.vertices[vertex].incident_edge;
        let mut current_edge = start_edge;
        loop {
            result.push(current_edge);
            let current_twin = self.halfedges[current_edge].twin;
            current_edge = self.halfedges[current_twin].next;
            if current_edge == start_edge { break; }
        }
        return result;
    }

    /// Remove a vertex and all attached halfedges.
    /// Does not affect faces!!
    pub fn remove_vertex(&mut self, vertex: usize) {
        let vertex_edges = self.get_edges_around_vertex(vertex);
        for edge in vertex_edges {
            self.remove_edge(edge);
        }
        self.vertices[vertex].alive = false;
    }
}

impl fmt::Debug for DCEL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vertices_disp = String::new();

        for (index, node) in self.vertices.iter().enumerate() {
            if node.alive {
                vertices_disp.push_str(format!("{}: {:?}\n", index, node).as_str());
            }
        }

        let mut faces_disp = String::new();

        for (index, node) in self.faces.iter().enumerate() {
            if node.alive {
                faces_disp.push_str(format!("{}: {}\n", index, node).as_str());
            }
        }

        let mut halfedges_disp = String::new();

        for (index, node) in self.halfedges.iter().enumerate() {
            if node.alive {
                halfedges_disp.push_str(format!("{}: {:?}\n", index, node).as_str());
            }
        }

        write!(f, "Vertices:\n{}\nFaces:\n{}\nHalfedges:\n{}", vertices_disp, faces_disp, halfedges_disp)
    }
}

/// A vertex of a DCEL
pub struct Vertex {
    /// (x, y) coordinates
    pub coordinates: Point,
    /// Some halfedge having this vertex as the origin
    pub incident_edge: usize, // index of halfedge
    /// False if the vertex has been deleted
    pub alive: bool,
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, edge: {}", self.coordinates, self.incident_edge)
    }
}

/// A halfedge of a DCEL
pub struct HalfEdge {
    /// The index of the vertex at the start of the halfedge
    pub origin: usize, // index of vertex
    /// The index of the twin halfedge
    pub twin: usize, // index of halfedge
    /// The index of the next halfedge
    pub next: usize, // index of halfedge
    pub face: usize, // index of face
    prev: usize, // index of halfedge
    pub alive: bool,
}

impl fmt::Debug for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "origin: {}, twin: {}, next: {}", self.origin, self.twin, self.next)
    }
}

impl HalfEdge {
    /// Construct an empty halfedge
    pub fn new() -> Self {
        HalfEdge {origin: NIL, twin: NIL, next: NIL, face: NIL, prev: NIL, alive: true}
    }
}

#[derive(Debug)]
/// A face of a DCEL
pub struct Face {
    pub outer_component: usize, // index of halfedge
    pub alive: bool,
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "outer: {}", self.outer_component)
    }
}

impl Face {
    /// Construct a new face, given an attached halfedge index
    pub fn new(edge: usize) -> Self {
        Face {outer_component: edge, alive: true}
    }
}

/// Construct faces for a DCEL.
///
/// # Panics
///
/// This method will panic if the DCEL has any faces already.
pub fn add_faces(dcel: &mut DCEL) {
    if !dcel.faces.is_empty() { panic!("add_faces only works on DCELs with no faces");}
    let num_halfedges = dcel.halfedges.len();
    let mut seen_edges = vec![false; num_halfedges];

    let mut processed_edges = 0;
    info!("Adding faces. There are {} halfedges.", num_halfedges);

    for edge_index in 0..num_halfedges {
        if seen_edges[edge_index] || !dcel.halfedges[edge_index].alive { continue; }
        processed_edges += 1;

        let face_index = dcel.faces.len();
        let new_face = Face::new(edge_index);
        dcel.faces.push(new_face);

        let mut current_edge = edge_index;
        loop {
            seen_edges[current_edge] = true;
            dcel.halfedges[current_edge].face = face_index;
            current_edge = dcel.halfedges[current_edge].next;
            if current_edge == edge_index { break; }
        }
    }
    info!("Generated faces for {} edges.", processed_edges);
}

// does not handle the case where line goes through dcel vertex
/// Add a line segment to a DCEL.
///
/// Vertices and halfedges are constructed and mutated as necessary.
/// Faces are not affected. This should be used before add_faces.
pub fn add_line(seg: Segment, dcel: &mut DCEL) {
    let mut intersections = get_line_intersections(seg, dcel);
    intersections.sort_by(|a, b| a.0.cmp(&b.0));
    let start_pt = if seg[0] < seg[1] { seg[0] } else { seg[1] };
    let end_pt   = if seg[0] < seg[1] { seg[1] } else { seg[0] };

    let (mut line_needs_next, mut line_needs_prev, _) = add_twins_from_pt(start_pt, dcel);
    dcel.halfedges[line_needs_prev].next = line_needs_next;
    let prev_pt = start_pt;

    for (int_pt, this_cut_edge) in intersections {
        let (new_line_needs_next, new_line_needs_prev, new_pt_ind) = add_twins_from_pt(int_pt, dcel);
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
}

/// Do the three points, in this order, make a left turn?
pub fn makes_left_turn(pt1: Point, pt2: Point, pt3: Point) -> bool {
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
    let mut seen_halfedges = vec![false; dcel.halfedges.len()];
    for (index, halfedge) in dcel.halfedges.iter().enumerate() {
        let twin = halfedge.twin;
        if seen_halfedges[index] || seen_halfedges[twin] || !halfedge.alive { continue; }
        let this_seg = [dcel.get_origin(index), dcel.get_origin(twin)];
        let this_intersection = segment_intersection(seg, this_seg);
        if let Some(int_pt) = this_intersection { intersections.push((int_pt, index)); }
        seen_halfedges[index] = true;
        seen_halfedges[twin] = true;
    }
    return intersections;
}

/// Constructs the line segments of the Voronoi diagram.
pub fn make_line_segments(dcel: &DCEL) -> Vec<Segment> {
    let mut result = vec![];
    for halfedge in &dcel.halfedges {
        if halfedge.origin != NIL && halfedge.next != NIL && halfedge.alive {
            if dcel.halfedges[halfedge.next].origin != NIL {
                result.push([dcel.vertices[halfedge.origin].coordinates,
                    dcel.get_origin(halfedge.next)])
            }
        }
    }
    result
}

/// Constructs the faces of the Voronoi diagram.
pub fn make_polygons(dcel: &DCEL) -> Vec<Vec<Point>> {
    let mut result = vec![];
    for face in &dcel.faces {
        if !face.alive { continue; }
        let mut this_poly = vec![];
        let start_edge = face.outer_component;
        let mut current_edge = start_edge;
        loop {
            this_poly.push(dcel.get_origin(current_edge));
            current_edge = dcel.halfedges[current_edge].next;
            if current_edge == start_edge { break; }
        }
        result.push(this_poly);
    }

    // remove the outer face
    result.sort_by(|a, b| a.len().cmp(&b.len()));
    result.pop();

    return result;
}
