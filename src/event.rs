use std::fmt;
use point::Point;
use ordered_float::OrderedFloat;
use geometry::circle_bottom;
use beachline::{BeachLine, BeachItem};

const NIL: usize = !0;
type TripleSite = (Point, Point, Point);

// This circle event representation is redundant,
// but it means I can get the height of the event
// without passing in the BeachLine
#[derive(Debug, Clone)]
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
            VoronoiEvent::Circle(_, triplesite) => circle_bottom(triplesite).unwrap(),
        }
    }
}

pub struct EventQueue {
    pub events: Vec<VoronoiEvent>,
}

fn parent(node: usize) -> usize {
    if node == NIL || node == 0 { return NIL; }
    return (((node - 1) as f32) / 2.).floor() as usize
}
fn right_child(node: usize) -> usize {
    2 * node + 1
}
fn left_child(node: usize) -> usize {
    2 * node + 2
}

impl fmt::Display for EventQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut queue_disp = String::new();

        for (index, event) in self.events.iter().enumerate() {
            queue_disp.push_str(format!("{}: {}", index, event).as_str());
            queue_disp.push_str("\n");
        }

        write!(f, "\n{}", queue_disp)
    }
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: vec![] }
    }
    pub fn push(&mut self, event: VoronoiEvent, beachline: &mut BeachLine) {
        let new_node_ind = self.events.len();
        info!("pushing event {}", new_node_ind);
        self.events.push(event);
        self.bubble_up(new_node_ind, beachline);
    }

    // assumes that the only violation of the heap property
    // is that the bubble might be larger than nodes above it
    fn bubble_up(&mut self, bubble_node: usize, beachline: &mut BeachLine) {
        info!("bubbling up node {}", bubble_node);
        let mut current_parent = parent(bubble_node);
        let mut current_bubble = bubble_node;
        let bubble_key = self.events[bubble_node].get_y();
        while current_parent != NIL {
            let parent_key = self.events[current_parent].get_y();
            if bubble_key <= parent_key { break; }
            self.swap(current_bubble, current_parent, beachline);
            current_bubble = current_parent;
            current_parent = parent(current_bubble);
        }
    }
    // assumes that the only violation of the heap property
    // is that the bubble might be smaller than nodes below it
    fn bubble_down(&mut self, bubble_node: usize, beachline: &mut BeachLine) {
        let mut largest = bubble_node;
        let bubble_key = self.events[bubble_node].get_y();

        if left_child(bubble_node) < self.events.len() {
            let left_key = self.events[left_child(bubble_node)].get_y();
            if left_key > bubble_key { largest = left_child(bubble_node); }
        }
        if right_child(bubble_node) < self.events.len() {
            let right_key = self.events[right_child(bubble_node)].get_y();
            let largest_key = self.events[largest].get_y();
            if right_key > largest_key { largest = right_child(bubble_node); }
        }
        if largest != bubble_node {
            self.swap(bubble_node, largest, beachline);
            self.bubble_down(largest, beachline);
        }
    }
    fn swap(&mut self, node_a: usize, node_b: usize, beachline: &mut BeachLine) {
        info!("swapping {} and {}", node_a, node_b);
        let mut leaf_a = NIL;
        let mut leaf_b = NIL;
        if let VoronoiEvent::Circle(l_a, _) = self.events[node_a] {
            leaf_a = l_a;
        }
        if let VoronoiEvent::Circle(l_b, _) = self.events[node_b] {
            leaf_b = l_b;
        }

        let event_a = self.events[node_a].clone();
        self.events[node_a] = self.events[node_b].clone();
        self.events[node_b] = event_a;

        if leaf_a != NIL {
            if let BeachItem::Leaf(ref mut arc_a) = beachline.nodes[leaf_a].item {
                info!("swap a: switched arc {} to point to {}", leaf_a, node_b);
                arc_a.site_event = Some(node_b);
            } else {
                panic!("circle event pointed to non-arc!");
            }
        }
        if leaf_b != NIL {
            if let BeachItem::Leaf(ref mut arc_b) = beachline.nodes[leaf_b].item {
                info!("swap b: switched arc {} to point to {}", leaf_b, node_a);
                arc_b.site_event = Some(node_a);
            } else {
                panic!("circle event pointed to non-arc!");
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    pub fn pop(&mut self, beachline: &mut BeachLine) -> Option<VoronoiEvent> {
        if self.is_empty() { return None; }
        let heapsize = self.events.len()-1;
        self.swap(0, heapsize, beachline);
        let result = self.events.pop();

        let mut this_leaf = NIL;
        if let Some(VoronoiEvent::Circle(leaf, _)) = result {
            this_leaf = leaf;
        }
        if this_leaf != NIL {
            if let BeachItem::Leaf(ref mut arc) = beachline.nodes[this_leaf].item {
                info!("popped circle event, so pointed arc {} to None", this_leaf);
                arc.site_event = None;
            } else {
                panic!("circle event pointed to non-arc!");
            }
        }

        if !self.is_empty() {
            self.bubble_down(0, beachline);
        }

        return result;
    }
    pub fn remove(&mut self, removed: usize, beachline: &mut BeachLine) {
        let heapsize = self.events.len()-1;
        info!("removing node {}, heapsize is {}", removed, heapsize);
        self.swap(removed, heapsize, beachline);
        let removed_event = self.events.pop();

        let mut this_leaf = NIL;
        if let Some(VoronoiEvent::Circle(leaf, _)) = removed_event {
            this_leaf = leaf;
        }
        if this_leaf != NIL {
            if let BeachItem::Leaf(ref mut arc) = beachline.nodes[this_leaf].item {
                info!("removed circle event, so pointed arc {} to None", this_leaf);
                arc.site_event = None;
            } else {
                panic!("circle event pointed to non-arc!");
            }
        }

        if self.is_empty() { return; }
        if removed >= self.events.len() { return; }

        // re-establish heap property
        let bubble_key = self.events[removed].get_y();
        let bubble_parent = parent(removed);
        if bubble_parent != NIL {
            let parent_key = self.events[bubble_parent].get_y();
            if bubble_key > parent_key {
                self.bubble_up(removed, beachline);
                return;
            }
        }
        self.bubble_down(removed, beachline);
    }
}
