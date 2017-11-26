use std::fmt;

use point::Point;

const NIL: usize = !0;

#[derive(Clone)]
pub enum VoronoiEvent {
    Site(Point),
    Circle(Point /* center */, f64 /* radius */, usize /* index of disappearing arc */),
}

impl fmt::Debug for VoronoiEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VoronoiEvent::Site(pt) => { write!(f, "Site at {:?}", pt) },
            VoronoiEvent::Circle(center, radius, leaf) => { write!(f, "Circle for leaf {}, center {:?}, radius {:?}", leaf, center, radius) },
        }
    }
}

impl VoronoiEvent {
    pub fn get_y(&self) -> f64 {
        match *self {
            VoronoiEvent::Site(ref pt) => pt.y(),
            VoronoiEvent::Circle(center, radius, _) => center.y() + radius,
        }
    }
}

#[derive(Default)]
pub struct EventQueue {
    pub next_event_id: usize,
    pub node_to_id: Vec<usize>,
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

impl fmt::Debug for EventQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut queue_disp = String::new();

        for (index, event) in self.events.iter().enumerate() {
            queue_disp.push_str(format!("{}: {:?}", index, event).as_str());
            queue_disp.push_str("\n");
        }

        write!(f, "\n{}", queue_disp)
    }
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue::default()
    }

    pub fn push(&mut self, event: VoronoiEvent) -> usize {
        let new_node_ind = self.events.len();
        info!("pushing event {}", new_node_ind);
        self.events.push(event);

        // Make event_id
        let event_id = self.next_event_id;
        self.next_event_id += 1;
        self.node_to_id.push(event_id);

        self.bubble_up(new_node_ind);

        event_id
    }

    // assumes that the only violation of the heap property
    // is that the bubble might be larger than nodes above it
    fn bubble_up(&mut self, bubble_node: usize) {
        info!("bubbling up node {}", bubble_node);
        let mut current_parent = parent(bubble_node);
        let mut current_bubble = bubble_node;
        let bubble_key = self.events[bubble_node].get_y();
        while current_parent != NIL {
            let parent_key = self.events[current_parent].get_y();
            if bubble_key <= parent_key { break; }
            self.swap(current_bubble, current_parent);
            current_bubble = current_parent;
            current_parent = parent(current_bubble);
        }
    }

    // assumes that the only violation of the heap property
    // is that the bubble might be smaller than nodes below it
    fn bubble_down(&mut self, bubble_node: usize) {
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
            self.swap(bubble_node, largest);
            self.bubble_down(largest);
        }
    }

    fn swap(&mut self, node_a: usize, node_b: usize) {
        info!("swapping {} and {}", node_a, node_b);
        let id_a = self.node_to_id[node_a];
        let id_b = self.node_to_id[node_b];

        let event_a = self.events[node_a].clone();
        self.events[node_a] = self.events[node_b].clone();
        self.events[node_b] = event_a;

        self.node_to_id[node_a] = id_b;
        self.node_to_id[node_b] = id_a;
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn pop(&mut self) -> Option<VoronoiEvent> {
        if self.is_empty() { return None; }
        let heapsize = self.events.len()-1;
        self.swap(0, heapsize);
        let result = self.events.pop();

        self.node_to_id.pop();

        if !self.is_empty() {
            self.bubble_down(0);
        }

        return result;
    }

    pub fn remove(&mut self, event_id: usize) {
        // Find node_id
        // FIXME: this is very slow!
        let node_id = 'find_node_id: loop {
            for (node_id, event_id_2) in self.node_to_id.iter().enumerate() {
                if *event_id_2 == event_id {
                    break 'find_node_id node_id;
                }
            }
            return;
        };

        let heapsize = self.events.len()-1;
        info!("removing node {}, heapsize is {}", node_id, heapsize);
        self.swap(node_id, heapsize);
        self.events.pop();
        self.node_to_id.pop();

        if self.is_empty() { return; }
        if node_id >= self.events.len() { return; }

        // re-establish heap property
        let bubble_key = self.events[node_id].get_y();
        let bubble_parent = parent(node_id);
        if bubble_parent != NIL {
            let parent_key = self.events[bubble_parent].get_y();
            if bubble_key > parent_key {
                self.bubble_up(node_id);
                return;
            }
        }
        self.bubble_down(node_id);
    }
}
