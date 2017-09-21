use std::fmt;
use point::Point;
use std::collections::HashSet;

type Segment = [Point; 2];

fn get_upper_point(seg: Segment) -> Point {
    let p0 = seg[0];
    let p1 = seg[1];
    if p0 > p1 { p0 } else { p1 }
}

fn get_lower_point(seg: Segment) -> Point {
    let p0 = seg[0];
    let p1 = seg[1];
    if p0 > p1 { p1 } else { p0 }
}

pub struct SegmentQueue {
    events: Vec<SegmentNode>,
    root: Option<usize>,
}

impl SegmentQueue {
    pub fn new() -> Self {
        SegmentQueue { events: vec![], root: None }
    }
    pub fn insert_seg(&mut self, seg: Segment) {
        let upper_point = get_upper_point(seg);
        let lower_point = get_lower_point(seg);
        let mut segs_below = HashSet::new();
        segs_below.insert(seg);
        let upper_event = SegmentEvent {point: upper_point, segments_below: segs_below };
        let lower_event = SegmentEvent {point: lower_point, segments_below: HashSet::new() };
        self.insert_event(upper_event);
        self.insert_event(lower_event);
    }
    pub fn insert_pt(&mut self, pt: Point) {
        self.insert_event(SegmentEvent {point: pt, segments_below: HashSet::new() });
    }
    // TODO: check if point already exists, and if so merge segments_below
    pub fn insert_event(&mut self, event: SegmentEvent) {
        let mut current_parent = None;
        let mut current_node = self.root;

        while let Some(current_node_ind) = current_node {
            current_parent = current_node;
            if event.point < self.events[current_node_ind].event.point {
                current_node = self.events[current_node_ind].left_child;
            } else {
                current_node = self.events[current_node_ind].right_child;
            }
        }

        let this_ind = self.events.len();
        let this_node = SegmentNode { parent: current_parent, left_child: None, right_child: None, event: event.clone()};
        self.events.push(this_node);

        if let None = current_parent {
            self.root = Some(this_ind);
        } else if event.point < self.events[current_parent.unwrap()].event.point {
            self.events[current_parent.unwrap()].left_child = Some(this_ind);
        } else {
            self.events[current_parent.unwrap()].right_child = Some(this_ind);
        }
    }
    pub fn pop(&mut self) -> Option<SegmentEvent> {
        let desired_ind = self.tree_maximum(self.root);
        if let None = desired_ind { return None; }
        let desired_ind = desired_ind.unwrap();
        let desired_event = self.events[desired_ind].event.clone();
        self.delete(desired_ind);
        return Some(desired_event);
    }
    fn delete(&mut self, del_ind: usize) {
        let left = self.events[del_ind].left_child;
        let right = self.events[del_ind].right_child;
        if let None = left {
            self.transplant(del_ind, right);
        } else if let None = right {
            self.transplant(del_ind, left);
        } else {
            let y = self.tree_minimum(right).unwrap();
            if self.events[y].parent != Some(del_ind) {
                let y_right = self.events[y].right_child;
                self.transplant(y, y_right);
                self.events[y].right_child = right;
                self.events[right.unwrap()].parent = Some(y);
            }
            self.transplant(del_ind, Some(y));
            self.events[y].left_child = left;
            self.events[left.unwrap()].parent = Some(y);
        }
    }
    fn transplant(&mut self, u: usize, v: Option<usize>) {
        let u_parent = self.events[u].parent;
        if let None = u_parent {
            self.root = v;
        } else if self.events[u_parent.unwrap()].left_child == Some(u) {
            self.events[u_parent.unwrap()].left_child = v;
        } else {
            self.events[u_parent.unwrap()].right_child = v;
        }
        if let Some(v_ind) = v {
            self.events[v_ind].parent = u_parent;
        }
    }
    fn tree_maximum(&self, root_ind: Option<usize>) -> Option<usize> {
        if let None = root_ind { return None; }
        let root_ind = root_ind.unwrap();

        let mut current_node = root_ind;
        while let Some(right) = self.events[current_node].right_child {
            current_node = right;
        }
        Some(current_node)
    }
    fn tree_minimum(&self, root_ind: Option<usize>) -> Option<usize> {
        if let None = root_ind { return None; }
        let root_ind = root_ind.unwrap();

        let mut current_node = root_ind;
        while let Some(left) = self.events[current_node].left_child {
            current_node = left;
        }
        Some(current_node)
    }
    pub fn is_empty(&self) -> bool {
        self.root == None
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

#[derive(Clone)]
pub struct SegmentEvent {
    pub point: Point,
    pub segments_below: HashSet<Segment>, 
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