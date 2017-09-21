use point::Point;

type Segment = [Point; 2];

struct SegmentQueue {
    segments: Vec<SegmentEvent>
}

struct SegmentEvent {
    point: Point,
    segment: 
}

pub fn all_intersections(segments: Vec<Segment>) -> Vec<(Point, Vec<Segment>)> {
    let mut queue = SegmentQueue::new();
    let mut sweepline = SweepLine::new();
    let mut result = vec![];

    for segment in segments {
        queue.push(segment);
    }

    while !queue.is_empty() {
        let this_event = queue.pop();
        let this_intersection = handle_event_point(this_event.point, &mut sweepline, &mut queue);
        if let Some(this_intersection) = this_intersection {
            result.push(this_intersection);
        }
    }
}

fn handle_event_point(pt: Point, sweepline: &mut SweepLine, queue: &mut queue) -> Option<(Point, Vec<Segment>)>{
    let upper = sweepline.get_upper_segments(pt);
    let lower = sweepline.get_lower_segments(pt);
    let container = sweepline.get_container_segments(pt);
    let result = None;

    let all_segs = vec_union_3(upper, lower, container);
    if all_segs.len() > 1 {
        result = (pt, all_segs);
    }

    let lc_segs = vec_union_2(lower, container);
    let uc_segs = vec_union_2(upper, container);

    sweepline.remove_all(lc_segs);
    sweepline.insert_all(uc_segs);

    if uc_segs.is_empty() {
        let sl = sweepline.left_neighbor(pt);
        let sr = sweepline.right_neighbor(pt);
        find_new_events(sl, sr, pt, queue);
    } else {
        let s_prime = sweepline.leftmost_of(uc_segs);
        let sl = sweepline.left_neighbor(s_prime);
        find_new_events(sl, s_prime, pt, queue);
        let s_pprime = sweepline.rightmost_of(uc_segs);
        let sr = sweepline.right_neighbor(s_pprime);
        find_new_events(s_pprime, sr, pt, queue);
    }
}

fn find_new_events(s1: Segment, s2: Segment, pt: Point, queue: &mut SegmentQueue) {
    if let Some(intersection) = segment_intersection(s1, s2) {
        if intersection.y() < pt.y() || (intersection.y() == pt.y() && intersection.x() > pt.x()) {
            if !queue.contains(intersection) {
                queue.insert(intersection);
            }
        }
    }
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