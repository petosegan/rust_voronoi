use std::fmt;
use point::Point;
use ordered_float::OrderedFloat;
use geometry::circle_bottom;

type TripleSite = (Point, Point, Point);

// This circle event representation is redundant,
// but it means I can get the height of the event
// without passing in the BeachLine
#[derive(Debug)]
pub enum VoronoiEvent {
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

impl VoronoiEvent {
	pub fn get_y(&self) -> OrderedFloat<f64> {
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

pub struct EventQueue {
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