extern crate rand;
extern crate ordered_float;

use rand::{Rng, Rand};
use std::ops::Mul;
use std::fmt;

const NIL: usize = !0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
	x: ordered_float::OrderedFloat<f64>,
	y: ordered_float::OrderedFloat<f64>
}

impl Point {
	pub fn new(x: f64, y: f64) -> Self {
		Point {x: ordered_float::OrderedFloat::<f64>(x), y: ordered_float::OrderedFloat::<f64>(y)}
	}
	pub fn x(&self) -> f64 {
		self.x.into_inner()
	}
	pub fn y(&self) -> f64 {
		self.y.into_inner()
	}
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0:.1}, {1:.1})", self.x(), self.y())
    }
}

#[allow(unused_variables)]
impl Rand for Point {
	fn rand<R: Rng>(rng: &mut R) -> Point {
		Point::new(rand::random::<f64>(), rand::random::<f64>())
	}
}

impl Mul<f64> for Point {
	type Output = Point;

	fn mul(self, _rhs: f64) -> Point {
		Point::new(self.x.into_inner() * _rhs, self.y.into_inner() * _rhs)
	}
}

#[derive(Debug)]
pub struct DCEL {
	vertices: Vec<Vertex>,
	halfedges: Vec<HalfEdge>,
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
	coordinates: Point,
	incident_edge: usize, // index of halfedge
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, edge: {}", self.coordinates, self.incident_edge)
    }
}

#[derive(Debug)]
pub struct HalfEdge {
	origin: usize, // index of vertex
	twin: usize, // index of halfedge
	next: usize, // index of halfedge
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

#[derive(Debug)]
struct BeachLine {
	nodes: Vec<BeachNode>,
	y_line: f64,
	root: usize,
}

impl fmt::Display for BeachLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut beachline_disp = String::new();

        for (index, node) in self.nodes.iter().enumerate() {
            beachline_disp.push_str(format!("{}: {}", index, node).as_str());
            beachline_disp.push_str("\n");
        }

        write!(f, "\nRoot: {}\ny_line: {}\n{}", self.root, self.y_line, beachline_disp)
    }
}

#[derive(Debug)]
struct BeachNode {
	parent: Option<usize>,
	left_child: Option<usize>,
	right_child: Option<usize>,
	item: BeachItem,
}

impl fmt::Display for BeachNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p: {:?}, l: {:?}, r: {:?}, item: {}", self.parent, self.left_child, self.right_child, self.item)
    }
}

impl BeachNode {
	fn make_root(item: BeachItem) -> Self {
		BeachNode { parent: None, left_child: None, right_child: None, item: item}
	}
	fn make_arc(parent: Option<usize>, item: BeachItem) -> Self {
		if let BeachItem::Leaf(_) = item {
			BeachNode { parent: parent, left_child: None, right_child: None, item: item}
		} else {
			panic!("make_arc can only make Leaf items!");
		}
	}
}

#[derive(Debug)]
enum BeachItem {
	Leaf(Arc),
	Internal(BreakPoint),
}

impl fmt::Display for BeachItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match *self {
        	BeachItem::Leaf(ref arc) => write!(f, "Leaf: {}", arc),
        	BeachItem::Internal(ref bp) => write!(f, "Internal: {}", bp),
        }
    }
}

#[derive(Debug)]
struct Arc {
	site: Point,
	site_event: Option<usize>,
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "site: {}, site_event: {:?}", self.site, self.site_event)
    }
}

#[derive(Debug)]
struct BreakPoint {
	left_site: Point,
	right_site: Point,
	halfedge: usize, // index of halfedge
}

impl fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "left: {}, right: {}, halfedge: {}", self.left_site, self.right_site, self.halfedge)
    }
}

impl BeachNode {
	fn get_y(&self) -> Option<ordered_float::OrderedFloat<f64>> {
		match self.item {
			BeachItem::Leaf(ref arc) => Some(arc.site.y),
			_ => None 
		}
	}
	fn get_x(&self) -> Option<ordered_float::OrderedFloat<f64>> {
		match self.item {
			BeachItem::Leaf(ref arc) => Some(arc.site.x),
			_ => None 
		}
	}
}

