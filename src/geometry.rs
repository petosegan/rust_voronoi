extern crate rand;
extern crate ordered_float;

use rand::{Rng, Rand};
use std::ops::Mul;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, PartialEq, Eq)]
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
}

pub struct Vertex {
	coordinates: Point,
	incident_edge: Box<HalfEdge>,
}

pub struct Face {
	outer_component: Option<Box<HalfEdge>>,
	inner_components: Vec<Box<HalfEdge>>,
}

pub struct HalfEdge {
	origin: Box<Vertex>,
	twin: Box<HalfEdge>,
	incident_face: Box<Face>,
	next: Box<HalfEdge>,
	prev: Box<HalfEdge>,
}

struct Status {
	nodes: Vec<StatusNode>,
}

enum StatusNode {
	Leaf {
		site: Point,
		site_event: Option<Box<VoronoiEvent>>,
	},
	Internal {
		left_site: Box<StatusNode>,
		right_site: Box<StatusNode>,
		halfedge: Box<HalfEdge>,
	},
}

impl StatusNode {
	fn get_y(&self) -> Option<ordered_float::OrderedFloat<f64>> {
		match *self {
			StatusNode::Leaf{ref site, ref site_event} => Some(site.y),
			_ => None 
		}
	}
}

impl Status {
	fn new() -> Self {
		Status { nodes: vec![] }
	}
	fn is_empty(&self) -> bool {
		self.nodes.is_empty()
	}
	fn insert_point(&mut self, pt: Point) {

	}
}

enum VoronoiEvent {
	Site(Point),
	Circle(StatusNode),
}

impl VoronoiEvent {
	fn get_y(&self) -> ordered_float::OrderedFloat<f64> {
		match *self {
			VoronoiEvent::Site(ref pt) => pt.y,
			VoronoiEvent::Circle(ref leaf) => leaf.get_y().unwrap(),
		}
	}
}

impl Eq for VoronoiEvent {}
impl PartialEq for VoronoiEvent {
    fn eq(&self, other: &VoronoiEvent) -> bool {
        self.get_y() == other.get_y()
    }
}

impl Ord for VoronoiEvent {
    fn cmp(&self, other: &VoronoiEvent) -> Ordering {
        other.get_y().cmp(&self.get_y())
    }
}
impl PartialOrd for VoronoiEvent {
    fn partial_cmp(&self, other: &VoronoiEvent) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type EventQueue = BinaryHeap<VoronoiEvent>;

pub fn voronoi(points: Vec<Point>) -> DCEL {
	let mut event_queue = BinaryHeap::new();
	for pt in points {
		event_queue.push(VoronoiEvent::Site { 0: pt });
	}
	let mut status = Status::new();
	let mut result = DCEL::new();

	while !event_queue.is_empty() {
		let this_event = event_queue.pop().unwrap();
		handle_event(this_event, &mut event_queue, &mut status, &mut result);
	}
	add_bounding_box(&status, &mut result);
	add_cell_records(&mut result);
	return result;
}

fn handle_event(this_event: VoronoiEvent, queue: &mut EventQueue, status: &mut Status, result: &mut DCEL) {
	match this_event {
		VoronoiEvent::Site(pt) => { handle_site_event(pt, queue, status, result); },
		VoronoiEvent::Circle(leaf) => { handle_circle_event(leaf, queue, status, result); }
	}
}

fn handle_site_event(site: Point, queue: &mut EventQueue, status: &mut Status, result: &mut DCEL) {
	if status.is_empty() {
		status.insert_point(site);
		return;
	}
	// let arc_above = status.get_arc_above(site);

}

fn handle_circle_event(leaf: StatusNode, queue: &mut EventQueue, status: &mut Status, result: &mut DCEL) {}

fn add_bounding_box(status: &Status, result: &mut DCEL) {}

fn add_cell_records(result: &mut DCEL) { }