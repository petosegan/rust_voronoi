use ordered_float::OrderedFloat;
use point::Point;
use beachline::BreakPoint;

type TripleSite = (Point, Point, Point);

pub type Segment = [Point; 2];

pub fn segment_intersection(seg1: Segment, seg2: Segment) -> Option<Point> {
    let a = seg1[0];
    let c = seg2[0];
    let r = seg1[1] - a;
    let s = seg2[1] - c;

    let denom = r.cross(s);
    if denom == 0.0 { return None; }

    let numer_a = (c - a).cross(s);
    let numer_c = (c - a).cross(r);

    let t = numer_a / denom;
    let u = numer_c / denom;

    if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 { return None; }

    return Some(a + r * t);
}

pub fn circle_bottom(triple_site: TripleSite) -> OrderedFloat<f64> {
	let circle_center = circle_center(triple_site);
	let (_, _, p3) = triple_site;
	let x3 = p3.x();
	let y3 = p3.y();
	let x_cen = circle_center.x();
	let y_cen = circle_center.y();

	let r = ((x3 - x_cen) * (x3 - x_cen) + (y3 - y_cen) * (y3 - y_cen)).sqrt();

	return OrderedFloat::<f64>(y_cen - r);
}

// TODO: handle all the special cases
pub fn circle_center(triple_site: TripleSite) -> Point {
	// trace!("Finding center for {}, {}, {}", triple_site.0, triple_site.1, triple_site.2);
	let (p1, p2, p3) = triple_site;
	let x1 = p1.x();
	let x2 = p2.x();
	let x3 = p3.x();
	let y1 = p1.y();
	let y2 = p2.y();
	let y3 = p3.y();

	let c1 = x3 * x3 + y3 * y3 - x1 * x1 - y1 * y1;
	let c2 = x3 * x3 + y3 * y3 - x2 * x2 - y2 * y2;
	let a1 = -2. * (x1 - x3);
	let a2 = -2. * (x2 - x3);
	let b1 = -2. * (y1 - y3);
	let b2 = -2. * (y2 - y3);

	let numer = c1 * a2 - c2 * a1;
	let denom = b1 * a2 - b2 * a1;

	let y_cen = numer / denom;

	let x_cen = (c2 - b2 * y_cen) / a2;
	// trace!("center at {}, {}", x_cen, y_cen);

	return Point::new(x_cen, y_cen);
}

// see http://www.kmschaal.de/Diplomarbeit_KevinSchaal.pdf, pg 27
pub fn breakpoints_converge(triple_site: TripleSite) -> bool {
	let (a, b, c) = triple_site;
	let ax = a.x();
	let ay = a.y();
	let bx = b.x();
	let by = b.y();
	let cx = c.x();
	let cy = c.y();

	(ay - by) * (bx - cx) > (by - cy) * (ax - bx)
}

// TODO: cover py1 = py2 case, and py1 = yl
pub fn get_breakpoint_x(bp: &BreakPoint, yl: f64) -> f64 {
	let ax = bp.left_site.x();
	let bx = bp.right_site.x();
	let ay = bp.left_site.y();
	let by = bp.right_site.y();

	// shift frames
	let bx_s = bx - ax;
	let ay_s = ay - yl;
	let by_s = by - yl;

	let discrim = ay_s * by_s * ((ay_s - by_s) * (ay_s - by_s) + bx_s * bx_s);
	let numer = ay_s * bx_s - discrim.sqrt();
	let denom = ay_s - by_s;

	let mut x_bp = numer / denom;
	x_bp += ax; // shift back to original frame

	return x_bp;
}

// TODO: handle py == yl case
pub fn get_breakpoint_y(bp: &BreakPoint, yl: f64) -> f64 {
	let px = bp.left_site.x();
	let py = bp.left_site.y();

	let bp_x = get_breakpoint_x(bp, yl);

	let numer = (px - bp_x) * (px - bp_x);
	let denom = 2. * (py - yl);

	return numer / denom + (py + yl) / 2.;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_circle_center() {
    	let circle_triple = (Point::new(-1.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 0.0));
        assert_eq!(circle_center(circle_triple), Point::new(0.0, 0.0));
    }

    #[test]
    fn simple_circle_bottom() {
    	let circle_triple = (Point::new(-1.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 0.0));
    	assert_eq!(circle_bottom(circle_triple), OrderedFloat(-1.0));
    }
}