impl BeachLine {
	fn new() -> Self {
		BeachLine { nodes: vec![], y_line: 0.0, root: NIL }
	}
	fn is_empty(&self) -> bool {
		self.nodes.is_empty()
	}
	fn insert_point(&mut self, pt: Point) {
		let this_arc = Arc {site: pt, site_event: None};
		let this_item = BeachItem::Leaf(this_arc);
		let this_node = BeachNode::make_root(this_item);
		self.nodes.push(this_node);
		self.root = self.nodes.len() - 1;
	}
	fn get_arc_above(&self, pt: Point) -> usize {
		if self.is_empty() { panic!("can't get_arc_above on empty beachline!"); }
		let mut current_node = self.root;
		loop {
			match self.nodes[current_node].item {
				BeachItem::Leaf(_) => { return current_node; }
				BeachItem::Internal(ref breakpoint) => {
					let x_bp = get_breakpoint_x(breakpoint, pt.y());
					if pt.x() < x_bp { current_node = self.nodes[current_node].left_child.unwrap(); }
					else { current_node = self.nodes[current_node].right_child.unwrap(); }
				}
			}
		}
	}
	fn tree_minimum(&self, root: usize) -> usize {
		let mut current_node = root;
		while let Some(left) = self.nodes[current_node].left_child {
			current_node = left;
		}
		current_node
	}
	fn tree_maximum(&self, root: usize) -> usize {
		let mut current_node = root;
		while let Some(right) = self.nodes[current_node].right_child {
			current_node = right;
		}
		current_node
	}
	fn successor(&self, node: usize) -> Option<usize> {
		if let Some(right) = self.nodes[node].right_child {
			return Some(self.tree_minimum(right));
		}
		let mut current_node = Some(node);
		let mut current_parent = self.nodes[node].parent;
		while current_parent.is_some() && current_node == self.nodes[current_parent.unwrap()].right_child {
			current_node = current_parent;
			current_parent = self.nodes[current_parent.unwrap()].parent;
		}
		return current_parent;
	}
	fn predecessor(&self, node: usize) -> Option<usize> {
		if let Some(left) = self.nodes[node].left_child {
			return Some(self.tree_maximum(left));
		}
		let mut current_node = Some(node);
		let mut current_parent = self.nodes[node].parent;
		while current_parent.is_some() && current_node == self.nodes[current_parent.unwrap()].left_child {
			current_node = current_parent;
			current_parent = self.nodes[current_parent.unwrap()].parent;
		}
		return current_parent;
	}
	fn get_left_arc(&self, node: Option<usize>) -> Option<usize> {
		if let None = node { return None; }
		let node = node.unwrap();
		if let Some(left) = self.predecessor(node) {
			self.predecessor(left)
		} else {
			None
		}
	}
	fn get_right_arc(&self, node: Option<usize>) -> Option<usize> {
		if let None = node { return None; }
		let node = node.unwrap();
		if let Some(right) = self.successor(node) {
			self.successor(right)
		} else {
			None
		}
	}
	fn get_leftward_triple(&self, node: usize) -> Option<TripleSite> {
		let left_arc = self.get_left_arc(Some(node));
		let left_left_arc = self.get_left_arc(left_arc);

		let this_site = self.get_site(Some(node));
		let left_site = self.get_site(left_arc);
		let left_left_site = self.get_site(left_left_arc);

		if this_site.is_some() && left_site.is_some() && left_left_site.is_some() {
			return Some((left_left_site.unwrap(), left_site.unwrap(), this_site.unwrap()));
		} else { return None; }
	}
	fn get_rightward_triple(&self, node: usize) -> Option<TripleSite> {
		let right_arc = self.get_right_arc(Some(node));
		let right_right_arc = self.get_right_arc(right_arc);

		let this_site = self.get_site(Some(node));
		let right_site = self.get_site(right_arc);
		let right_right_site = self.get_site(right_right_arc);

		if this_site.is_some() && right_site.is_some() && right_right_site.is_some() {
			return Some((this_site.unwrap(), right_site.unwrap(), right_right_site.unwrap()));
		} else { return None; }
	}
	fn get_centered_triple(&self, node: usize) -> Option<TripleSite> {
		let right_arc = self.get_right_arc(Some(node));
		let left_arc = self.get_left_arc(Some(node));

		let this_site = self.get_site(Some(node));
		let right_site = self.get_site(right_arc);
		let left_site = self.get_site(left_arc);

		if this_site.is_some() && right_site.is_some() && left_site.is_some() {
			return Some((left_site.unwrap(), this_site.unwrap(), right_site.unwrap()));
		} else { return None; }
	}
	fn get_site(&self, node: Option<usize>) -> Option<Point> {
		if let None = node { return None; }
		let node = node.unwrap();
		if let BeachItem::Leaf(ref arc) = self.nodes[node].item {
			return Some(arc.site);
		} else {
			return None;
		}
	}
}


