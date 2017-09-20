use std::fmt;
use point::Point;
use geometry::get_breakpoint_x;

const NIL: usize = !0;
type TripleSite = (Point, Point, Point);

#[derive(Debug)]
pub struct BeachLine {
	pub nodes: Vec<BeachNode>,
	pub y_line: f64,
	pub root: usize,
}

impl fmt::Display for BeachLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut beachline_disp = String::new();

        for (index, node) in self.nodes.iter().enumerate() {
            beachline_disp.push_str(format!("{}: {}", index, node).as_str());
            beachline_disp.push_str("\n");
        }

        write!(f, "\nRoot: {}\ny_line: {}\n{}", self.root, self.y_line, beachline_disp)
    }
}

#[derive(Debug)]
pub struct BeachNode {
	pub parent: Option<usize>,
	pub left_child: Option<usize>,
	pub right_child: Option<usize>,
	pub item: BeachItem,
}

impl fmt::Display for BeachNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p: {:?}, l: {:?}, r: {:?}, item: {}", self.parent, self.left_child, self.right_child, self.item)
    }
}

impl BeachNode {
	fn make_root(item: BeachItem) -> Self {
		BeachNode { parent: None, left_child: None, right_child: None, item: item}
	}
	pub fn make_arc(parent: Option<usize>, item: BeachItem) -> Self {
		if let BeachItem::Leaf(_) = item {
			BeachNode { parent: parent, left_child: None, right_child: None, item: item}
		} else {
			panic!("make_arc can only make Leaf items!");
		}
	}
}

#[derive(Debug)]
pub enum BeachItem {
	Leaf(Arc),
	Internal(BreakPoint),
}

impl fmt::Display for BeachItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match *self {
        	BeachItem::Leaf(ref arc) => write!(f, "Leaf: {}", arc),
        	BeachItem::Internal(ref bp) => write!(f, "Internal: {}", bp),
        }
    }
}

#[derive(Debug)]
pub struct Arc {
	pub site: Point,
	pub site_event: Option<usize>, // index to circle event in EventQueue
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "site: {}, site_event: {:?}", self.site, self.site_event)
    }
}

#[derive(Debug)]
pub struct BreakPoint {
	pub left_site: Point,
	pub right_site: Point,
	pub halfedge: usize, // index of halfedge
}

impl fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "left: {}, right: {}, halfedge: {}", self.left_site, self.right_site, self.halfedge)
    }
}

impl BeachLine {
	pub fn new() -> Self {
		BeachLine { nodes: vec![], y_line: 0.0, root: NIL }
	}
	pub fn is_empty(&self) -> bool {
		self.nodes.is_empty()
	}
	pub fn insert_point(&mut self, pt: Point) {
		let this_arc = Arc {site: pt, site_event: None};
		let this_item = BeachItem::Leaf(this_arc);
		let this_node = BeachNode::make_root(this_item);
		self.nodes.push(this_node);
		self.root = self.nodes.len() - 1;
	}
	pub fn get_arc_above(&self, pt: Point) -> usize {
		if self.is_empty() { panic!("can't get_arc_above on empty beachline!"); }
		let mut current_node = self.root;
		loop {
			match self.nodes[current_node].item {
				BeachItem::Leaf(_) => { return current_node; }
				BeachItem::Internal(ref breakpoint) => {
					let x_bp = get_breakpoint_x(breakpoint, pt.y());
					if pt.x() < x_bp { current_node = self.nodes[current_node].left_child.unwrap(); }
					else { current_node = self.nodes[current_node].right_child.unwrap(); }
				}
			}
		}
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
	pub fn successor(&self, node: usize) -> Option<usize> {
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
	pub fn predecessor(&self, node: usize) -> Option<usize> {
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
	pub fn get_left_arc(&self, node: Option<usize>) -> Option<usize> {
		if let None = node { return None; }
		let node = node.unwrap();
		if let Some(left) = self.predecessor(node) {
			self.predecessor(left)
		} else {
			None
		}
	}
	pub fn get_right_arc(&self, node: Option<usize>) -> Option<usize> {
		if let None = node { return None; }
		let node = node.unwrap();
		if let Some(right) = self.successor(node) {
			self.successor(right)
		} else {
			None
		}
	}
	pub fn get_leftward_triple(&self, node: usize) -> Option<TripleSite> {
		let left_arc = self.get_left_arc(Some(node));
		let left_left_arc = self.get_left_arc(left_arc);

		let this_site = self.get_site(Some(node));
		let left_site = self.get_site(left_arc);
		let left_left_site = self.get_site(left_left_arc);

		if this_site.is_some() && left_site.is_some() && left_left_site.is_some() {
			return Some((left_left_site.unwrap(), left_site.unwrap(), this_site.unwrap()));
		} else { return None; }
	}
	pub fn get_rightward_triple(&self, node: usize) -> Option<TripleSite> {
		let right_arc = self.get_right_arc(Some(node));
		let right_right_arc = self.get_right_arc(right_arc);

		let this_site = self.get_site(Some(node));
		let right_site = self.get_site(right_arc);
		let right_right_site = self.get_site(right_right_arc);

		if this_site.is_some() && right_site.is_some() && right_right_site.is_some() {
			return Some((this_site.unwrap(), right_site.unwrap(), right_right_site.unwrap()));
		} else { return None; }
	}
	pub fn get_centered_triple(&self, node: usize) -> Option<TripleSite> {
		let right_arc = self.get_right_arc(Some(node));
		let left_arc = self.get_left_arc(Some(node));

		let this_site = self.get_site(Some(node));
		let right_site = self.get_site(right_arc);
		let left_site = self.get_site(left_arc);

		if this_site.is_some() && right_site.is_some() && left_site.is_some() {
			return Some((left_site.unwrap(), this_site.unwrap(), right_site.unwrap()));
		} else { return None; }
	}
	pub fn get_site(&self, node: Option<usize>) -> Option<Point> {
		if let None = node { return None; }
		let node = node.unwrap();
		if let BeachItem::Leaf(ref arc) = self.nodes[node].item {
			return Some(arc.site);
		} else {
			return None;
		}
	}
}