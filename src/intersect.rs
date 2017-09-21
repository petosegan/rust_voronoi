use point::Point;
use segment_queue::{SegmentQueue, SegmentEvent};
use sweepline::SweepLine;

type Segment = [Point; 2];

pub fn all_intersections(segments: Vec<Segment>) -> Vec<(Point, Vec<Segment>)> {
    let mut queue = SegmentQueue::new();
    let mut sweepline = SweepLine::new();
    let mut result = vec![];

    for segment in segments {
        queue.insert_seg(segment);
    }

    while !queue.is_empty() {
        let this_event = queue.pop().unwrap();
        let this_intersection = handle_event_point(this_event, &mut sweepline, &mut queue);
        if let Some(this_intersection) = this_intersection {
            result.push(this_intersection);
        }
    }
    return result;
}

fn vec_union_2(v1: Vec<Segment>, v2: Vec<Segment>) -> Vec<Segment> {
    unimplemented!();
}

fn vec_union_3(v1: Vec<Segment>, v2: Vec<Segment>, v3: Vec<Segment>) -> Vec<Segment> {
    unimplemented!();
}

fn handle_event_point(event: SegmentEvent, sweepline: &mut SweepLine, queue: &mut SegmentQueue) -> Option<(Point, Vec<Segment>)>{
    let upper = event.segments_below;
    let pt = event.point;
    let lower = sweepline.get_lower_segments(pt);
    let container = sweepline.get_container_segments(pt);
    let mut result = None;

    let all_segs = vec_union_3(upper.clone(), lower.clone(), container.clone());
    if all_segs.len() > 1 {
        result = Some((pt, all_segs));
    }

    let lc_segs = vec_union_2(lower.clone(), container.clone());
    let uc_segs = vec_union_2(upper.clone(), container.clone());

    sweepline.remove_all(lc_segs.clone());
    sweepline.insert_all(uc_segs.clone());

    if uc_segs.is_empty() {
        let sl = sweepline.pt_left_neighbor(pt);
        let sr = sweepline.pt_right_neighbor(pt);
        find_new_events(sl, sr, pt, queue);
    } else {
        let s_prime = sweepline.leftmost_of(uc_segs.clone());
        let sl = sweepline.left_neighbor(s_prime);
        find_new_events(sl, s_prime, pt, queue);
        let s_pprime = sweepline.rightmost_of(uc_segs.clone());
        let sr = sweepline.right_neighbor(s_pprime);
        find_new_events(s_pprime, sr, pt, queue);
    }
    return result;
}

fn find_new_events(s1: Segment, s2: Segment, pt: Point, queue: &mut SegmentQueue) {
    if let Some(intersection) = segment_intersection(s1, s2) {
        if intersection.y() < pt.y() || (intersection.y() == pt.y() && intersection.x() > pt.x()) {
            if !queue.contains(intersection) {
                queue.insert_pt(intersection);
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