// TODO: cover py1 = py2 case, and py1 = yl
fn get_breakpoint_x(bp: &BreakPoint, yl: f64) -> f64 {
	let ax = bp.left_site.x();
	let bx = bp.right_site.x();
	let ay = bp.left_site.y();
	let by = bp.right_site.y();

	// shift frames
	let bx_s = bx - ax;
	let ay_s = ay - yl;
	let by_s = by - yl;

	let discrim = ay_s * by_s * ((ay_s - by_s) * (ay_s - by_s) + bx_s * bx_s);
	let numer1 = ay_s * bx_s - discrim.sqrt();
	let denom = ay_s - by_s;

	let mut x1 = numer1 / denom;
	x1 += ax; // shift back to original frame

	return x1;
}

// TODO: handle py == yl case
fn get_breakpoint_y(bp: &BreakPoint, yl: f64) -> f64 {
	let px = bp.left_site.x();
	let py = bp.left_site.y();

	let bp_x = get_breakpoint_x(bp, yl);

	let numer = (px - bp_x) * (px - bp_x);
	let denom = 2. * (py - yl);

	return numer / denom + (py + yl) / 2.;
}

// This circle event representation is redundant,
// but it means I can get the height of the event
// without passing in the BeachLine
#[derive(Debug)]
enum VoronoiEvent {
	Site(Point),
	Circle(usize, TripleSite), // index of disappearing arc, points of circle
}

impl fmt::Display for VoronoiEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match *self {
        	VoronoiEvent::Site(pt) => { write!(f, "Site at {}", pt) },
        	VoronoiEvent::Circle(leaf, triplesite) => { write!(f, "Circle for leaf {}, pts {}, {}, {}", leaf, triplesite.0, triplesite.1, triplesite.2) },
        }
    }
}

type TripleSite = (Point, Point, Point);

impl VoronoiEvent {
	pub fn get_y(&self) -> ordered_float::OrderedFloat<f64> {
		match *self {
			VoronoiEvent::Site(ref pt) => pt.y,
			VoronoiEvent::Circle(_, triplesite) => circle_bottom(triplesite),
		}
	}
	pub fn is_circle_with_leaf(&self, leaf: usize) -> bool {
		match *self {
			VoronoiEvent::Site(_) => false,
			VoronoiEvent::Circle(my_leaf, _) => my_leaf == leaf,
		}
	}
}

fn circle_bottom(triple_site: TripleSite) -> ordered_float::OrderedFloat<f64> {
	let circle_center = circle_center(triple_site);
	let (_, _, p3) = triple_site;
	let x3 = p3.x();
	let y3 = p3.y();
	let x_cen = circle_center.x();
	let y_cen = circle_center.y();

	let r = ((x3 - x_cen) * (x3 - x_cen) + (y3 - y_cen) * (y3 - y_cen)).sqrt();

	return ordered_float::OrderedFloat::<f64>(y_cen - r);
}

// TODO: handle all the special cases
fn circle_center(triple_site: TripleSite) -> Point {
	trace!("Finding center for {}, {}, {}", triple_site.0, triple_site.1, triple_site.2);
	let (p1, p2, p3) = triple_site;
	let x1 = p1.x();
	let x2 = p2.x();
	let x3 = p3.x();
	let y1 = p1.y();
	let y2 = p2.y();
	let y3 = p3.y();

	let c1 = x3 * x3 + y3 * y3 - x1 * x1 - y1 * y1;
	let c2 = x3 * x3 + y3 * y3 - x2 * x2 - y2 * y2;
	let a1 = -2. * (x1 - x3);
	let a2 = -2. * (x2 - x3);
	let b1 = -2. * (y1 - y3);
	let b2 = -2. * (y2 - y3);

	let numer = c1 * a2 - c2 * a1;
	let denom = b1 * a2 - b2 * a1;

	let y_cen = numer / denom;

	let x_cen = (c2 - b2 * y_cen) / a2;
	trace!("center at {}, {}", x_cen, y_cen);

	return Point::new(x_cen, y_cen);
}

