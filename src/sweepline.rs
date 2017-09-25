use point::Point;
use std::fmt;
use std::collections::HashSet;
use intersect::{Segment, get_lower_point, segment_intersection, seg_length};

use ordered_float::OrderedFloat;
extern crate env_logger;


const EPSILON: f64 = 0.001; // lol

pub struct SweepLine {
    nodes: Vec<SweepNode>,
    root: Option<usize>,
    pub y_line: f64,
}

fn seg_is_right_of_seg(seg1: Segment, seg2: Segment, y_line: f64) -> bool {
    // trace!("Checking if {} to {} is right of {} to {}", seg1[0], seg1[1], seg2[0], seg2[1]);
    let result = get_segment_x(seg1, y_line) > get_segment_x(seg2, y_line);
    // if result { trace!("yes, it is."); }
    // else { trace!("no, it isn't"); }
    return result;
}

fn pt_is_right_of_seg(pt: Point, seg: Segment, y_line: f64) -> bool {
    pt.x > get_segment_x(seg, y_line)
}


// https://stackoverflow.com/questions/328107/how-can-you-determine-a-point-is-between-two-other-points-on-a-line-segment
fn seg_contains_pt(seg: Segment, pt: Point) -> bool {
    if seg[0] == pt || seg[1] == pt { return false; }
    let a = seg[0];
    let b = seg[1];

    let cross = (pt - a).cross(b - a);
    if cross.abs() > EPSILON { return false; }

    let dot = (pt - a).dot(b - a);
    if dot < 0. { return false; }

    if dot > seg_length(seg) * seg_length(seg) { return false; }

    return true;
}

fn get_segment_x(seg: Segment, y_line: f64) -> OrderedFloat<f64> {
    // trace!("getting segment_x for {} to {} at y_line = {}", seg[0], seg[1], y_line);

    // handle segments that end at the y_line
    let lp = get_lower_point(seg);
    if lp.y() == y_line { return lp.x; }

    let mut x0 = seg[0].x();
    let mut x1 = seg[1].x();
    if x0 == x1 {
        x0 -= 0.5;
        x1 += 0.5
    }
    let y_segment = [Point::new(x0, y_line - EPSILON), Point::new(x1, y_line - EPSILON)];
    let intersection = segment_intersection(seg, y_segment);
    if let None = intersection { panic!("invalid get_segment_x for {} to {} at y_line = {}", seg[0], seg[1], y_line); }
    // trace!("segment_x was {}", intersection.unwrap().x);
    return intersection.unwrap().x;
}

impl fmt::Display for SweepLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut line_disp = String::new();

        for (index, node) in self.nodes.iter().enumerate() {
            line_disp.push_str(format!("{}: {}", index, node).as_str());
            line_disp.push_str("\n");
        }

        write!(f, "\nroot: {:?}\ny_line: {}\n{}", self.root, self.y_line, line_disp)
    }
}

