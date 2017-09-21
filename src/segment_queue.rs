use std::fmt;
use point::Point;

type Segment = [Point; 2];

pub struct SegmentQueue {
    events: Vec<SegmentNode>
}

impl SegmentQueue {
    pub fn new() -> Self {
        SegmentQueue { events: vec![] }
    }
    pub fn insert_seg(&mut self, seg: Segment) {
        unimplemented!();
    }
    pub fn insert_pt(&mut self, pt: Point) {
        unimplemented!();
    }
    pub fn pop(&mut self) -> Option<SegmentEvent> {
        unimplemented!();
    }
    pub fn is_empty(&self) -> bool {
        unimplemented!();
    }
    pub fn contains(&self, pt: Point) -> bool {
        unimplemented!();
    }
}

impl fmt::Display for SegmentQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut queue_disp = String::new();

        for (index, node) in self.events.iter().enumerate() {
            queue_disp.push_str(format!("{}: {}", index, node).as_str());
            queue_disp.push_str("\n");
        }

        write!(f, "\n{}", queue_disp)
    }
}

struct SegmentNode {
    parent: Option<usize>,
    left_child: Option<usize>,
    right_child: Option<usize>,
    event: SegmentEvent,
}

impl fmt::Display for SegmentNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p: {:?}, l: {:?}, r: {:?}, event: {}", self.parent, self.left_child, self.right_child, self.event)
    }
}

pub struct SegmentEvent {
    pub point: Point,
    pub segments_below: Vec<Segment>, 
}

impl fmt::Display for SegmentEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut seg_disp = String::new();
        for seg in &self.segments_below {
            seg_disp.push_str(format!("{} to {}, ", seg[0], seg[1]).as_str());
        }
        write!(f, "Point: {}, Segments_Below: {}\n", self.point, seg_disp)
    }
}