// see http://www.kmschaal.de/Diplomarbeit_KevinSchaal.pdf, pg 27
fn breakpoints_converge(triple_site: TripleSite) -> bool {
	let (a, b, c) = triple_site;
	let ax = a.x();
	let ay = a.y();
	let bx = b.x();
	let by = b.y();
	let cx = c.x();
	let cy = c.y();

	(ay - by) * (bx - cx) > (by - cy) * (ax - bx)
}

struct EventQueue {
	events: Vec<VoronoiEvent>,
}

// TODO: implement priority queue with deletion
impl EventQueue {
	pub fn new() -> Self {
		EventQueue { events: vec![] }
	}
	pub fn push(&mut self, event: VoronoiEvent) {
		self.events.push(event);
		self.events.sort_by(|a, b| a.get_y().cmp(&b.get_y()));
	}
	pub fn is_empty(&self) -> bool {
		self.events.is_empty()
	}
	pub fn pop(&mut self) -> Option<VoronoiEvent> {
		self.events.pop()
	}
	pub fn remove_circles_with_leaf(&mut self, leaf: usize) {
		self.events.retain(|x| !x.is_circle_with_leaf(leaf))
	}
}

pub fn voronoi(points: Vec<Point>) -> DCEL {
	trace!("Starting Voronoi Computation");
	let mut event_queue = EventQueue::new();
	for pt in points {
		event_queue.push(VoronoiEvent::Site { 0: pt });
	}
	let mut beachline = BeachLine::new();
	let mut result = DCEL::new();

	while !event_queue.is_empty() {
		trace!("\n\n");
		trace!("Beachline: {}", beachline);
		let this_event = event_queue.pop().unwrap();
		trace!("Popped event from queue: {}", this_event);
		handle_event(this_event, &mut event_queue, &mut beachline, &mut result);
	}
	add_bounding_box(&beachline, &mut result);
	// add_cell_records(&mut result);
	return result;
}

fn handle_event(this_event: VoronoiEvent, queue: &mut EventQueue, beachline: &mut BeachLine, result: &mut DCEL) {
	match this_event {
		VoronoiEvent::Site(pt) => { handle_site_event(pt, queue, beachline, result); },
		VoronoiEvent::Circle(leaf, triplesite) => { handle_circle_event(leaf, triplesite, queue, beachline, result); }
	}
}

fn handle_site_event(site: Point, queue: &mut EventQueue, beachline: &mut BeachLine, result: &mut DCEL) {
	trace!("Handling site event at {}", site);
	if beachline.is_empty() {
		trace!("Beachline was empty, inserting point.");
		beachline.insert_point(site);
		return;
	}
	
	let arc_above = beachline.get_arc_above(site);

	queue.remove_circles_with_leaf(arc_above);

	let new_node = split_arc(arc_above, site, beachline, result);

	if let Some(left_triple) = beachline.get_leftward_triple(new_node) {
		trace!("Checking leftward triple {}, {}, {}", left_triple.0, left_triple.1, left_triple.2);
		if breakpoints_converge(left_triple) {
			trace!("Found converging triple");
			let left_arc = beachline.get_left_arc(Some(new_node)).unwrap();
			let this_event = VoronoiEvent::Circle {0: left_arc, 1: left_triple};
			queue.push(this_event);
		}
	}
	if let Some(right_triple) = beachline.get_rightward_triple(new_node) {
		trace!("Checking rightward triple {}, {}, {}", right_triple.0, right_triple.1, right_triple.2);
		if breakpoints_converge(right_triple) {
			trace!("Found converging triple");
			let right_arc = beachline.get_right_arc(Some(new_node)).unwrap();
			let this_event = VoronoiEvent::Circle {0: right_arc, 1: right_triple};
			queue.push(this_event);
		}
	}
}