impl SweepLine {
    pub fn new() -> Self {
        SweepLine { nodes: vec![], root: None, y_line: 0.0 }
    }
    fn is_empty(&self) -> bool {
        self.root == None
    }
    fn is_leaf(&self, node: usize) -> bool {
        self.nodes[node].right_child == None && self.nodes[node].left_child == None
    }
    fn remove_segment(&mut self, seg: Segment) {
        info!("Removing segment {} to {} from sweepline", seg[0], seg[1]);
        let seg_node = self.search(seg);
        if let None = seg_node { panic!("tried to remove segment that is not in sweepline"); }
        let seg_node = seg_node.unwrap();

        self.remove(seg_node);
    }
    #[allow(unused_assignments)]
    fn insert_segment(&mut self, seg: Segment) {
        info!("Inserting segment {} to {} in sweepline", seg[0], seg[1]);
        let seg_node = self.search(seg);
        if let Some(_) = seg_node { panic!("tried to insert segment that is already in sweepline"); }
        let mut current_parent = None;
        let mut current_node = self.root;
        let mut new_node_parent = None;

        let new_node_ind = self.nodes.len();
        let new_node = SweepNode { parent: None, left_child: None, right_child: None, segment: seg};
        self.nodes.push(new_node);

        loop {
            if let None = current_node { break; } // tree was empty
            let current_node_ind = current_node.unwrap();

            if self.is_leaf(current_node_ind) {
                trace!("inserting at node {}", current_node_ind);
                let new_internal_ind = self.nodes.len();
                let mut new_internal_left = None;
                let mut new_internal_right = None;

                new_node_parent = Some(new_internal_ind);

                if let None = current_parent {
                    trace!("tree was a singleton");
                    self.root = Some(new_internal_ind);
                } else {
                    let parent_ind: usize = current_parent.unwrap();
                    let parent_left_child: usize = self.nodes[parent_ind].left_child.unwrap();
                    let parent_right_child: usize = self.nodes[parent_ind].right_child.unwrap();
                    if parent_left_child == current_node_ind {
                        self.nodes[parent_ind].left_child = Some(new_internal_ind);
                    } else if parent_right_child == current_node_ind {
                        self.nodes[parent_ind].right_child = Some(new_internal_ind);
                    } else {
                        panic!("tree is borked");
                    }
                }

                let new_internal_seg;
                if seg_is_right_of_seg(seg, self.nodes[current_node_ind].segment, self.y_line) {
                    new_internal_right = Some(new_node_ind);
                    new_internal_left = Some(current_node_ind);
                    new_internal_seg = self.nodes[current_node_ind].segment;
                } else {
                    new_internal_right = Some(current_node_ind);
                    new_internal_left = Some(new_node_ind);
                    new_internal_seg = seg;
                }

                self.nodes[current_node_ind].parent = Some(new_internal_ind);
                let internal_node = SweepNode { parent: current_parent,
                                                left_child: new_internal_left,
                                                right_child: new_internal_right,
                                                segment: new_internal_seg};
                self.nodes.push(internal_node);
                break;
            }

            current_parent = current_node;
            if seg_is_right_of_seg(seg, self.nodes[current_node_ind].segment, self.y_line) {
                current_node = self.nodes[current_node_ind].right_child;
            } else {
                current_node = self.nodes[current_node_ind].left_child;
            }
        }

        if let None = new_node_parent {
            self.root = Some(new_node_ind);
        }
        self.nodes[new_node_ind].parent = new_node_parent;
    }
    fn search(&self, seg: Segment) -> Option<usize> {
        let mut current_node = self.root;
        loop {
            if let None = current_node { return None; }
            let current_node_ind = current_node.unwrap();

            if self.is_leaf(current_node_ind) {
                if self.nodes[current_node_ind].segment == seg { return Some(current_node_ind); }
                else { return None; }
            }

            if seg_is_right_of_seg(seg, self.nodes[current_node_ind].segment, self.y_line) {
                current_node = self.nodes[current_node_ind].right_child;
            } else {
                current_node = self.nodes[current_node_ind].left_child;
            }
        }
    }
    fn remove(&mut self, node: usize) {
        trace!("Removing node {} from sweepline", node);
        if !self.is_leaf(node) { panic!("tried to remove non-leaf node!"); }

        let parent = self.nodes[node].parent;
        if let None = parent {
            trace!("removed node was root, sweepline now empty");
            self.root = None;
            return;
        }
        let parent_ind = parent.unwrap();
        let grandparent = self.nodes[parent_ind].parent;
        let sibling_ind = self.sibling(node);

        let succ = self.successor(Some(node));

        if let None = grandparent {
            trace!("removed node had no grandparent, sibling is now root");
            self.nodes[sibling_ind].parent = None;
            self.root = Some(sibling_ind);
            return;
        }
        let grandparent_ind = grandparent.unwrap();

        if self.nodes[grandparent_ind].left_child == parent {
            self.nodes[grandparent_ind].left_child = Some(sibling_ind);
        } else if self.nodes[grandparent_ind].right_child == parent {
            self.nodes[grandparent_ind].right_child = Some(sibling_ind);
        } else { panic!("broken tree!"); }

        self.nodes[sibling_ind].parent = grandparent;

        if let Some(succ_ind) = succ {
            let new_pred = self.predecessor(Some(succ_ind));
            self.nodes[succ_ind].segment = self.nodes[new_pred.unwrap()].segment;
        }
    }
    fn sibling(&self, node: usize) -> usize {
        if !self.is_leaf(node) { panic!("tried to get sibling of non-leaf node!"); }
        let parent = self.nodes[node].parent;
        if let None = parent { panic!("tried to get sibling of root node!"); }
        let parent_ind = parent.unwrap();

        if self.nodes[parent_ind].right_child == Some(node) { return self.nodes[parent_ind].left_child.unwrap(); }
        else if self.nodes[parent_ind].left_child == Some(node) { return self.nodes[parent_ind].right_child.unwrap(); }
        else { panic!("broken tree!"); }
    }
    pub fn tree_minimum(&self, root: usize) -> usize {
        let mut current_node = root;
        while let Some(left) = self.nodes[current_node].left_child {
            current_node = left;
        }
        current_node
    }
    pub fn tree_maximum(&self, root: usize) -> usize {
        let mut current_node = root;
        while let Some(right) = self.nodes[current_node].right_child {
            current_node = right;
        }
        current_node
    }
    fn successor(&self, node: Option<usize>) -> Option<usize> {
        if let None = node { return None; }
        let node = node.unwrap();

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
    fn predecessor(&self, node: Option<usize>) -> Option<usize> {
        if let None = node { return None; }
        let node = node.unwrap();

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
    fn left_neighbor(&self, node: Option<usize>) -> Option<usize> {
        if let None = node { return None; }
        let node = node.unwrap();

        if !self.is_leaf(node) { panic!("tried to get neighbor of non-leaf node!"); }
        let pred = self.predecessor(Some(node));
        self.predecessor(pred)
    }
    fn right_neighbor(&self, node: Option<usize>) -> Option<usize> {
        if let None = node { return None; }
        let node = node.unwrap();

        if !self.is_leaf(node) { panic!("tried to get neighbor of non-leaf node!"); }
        let succ = self.successor(Some(node));
        self.successor(succ)
    }
    pub fn get_lower_segments(&self, pt: Point) -> HashSet<Segment> {
        trace!("get_lower_segments for {}", pt);
        let mut lower_segs = HashSet::new();
        if self.is_empty() { return lower_segs; }
        let mut current_node = Some(self.tree_minimum(self.root.unwrap()));

        while let Some(current_node_ind) = current_node {
            let this_seg = self.nodes[current_node_ind].segment;
            current_node = self.right_neighbor(current_node);
            if get_lower_point(this_seg) != pt { continue; }
            trace!("{} to {} is a lower segment for {}", this_seg[0], this_seg[1], pt);
            lower_segs.insert(this_seg);
        }

        lower_segs
    }
    pub fn get_container_segments(&self, pt: Point) -> HashSet<Segment> {
        trace!("get_container_segments for {}", pt);
        let mut container_segs = HashSet::new();
        if self.is_empty() { return container_segs; }
        let mut current_node = Some(self.tree_minimum(self.root.unwrap()));

        while let Some(current_node_ind) = current_node {
            let this_seg = self.nodes[current_node_ind].segment;
            current_node = self.right_neighbor(current_node);
            if !seg_contains_pt(this_seg, pt) { continue; }
            trace!("{} to {} is a container segment for {}", this_seg[0], this_seg[1], pt);
            container_segs.insert(this_seg);
        }

        container_segs
    }
    pub fn remove_all(&mut self, segs: HashSet<Segment>) {
        for seg in segs { self.remove_segment(seg); }
    }
    pub fn insert_all(&mut self, segs: HashSet<Segment>) {
        for seg in segs { self.insert_segment(seg); }
    }
    pub fn pt_left_neighbor(&self, pt: Point) -> Option<Segment> {
        let mut current_node = self.root;

        loop {
            if let None = current_node { return None; }
            let current_node_ind = current_node.unwrap();
            let current_seg = self.nodes[current_node_ind].segment;

            if self.is_leaf(current_node_ind) {
                if pt_is_right_of_seg(pt, current_seg, self.y_line) { return Some(current_seg); }
                else {
                    let seg_neighbor = self.left_neighbor(Some(current_node_ind));
                    if let None = seg_neighbor { return None; }
                    let seg_neighbor_ind = seg_neighbor.unwrap();
                    return Some(self.nodes[seg_neighbor_ind].segment);
                }
            }

            if pt_is_right_of_seg(pt, current_seg, self.y_line) {
                current_node = self.nodes[current_node_ind].right_child;
            } else {
                current_node = self.nodes[current_node_ind].left_child;
            }
        }
    }
    pub fn pt_right_neighbor(&self, pt: Point) -> Option<Segment> {
        let mut current_node = self.root;

        loop {
            if let None = current_node { return None; }
            let current_node_ind = current_node.unwrap();
            let current_seg = self.nodes[current_node_ind].segment;

            if self.is_leaf(current_node_ind) {
                if !pt_is_right_of_seg(pt, current_seg, self.y_line) { return Some(current_seg); }
                else {
                    let seg_neighbor = self.right_neighbor(Some(current_node_ind));
                    if let None = seg_neighbor { return None; }
                    let seg_neighbor_ind = seg_neighbor.unwrap();
                    return Some(self.nodes[seg_neighbor_ind].segment);
                }
            }

            if pt_is_right_of_seg(pt, current_seg, self.y_line) {
                current_node = self.nodes[current_node_ind].right_child;
            } else {
                current_node = self.nodes[current_node_ind].left_child;
            }
        }
    }
    pub fn segment_left_neighbor(&self, seg: Option<Segment>) -> Option<Segment> {
        if let None = seg { return None; }
        let seg = seg.unwrap();

        let seg_node = self.search(seg);
        let left_node = self.left_neighbor(seg_node);

        if let None = left_node { return None; }
        let left_node = left_node.unwrap();

        Some(self.nodes[left_node].segment)
    }
    pub fn segment_right_neighbor(&self, seg: Option<Segment>) -> Option<Segment> {
        if let None = seg { return None; }
        let seg = seg.unwrap();

        let seg_node = self.search(seg);
        let right_node = self.right_neighbor(seg_node);

        if let None = right_node { return None; }
        let right_node = right_node.unwrap();

        Some(self.nodes[right_node].segment)
    }
    pub fn leftmost_of(&self, segs: HashSet<Segment>) -> Option<Segment> {
        if segs.is_empty() { return None; }
        let mut segs_vec = segs.iter().cloned().collect::<Vec<_>>();
        segs_vec.sort_by(|a, b| get_segment_x(*a, self.y_line).cmp(&get_segment_x(*b, self.y_line)));
        Some(segs_vec[0])
    }
    pub fn rightmost_of(&self, segs: HashSet<Segment>) -> Option<Segment> {
        if segs.is_empty() { return None; }
        let mut segs_vec = segs.iter().cloned().collect::<Vec<_>>();
        segs_vec.sort_by(|a, b| get_segment_x(*b, self.y_line).cmp(&get_segment_x(*a, self.y_line)));
        Some(segs_vec[0])
    }
}

struct SweepNode {
    parent: Option<usize>,
    left_child: Option<usize>,
    right_child: Option<usize>,
    segment: Segment,
}

impl fmt::Display for SweepNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p: {:?}, l: {:?}, r: {:?}, segment: {} to {}", self.parent, self.left_child, self.right_child, self.segment[0], self.segment[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_segment_order() {
        // let _ = env_logger::init();
        trace!("\n\n");

        let line1 = [Point::new(0.0, 1.0), Point::new(2.0, 1.0)];
        let line2 = [Point::new(1.0, 0.0), Point::new(1.0, 2.0)];

        let mut sweepline = SweepLine::new();
        sweepline.y_line = 1.0;
        sweepline.insert_segment(line1);
        sweepline.insert_segment(line2);
        trace!("Sweepline: {}", sweepline);
        let right_ind = sweepline.nodes[sweepline.root.unwrap()].right_child.unwrap();
        assert!(sweepline.nodes[right_ind].segment == line1);
    }
    #[test]
    fn slanted_segment_order_1() {
        // let _ = env_logger::init();
        trace!("\n\n");

        let line1 = [Point::new(0.0, 0.0), Point::new(2.0, 2.0)];
        let line2 = [Point::new(1.0, 0.0), Point::new(1.0, 2.0)];

        let mut sweepline = SweepLine::new();
        sweepline.y_line = 2.0;
        sweepline.insert_segment(line1);
        sweepline.insert_segment(line2);
        trace!("Sweepline: {}", sweepline);
        let right_ind = sweepline.nodes[sweepline.root.unwrap()].right_child.unwrap();
        assert!(sweepline.nodes[right_ind].segment == line1);
    }
    #[test]
    fn slanted_segment_order_2() {
        // let _ = env_logger::init();
        trace!("\n\n");

        let line1 = [Point::new(0.0, 0.0), Point::new(2.0, 2.0)];
        let line2 = [Point::new(1.0, 0.0), Point::new(1.0, 2.0)];

        let mut sweepline = SweepLine::new();
        sweepline.y_line = 1.0;
        sweepline.insert_segment(line1);
        sweepline.insert_segment(line2);
        trace!("Sweepline: {}", sweepline);
        let right_ind = sweepline.nodes[sweepline.root.unwrap()].right_child.unwrap();
        assert!(sweepline.nodes[right_ind].segment == line2);
    }
    #[test]
    fn slanted_segment_order_3() {
        // let _ = env_logger::init();
        trace!("\n\n");

        let line1 = [Point::new(0.0, 0.0), Point::new(2.0, 2.0)];
        let line2 = [Point::new(1.0, 0.0), Point::new(1.0, 2.0)];

        let mut sweepline = SweepLine::new();
        sweepline.y_line = 0.5;
        sweepline.insert_segment(line1);
        sweepline.insert_segment(line2);
        trace!("Sweepline: {}", sweepline);
        let right_ind = sweepline.nodes[sweepline.root.unwrap()].right_child.unwrap();
        assert!(sweepline.nodes[right_ind].segment == line2);
    }

    #[test]
    fn vertical_get_x() {
        trace!("\n\n");

        let seg = [Point::new(0.0, 0.0), Point::new(0.0, 1.0)];

        assert!(get_segment_x(seg, 0.).into_inner() == 0.0);
    }
}