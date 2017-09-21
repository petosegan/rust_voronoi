use point::Point;
use std::fmt;
type Segment = [Point; 2];

pub struct SweepLine {
    nodes: Vec<SweepNode>
}

impl fmt::Display for SweepLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut line_disp = String::new();

        for (index, node) in self.nodes.iter().enumerate() {
            line_disp.push_str(format!("{}: {}", index, node).as_str());
            line_disp.push_str("\n");
        }

        write!(f, "\n{}", line_disp)
    }
}

impl SweepLine {
    pub fn new() -> Self {
        SweepLine { nodes: vec![] }
    }
    pub fn get_lower_segments(&self, pt: Point) -> Vec<Segment> {
        unimplemented!();
    }
    pub fn get_container_segments(&self, pt: Point) -> Vec<Segment> { 
        unimplemented!();
    }
    pub fn remove_all(&mut self, segs: Vec<Segment>) {
        unimplemented!();
    }
    pub fn insert_all(&mut self, segs: Vec<Segment>) {
        unimplemented!();
    }
    pub fn pt_left_neighbor(&self, pt: Point) -> Segment {
        unimplemented!();
    }
    pub fn pt_right_neighbor(&self, pt: Point) -> Segment {
        unimplemented!();
    }
    pub fn left_neighbor(&self, seg: Segment) -> Segment {
        unimplemented!();
    }
    pub fn right_neighbor(&self, seg: Segment) -> Segment {
        unimplemented!();
    }
    pub fn leftmost_of(&self, segs: Vec<Segment>) -> Segment {
        unimplemented!();
    }
    pub fn rightmost_of(&self, segs: Vec<Segment>) -> Segment {
        unimplemented!();
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