#[allow(non_snake_case)]
// return: the index of the node for the new arc
fn split_arc(arc: usize, pt: Point, beachline: &mut BeachLine, dcel: &mut DCEL) -> usize {
	trace!("Splitting arc {}", arc);
	let parent = beachline.nodes[arc].parent;

	let mut arc_pt = Point::new(0.0, 0.0);
	if let BeachItem::Leaf(ref this_arc) = beachline.nodes[arc].item {
		arc_pt = this_arc.site;
	}

	let (twin1, twin2) = dcel.add_twins();
	
	let breakpoint_AB = BreakPoint {
		left_site: arc_pt,
		right_site: pt,
		halfedge: twin1,
	};
	let breakpoint_BA = BreakPoint {
		left_site: pt,
		right_site: arc_pt,
		halfedge: twin2,
	};

	let internal_AB = BeachItem::Internal(breakpoint_AB);
	let internal_BA = BeachItem::Internal(breakpoint_BA);

	let arc_A1 = Arc {
		site: arc_pt,
		site_event: None,
	};
	let arc_A2 = Arc {
		site: arc_pt,
		site_event: None,
	};
	let arc_B = Arc {
		site: pt,
		site_event: None,
	};

	let leaf_A1 = BeachItem::Leaf(arc_A1);
	let leaf_A2 = BeachItem::Leaf(arc_A2);
	let leaf_B = BeachItem::Leaf(arc_B);

	let ind_AB = beachline.nodes.len();
	let ind_BA = ind_AB + 1;
	let ind_A1 = ind_AB + 2;
	let ind_B  = ind_AB + 3;
	let ind_A2 = ind_AB + 4;

	let node_AB = BeachNode { parent: parent, left_child: Some(ind_A1), right_child: Some(ind_BA), item: internal_AB};
	beachline.nodes.push(node_AB);
	if let Some(parent_ind) = parent {
		let parent_node = &mut beachline.nodes[parent_ind];
		if parent_node.right_child.is_some() && parent_node.right_child.unwrap() == arc {
			parent_node.right_child = Some(ind_AB);
		} else if parent_node.left_child.is_some() && parent_node.left_child.unwrap() == arc {
			parent_node.left_child = Some(ind_AB);
		} else {
			panic!("tree is borked");
		}
	} else {
		beachline.root = ind_AB;
	}

	let node_BA = BeachNode {parent: Some(ind_AB), left_child: Some(ind_B), right_child: Some(ind_A2), item: internal_BA};
	beachline.nodes.push(node_BA);

	let node_A1 = BeachNode::make_arc(Some(ind_AB), leaf_A1);
	beachline.nodes.push(node_A1);

	let node_B = BeachNode::make_arc(Some(ind_BA), leaf_B);
	beachline.nodes.push(node_B);

	let node_A2 = BeachNode::make_arc(Some(ind_BA), leaf_A2);
	beachline.nodes.push(node_A2);

	return ind_B;
}

// return: indices of predecessor, successor, parent, 'other'
// where 'other' is the one of predecessor or sucessor that
// is not the parent of the leaf.
fn delete_leaf(leaf: usize, beachline: &mut BeachLine) -> (usize, usize, usize, usize) {
	let pred = beachline.predecessor(leaf).unwrap();
	let succ = beachline.successor(leaf).unwrap();
	let parent = beachline.nodes[leaf].parent.unwrap();
	let grandparent = beachline.nodes[parent].parent.unwrap();
	
	let other = if parent == pred { succ } else { pred };

	let sibling;
	if beachline.nodes[parent].right_child.unwrap() == leaf {
		sibling = beachline.nodes[parent].left_child.unwrap();
	} else if beachline.nodes[parent].left_child.unwrap() == leaf {
		sibling = beachline.nodes[parent].right_child.unwrap();
	} else {
		panic!("family strife! parent does not acknowledge leaf!");
	}

	// transplant the sibling to replace the parent
	beachline.nodes[sibling].parent = Some(grandparent);
	if beachline.nodes[grandparent].left_child.unwrap() == parent {
		beachline.nodes[grandparent].left_child = Some(sibling);
	} else if beachline.nodes[grandparent].right_child.unwrap() == parent {
		beachline.nodes[grandparent].right_child = Some(sibling);
	} else {
		panic!("family strife! grandparent does not acknowledge parent!");
	}

	// correct the site on 'other'
	if other == pred {
		let new_other_succ = beachline.successor(other).unwrap();
		let new_site;
		if let BeachItem::Leaf(ref arc) = beachline.nodes[new_other_succ].item {
			new_site = arc.site;
		} else {
			panic!("successor of breakpoint should be a leaf");
		}
		if let BeachItem::Internal(ref mut bp) = beachline.nodes[other].item {
			bp.right_site = new_site;
		} else {
			panic!("predecessor and successor of leaf should be internal");
		}
	} else {
		let new_other_pred = beachline.predecessor(other).unwrap();
		let new_site;
		if let BeachItem::Leaf(ref arc) = beachline.nodes[new_other_pred].item {
			new_site = arc.site;
		} else {
			panic!("predecessor of breakpoint should be a leaf");
		}
		if let BeachItem::Internal(ref mut bp) = beachline.nodes[other].item {
			bp.left_site = new_site;
		} else {
			panic!("predecessor and successor of leaf should be internal");
		}
	}

	(pred, succ, parent, other)
}

