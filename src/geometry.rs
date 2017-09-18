extern crate rand;
extern crate ordered_float;

use rand::{Rng, Rand};
use std::ops::Mul;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const NIL: usize = !0;

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub struct DCEL {
	vertices: Vec<Vertex>,
	faces: Vec<Face>,
	halfedges: Vec<HalfEdge>,
}

impl DCEL {
	pub fn new() -> Self {
		DCEL {vertices: vec![], faces: vec![], halfedges: vec![]}
	}
	pub fn add_twins(&mut self) -> (usize, usize) {
		let mut he1 = HalfEdge::new();
		let mut he2 = HalfEdge::new();

		let start_index = self.halfedges.len();
		he1.twin = start_index + 2;
		he2.twin = start_index + 1;
		self.halfedges.push(he1);
		self.halfedges.push(he2);
		(start_index + 1, start_index + 2)
	}
}

pub struct Vertex {
	coordinates: Point,
	incident_edge: usize, // index of halfedge
}

pub struct Face {
	outer_component: Option<usize>, // index of halfedge
}

pub struct HalfEdge {
	origin: usize, // index of vertex
	twin: usize, // index of halfedge
	incident_face: usize, // index of face
	next: usize, // index of halfedge
	prev: usize, // index of halfedge
}

impl HalfEdge {
	pub fn new() -> Self {
		HalfEdge {origin: NIL, twin: NIL, incident_face: NIL, next: NIL, prev: NIL}
	}
}

struct BeachLine {
	nodes: Vec<BeachNode>,
	y_line: f64,
	root: usize,
}

