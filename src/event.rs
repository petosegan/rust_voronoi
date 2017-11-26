use std::fmt;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use fnv::FnvHashSet;

use point::Point;

#[derive(Clone)]
pub enum VoronoiEvent {
    Site(Point),
    Circle(Point /* center */, f64 /* radius */, usize /* index of disappearing arc */, usize /* id */),
}

impl fmt::Debug for VoronoiEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VoronoiEvent::Site(pt) => { write!(f, "Site at {:?}", pt) },
            VoronoiEvent::Circle(center, radius, leaf, _) => { write!(f, "Circle for leaf {}, center {:?}, radius {:?}", leaf, center, radius) },
        }
    }
}

impl PartialEq for VoronoiEvent {
    fn eq(&self, other: &VoronoiEvent) -> bool {
        self.get_y().eq(&other.get_y())
    }
}

impl Eq for VoronoiEvent {}

impl PartialOrd for VoronoiEvent {
    fn partial_cmp(&self, other: &VoronoiEvent) -> Option<Ordering> {
        let y = self.get_y();
        let other_y = other.get_y();
        y.partial_cmp(&other_y)
    }
}

impl Ord for VoronoiEvent {
    fn cmp(&self, other: &VoronoiEvent) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Greater)
    }
}

impl VoronoiEvent {
    pub fn get_y(&self) -> f64 {
        match *self {
            VoronoiEvent::Site(ref pt) => pt.y(),
            VoronoiEvent::Circle(center, radius, _, _) => center.y() + radius,
        }
    }
}

#[derive(Default)]
pub struct EventQueue {
    pub next_event_id: usize,
    pub events: BinaryHeap<VoronoiEvent>,
    pub removed_event_ids: FnvHashSet<usize>,
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

    pub fn push(&mut self, mut event: VoronoiEvent) -> usize {
        // Set event_id
        let event_id = self.next_event_id;
        self.next_event_id += 1;
        if let VoronoiEvent::Circle(.., ref mut id) = event {
            *id = event_id;
        }

        let new_node_ind = self.events.len();
        info!("pushing event {}", new_node_ind);
        self.events.push(event);

        event_id
    }

    pub fn pop(&mut self) -> Option<VoronoiEvent> {
        while let Some(event) = self.events.pop() {
            // If this event was removed, pop another event
            if let VoronoiEvent::Circle(.., id) = event {
                if self.removed_event_ids.remove(&id) {
                    continue;
                }
            }

            return Some(event);
        }

        None
    }

    pub fn remove(&mut self, event_id: usize) {
        self.removed_event_ids.insert(event_id);
    }
}