fn handle_circle_event(
	leaf: usize,
	triplesite: TripleSite,
	queue: &mut EventQueue,
	beachline: &mut BeachLine,
	dcel: &mut DCEL) {

	let left_neighbor = beachline.get_left_arc(Some(leaf)).unwrap();
	let right_neighbor = beachline.get_right_arc(Some(leaf)).unwrap();
	let (pred, succ, parent, other) = delete_leaf(leaf, beachline);

	queue.remove_circles_with_leaf(leaf);

	let (twin1, twin2) = dcel.add_twins();

	let circle_center = circle_center(triplesite);
	let center_vertex = Vertex { coordinates: circle_center, incident_edge: twin1};
	let center_vertex_ind = dcel.vertices.len();
	dcel.vertices.push(center_vertex);

	let pred_edge = {
		if let BeachItem::Internal(ref breakpoint) = beachline.nodes[pred].item {
			breakpoint.halfedge
		} else {panic!("predecessor should be Internal");}
	};
	let succ_edge = {
		if let BeachItem::Internal(ref breakpoint) = beachline.nodes[succ].item {
			breakpoint.halfedge
		} else {panic!("successor should be Internal");}
	};
	let parent_edge = {
		if let BeachItem::Internal(ref breakpoint) = beachline.nodes[parent].item {
			breakpoint.halfedge
		} else {panic!("parent should be Internal");}
	};
	let other_edge = {
		if let BeachItem::Internal(ref breakpoint) = beachline.nodes[other].item {
			breakpoint.halfedge
		} else {panic!("other should be Internal");}
	};
	let pred_edge_twin = dcel.halfedges[pred_edge].twin;
	let succ_edge_twin = dcel.halfedges[succ_edge].twin;

	dcel.halfedges[parent_edge].origin = center_vertex_ind;
	dcel.halfedges[other_edge].origin = center_vertex_ind;
	dcel.halfedges[twin1].origin = center_vertex_ind;

	dcel.halfedges[pred_edge_twin].next = succ_edge;
	dcel.halfedges[succ_edge_twin].next = twin1;
	dcel.halfedges[twin2].next = pred_edge;

	if let BeachItem::Internal(ref mut breakpoint) = beachline.nodes[other].item {
		breakpoint.halfedge = twin2;
	}

	if let Some(left_triple) = beachline.get_centered_triple(left_neighbor) {
		trace!("Checking leftward triple {}, {}, {}", left_triple.0, left_triple.1, left_triple.2);
		if breakpoints_converge(left_triple) {
			trace!("Found converging triple");
			let this_event = VoronoiEvent::Circle {0: left_neighbor, 1: left_triple};
			queue.push(this_event);
		}
	}
	if let Some(right_triple) = beachline.get_centered_triple(right_neighbor) {
		trace!("Checking rightward triple {}, {}, {}", right_triple.0, right_triple.1, right_triple.2);
		if breakpoints_converge(right_triple) {
			trace!("Found converging triple");
			let this_event = VoronoiEvent::Circle {0: right_neighbor, 1: right_triple};
			queue.push(this_event);
		}
	}
}

fn add_bounding_box(beachline: &BeachLine, dcel: &mut DCEL) {
	let mut current_node = beachline.tree_minimum(beachline.root);
	trace!("\n\n");
	loop {
		match beachline.nodes[current_node].item {
			BeachItem::Leaf(_) => {},
			BeachItem::Internal(ref breakpoint) => {
				let this_edge = breakpoint.halfedge;
				trace!("Extending halfedge {} with breakpoint {}, {}", this_edge, breakpoint.left_site, breakpoint.right_site);
				let this_x = get_breakpoint_x(&breakpoint, -1000.0);
				let this_y = get_breakpoint_y(&breakpoint, -1000.0);

				let vert = Vertex {coordinates: Point::new(this_x, this_y), incident_edge: this_edge};
				let vert_ind = dcel.vertices.len();

				dcel.halfedges[this_edge].origin = vert_ind;
				let this_twin = dcel.halfedges[this_edge].twin;
				dcel.halfedges[this_twin].next = this_edge;

				dcel.vertices.push(vert);
			}
		}
		if let Some(next_node) = beachline.successor(current_node) {
			current_node = next_node;
		} else { break; }
	}

}

// fn add_cell_records(result: &mut DCEL) { unimplemented!(); }

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