struct BeachNode {
	parent: Option<usize>,
	left_child: Option<usize>,
	right_child: Option<usize>,
	item: BeachItem,
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

enum BeachItem {
	Leaf(Arc),
	Internal(BreakPoint),
}

struct Arc {
	site: Point,
	site_event: Option<usize>,
}

struct BreakPoint {
	left_site: Point,
	right_site: Point,
	halfedge: usize, // index of halfedge
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
		let mut current_node = 0; // root
		loop {
			match self.nodes[current_node].item {
				BeachItem::Leaf(ref arc) => { return current_node; }
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
	fn remove_arc(&mut self, arc: usize) {
		unimplemented!();
	}
}

fn get_breakpoint_x(bp: &BreakPoint, yl: f64) -> f64 {
	let px1 = bp.left_site.x();
	let px2 = bp.right_site.x();
	let py1 = bp.left_site.y();
	let py2 = bp.right_site.y();

	// TODO: cover py1 = py2 case, and py1 = yl

	let c = ((py2 - yl) / (py1 - yl)).sqrt();

	return (c * px1 - px2) / (c - 1.);
}

// This circle event representation is redundant,
// but it means I can get the height of the event
// without passing in the BeachLine
enum VoronoiEvent {
	Site(Point),
	Circle(usize, TripleSite), // index of disappearing arc, points of circle
}

type TripleSite = (Point, Point, Point);

impl VoronoiEvent {
	pub fn get_y(&self) -> ordered_float::OrderedFloat<f64> {
		match *self {
			VoronoiEvent::Site(ref pt) => pt.y,
			VoronoiEvent::Circle(_, triplesite) => circle_bottom(triplesite),
		}
	}
}

// TODO: handle all the special cases
fn circle_bottom(triple_site: TripleSite) -> ordered_float::OrderedFloat<f64> {
	let (p1, p2, p3) = triple_site;
	let x1 = p1.x();
	let x2 = p2.x();
	let x3 = p3.x();
	let y1 = p1.y();
	let y2 = p2.y();
	let y3 = p3.y();

	let c1 = x3 * x3 + y3 * y3 - x1 * x1 - y1 * y1;
	let c2 = x3 * x3 + y3 * y3 - x2 * x2 - y2 * y2;
	let a1 = 2. * (x1 - x3);
	let a2 = 2. * (x2 - x3);
	let b1 = 2. * (y1 - y3);
	let b2 = 2. * (y2 - y3);

	let numer = c1 * a2 - c2 * a1;
	let denom = b1 * a2 - b2 * a1;

	let y_cen = numer / denom;

	let x_cen = (c2 - b2 * y_cen) / a2;

	let r = ((x3 - x_cen) * (x3 - x_cen) + (y3 - y_cen) * (y3 - y_cen)).sqrt();

	return ordered_float::OrderedFloat::<f64>(y_cen - r);
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
	pub fn remove(&mut self, index: usize) {
		self.events.remove(index);
	}
}

pub fn voronoi(points: Vec<Point>) -> DCEL {
	let mut event_queue = EventQueue::new();
	for pt in points {
		event_queue.push(VoronoiEvent::Site { 0: pt });
	}
	let mut beachline = BeachLine::new();
	let mut result = DCEL::new();

	while !event_queue.is_empty() {
		let this_event = event_queue.pop().unwrap();
		handle_event(this_event, &mut event_queue, &mut beachline, &mut result);
	}
	add_bounding_box(&beachline, &mut result);
	add_cell_records(&mut result);
	return result;
}

fn handle_event(this_event: VoronoiEvent, queue: &mut EventQueue, beachline: &mut BeachLine, result: &mut DCEL) {
	match this_event {
		VoronoiEvent::Site(pt) => { handle_site_event(pt, queue, beachline, result); },
		VoronoiEvent::Circle(leaf, _) => { handle_circle_event(leaf, queue, beachline, result); }
	}
}

fn handle_site_event(site: Point, queue: &mut EventQueue, beachline: &mut BeachLine, result: &mut DCEL) {
	
	if beachline.is_empty() {
		beachline.insert_point(site);
		return;
	}
	
	let arc_above = beachline.get_arc_above(site);

	remove_false_alarm(arc_above, beachline, queue);

	split_arc(arc_above, site, beachline, result);

	//    Check the triple of arcs from p leftward to see
	//    if the breakpoints converge. If so, insert the
	//    circle event and add pointers to Status.
	//    Repeat for the rightward triple.
	unimplemented!();

}

fn remove_false_alarm(arc_above: usize, beachline: &mut BeachLine, queue: &mut EventQueue) {
	let mut has_circle = false;
	let mut circle_leaf;
	if let BeachItem::Leaf(ref arc) = beachline.nodes[arc_above].item {
		if let Some(event_index) = arc.site_event {
			if let VoronoiEvent::Circle(leaf, _) = queue.events[event_index] {
				has_circle = true;
				circle_leaf = leaf;
			} else {
				panic!("arcs should only point to circle events!");
			}
		}
	} else {
		panic!("arc above should always be a leaf!");
	}
	if has_circle { 
		unimplemented!();
	}
}

#[allow(non_snake_case)]
fn split_arc(arc: usize, pt: Point, beachline: &mut BeachLine, dcel: &mut DCEL) {
	let parent = beachline.nodes[arc].parent;

	let mut arc_pt = Point::new(0.0, 0.0);
	if let BeachItem::Leaf(ref this_arc) = beachline.nodes[arc].item {
		arc_pt = this_arc.site;
	}

	let (twin1, twin2) = dcel.add_twins();

	// TODO: set site_events?
	
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

	let node_A2 = BeachNode::make_arc(Some(ind_A2), leaf_A2);
	beachline.nodes.push(node_A2);

}

fn handle_circle_event(
	leaf: usize,
	queue: &mut EventQueue,
	beachline: &mut BeachLine,
	result: &mut DCEL) {
	// 1. Delete the leaf from BeachLine. Update breakpoints.
	//    Delete all circle events involving leaf.
	unimplemented!();

	// 2. Add the center of the circle as a vertex to the
	//    DCEL. Create halfedges for the breakpoint, and
	//    set their pointers. Attach to the half-edges
	//    that end at the vertex
	unimplemented!();

	// 3. Check new triple of arcs centered on right neighbor
	//    to see if breakpoints converge. If so, insert
	//    the circle event and add pointers to BeachLine.
	//    Repeat for the left neighbor triple.
	unimplemented!();
}

fn add_bounding_box(beachline: &BeachLine, result: &mut DCEL) { unimplemented!(); }

fn add_cell_records(result: &mut DCEL) { unimplemented!(); }