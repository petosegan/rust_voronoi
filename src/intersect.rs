use point::Point;

type Segment = [Point; 2];

pub fn all_intersections(segments: Vec<Segment>) -> Vec<Point> {
    unimplemented!();
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_lines_intersect() {
    	let line1 = [Point::new(-1.0, 0.0), Point::new(1.0, 0.0)];
        let line2 = [Point::new(0.0, -1.0), Point::new(0.0, 1.0)];
        assert_eq!(segment_intersection(line1, line2), Some(Point::new(0.0, 0.0)));
    }

    #[test]
    fn simple_lines_nonintersect() {
        let line1 = [Point::new(-1.0, 10.0), Point::new(1.0, 10.0)];
        let line2 = [Point::new(0.0, -1.0), Point::new(0.0, 1.0)];
        assert_eq!(segment_intersection(line1, line2), None);
    }

    #[test]
    fn multiple_lines_intersect() {
        let line1 = [Point::new(1.0, 0.0), Point::new(1.0, 3.0)];
        let line2 = [Point::new(2.0, 0.0), Point::new(2.0, 3.0)];
        let line3 = [Point::new(0.0, 1.0), Point::new(3.0, 1.0)];
        let line4 = [Point::new(0.0, 2.0), Point::new(3.0, 2.0)];

        let intersections = all_intersections(vec![line1, line2, line3, line4]);

        assert!(intersections.contains(&Point::new(1.0, 1.0)));
        assert!(intersections.contains(&Point::new(2.0, 2.0)));
        assert!(intersections.contains(&Point::new(1.0, 2.0)));
        assert!(intersections.contains(&Point::new(2.0, 1.0)));
        assert!(intersections.len() == 4);
    }

    #[test]
    fn crossed_lines_intersect() {
        let line1 = [Point::new(0.0, 0.0), Point::new(4.0, 4.0)];
        let line2 = [Point::new(2.0, 0.0), Point::new(0.0, 2.0)];
        let line3 = [Point::new(4.0, 0.0), Point::new(0.0, 4.0)];
        let line4 = [Point::new(4.0, 2.0), Point::new(2.0, 4.0)];

        let intersections = all_intersections(vec![line1, line2, line3, line4]);

        assert!(intersections.contains(&Point::new(1.0, 1.0)));
        assert!(intersections.contains(&Point::new(2.0, 2.0)));
        assert!(intersections.contains(&Point::new(3.0, 3.0)));
        assert!(intersections.len() == 3);
    }
}