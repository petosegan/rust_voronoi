use point::Point;
use voronoi::voronoi;
use dcel::{makes_left_turn, add_faces, make_polygons};

fn is_convex(pts: &Vec<Point>) -> bool {
	let num_pts = pts.len();
	if num_pts < 3 { return false; }

	let mut right_turns = vec![];
	for index in 0..(num_pts - 2) {
		right_turns.push(makes_left_turn(pts[index], pts[index+1], pts[index+2]));
	}
	right_turns.push(makes_left_turn(pts[num_pts-2], pts[num_pts-1], pts[0]));
	right_turns.push(makes_left_turn(pts[num_pts-1], pts[0], pts[1]));
	return right_turns.iter().all(|&x| x) || !right_turns.iter().any(|&x| x);
}

fn polygon_centroid(pts: &Vec<Point>) -> Point {
	let mut pt_sum = Point::new(0.0, 0.0);
	for pt in pts {
		pt_sum = *pt + pt_sum;
	}
	pt_sum * (1.0 / (pts.len() as f64))
}

pub fn lloyd_relaxation(pts: Vec<Point>, box_size: f64) -> Vec<Point> {
	let mut voronoi = voronoi(pts, box_size);
	add_faces(&mut voronoi);
	let mut faces = make_polygons(&voronoi);
	faces.iter().map(polygon_centroid).collect::<Vec<Point>>()
}