use point::Point;
use segment_queue::{SegmentQueue, SegmentEvent};
use sweepline::SweepLine;
use std::collections::HashSet;
extern crate env_logger;

pub type Segment = [Point; 2];

pub fn get_upper_point(seg: Segment) -> Point {
    let p0 = seg[0];
    let p1 = seg[1];
    if p0 > p1 { p0 } else { p1 }
}

pub fn get_lower_point(seg: Segment) -> Point {
    let p0 = seg[0];
    let p1 = seg[1];
    if p0 > p1 { p1 } else { p0 }
}

pub fn seg_length(seg: Segment) -> f64 {
    let p0 = seg[0];
    let p1 = seg[1];

    let sqr_len = (p0.x() - p1.x()) * (p0.x() - p1.x()) + (p0.y() - p1.y()) * (p0.y() - p1.y());
    return sqr_len.sqrt();
}

pub fn all_intersections(segments: Vec<Segment>) -> Vec<(Point, Vec<Segment>)> {
    trace!("Running all_intersections");
    let mut queue = SegmentQueue::new();
    let mut sweepline = SweepLine::new();
    let mut result = vec![];

    for segment in segments {
        trace!("Inserting segment {} to {} into queue", segment[0], segment[1]);
        queue.insert_seg(segment);
    }

    while !queue.is_empty() {
        let this_event = queue.pop().unwrap();
        trace!("\n\n");
        trace!("Queue: {}", queue);
        trace!("Sweepline: {}", sweepline);
        trace!("Popped event {}", this_event);
        sweepline.y_line = this_event.point.y();
        let this_intersection = handle_event_point(this_event, &mut sweepline, &mut queue);
        if let Some(this_intersection) = this_intersection {
            result.push(this_intersection);
        }
    }
    return result;
}

fn handle_event_point(event: SegmentEvent, sweepline: &mut SweepLine, queue: &mut SegmentQueue) -> Option<(Point, Vec<Segment>)>{
    let upper = event.segments_below;
    let pt = event.point;
    let lower = sweepline.get_lower_segments(pt);
    let container = sweepline.get_container_segments(pt);
    let mut result = None;

    let lc_segs: HashSet<_> = lower.union(&container).cloned().collect();
    let uc_segs: HashSet<_>  = upper.union(&container).cloned().collect();
    let all_segs: HashSet<_>  = upper.union(&lc_segs).cloned().collect();

    if all_segs.len() > 1 {
        trace!("Found intersection at {}", pt);
        result = Some((pt, all_segs.iter().cloned().collect::<Vec<_>>()));
    }

    sweepline.remove_all(lc_segs.clone());
    // trace!("Sweepline: {}", sweepline);
    sweepline.insert_all(uc_segs.clone());

    if uc_segs.is_empty() {
        trace!("No upper or container segments for {}", pt);
        let sl = sweepline.pt_left_neighbor(pt);
        trace!("Left neighbor is {:?}", sl);
        let sr = sweepline.pt_right_neighbor(pt);
        trace!("Right neighbor is {:?}", sr);
        find_new_events(sl, sr, pt, queue);
    } else {
        trace!("Found upper or container segments for {}", pt);
        let s_prime = sweepline.leftmost_of(uc_segs.clone());
        trace!("Leftmost is {:?}", s_prime);
        let sl = sweepline.segment_left_neighbor(s_prime);
        trace!("Left neighbor of leftmost is {:?}", sl);
        find_new_events(sl, s_prime, pt, queue);
        let s_pprime = sweepline.rightmost_of(uc_segs.clone());
        trace!("Rightmost is {:?}", s_pprime);
        let sr = sweepline.segment_right_neighbor(s_pprime);
        trace!("Right neighbor of rightmost is {:?}", sr);
        find_new_events(s_pprime, sr, pt, queue);
    }
    return result;
}

fn find_new_events(s1: Option<Segment>, s2: Option<Segment>, pt: Point, queue: &mut SegmentQueue) {
    if let None = s1 { return; }
    if let None = s2 { return; }
    let s1 = s1.unwrap();
    let s2 = s2.unwrap();
    trace!("Looking for new events from segments {} to {} and {} to {}", s1[0], s1[1], s2[0], s2[1]);

    if let Some(intersection) = segment_intersection(s1, s2) {
        if intersection.y() < pt.y() || (intersection.y() == pt.y() && intersection.x() > pt.x()) {
            if !queue.contains(intersection) {
                trace!("Inserting intersection at {}", intersection);
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
    #[ignore]
    fn simple_segments_intersect() {
    	let line1 = [Point::new(-1.0, 0.0), Point::new(1.0, 0.0)];
        let line2 = [Point::new(0.0, -1.0), Point::new(0.0, 1.0)];
        assert_eq!(segment_intersection(line1, line2), Some(Point::new(0.0, 0.0)));
    }

    #[test]
    #[ignore]
    fn simple_segments_nonintersect() {
        let line1 = [Point::new(-1.0, 10.0), Point::new(1.0, 10.0)];
        let line2 = [Point::new(0.0, -1.0), Point::new(0.0, 1.0)];
        assert_eq!(segment_intersection(line1, line2), None);
    }

    #[test]
    #[ignore]
    fn simplest_lines_intersect() {
        // let _ = env_logger::init();

        debug!("Test on Multiple Intersecting Lines");
        let line1 = [Point::new(0.0, 0.0), Point::new(2.0, 2.0)];
        let line2 = [Point::new(0.0, 2.0), Point::new(2.0, 0.0)];

        let intersections = all_intersections(vec![line1, line2]);
        let mut int_pts = vec![];
        for intersection in intersections {
            int_pts.push(intersection.0)
        }

        let mut int_disp = String::new();

        for pt in &int_pts {
            int_disp.push_str(format!("{}, ", pt).as_str());
            int_disp.push_str("\n");
        }

        println!("Intersections: {}", int_disp);

        assert!(int_pts.contains(&Point::new(1.0, 1.0)));
        assert!(int_pts.len() == 1);
    }

    #[test]
    fn tee_lines_intersect() {
        // let _ = env_logger::init();

        debug!("Test on Tee Intersecting Lines");
        let line1 = [Point::new(1.0, 0.0), Point::new(1.0, 2.0)];
        let line2 = [Point::new(0.0, 1.0), Point::new(2.0, 1.0)];

        let intersections = all_intersections(vec![line1, line2]);
        let mut int_pts = vec![];
        for intersection in intersections {
            int_pts.push(intersection.0)
        }

        let mut int_disp = String::new();

        for pt in &int_pts {
            int_disp.push_str(format!("{}, ", pt).as_str());
            int_disp.push_str("\n");
        }

        println!("Intersections: {}", int_disp);

        assert!(int_pts.contains(&Point::new(1.0, 1.0)));
        assert!(int_pts.len() == 1);
    }

    #[test]
    #[ignore]
    fn doublecross_intersect() {
        // let _ = env_logger::init();

        debug!("Test on Double Cross");
        let line1 = [Point::new(1.0, 0.0), Point::new(1.0, 3.0)];
        let line2 = [Point::new(0.0, 1.0), Point::new(2.0, 1.0)];
        let line3 = [Point::new(0.0, 2.0), Point::new(2.0, 2.0)];

        let intersections = all_intersections(vec![line1, line2, line3]);
        let mut int_pts = vec![];
        for intersection in intersections {
            int_pts.push(intersection.0)
        }

        let mut int_disp = String::new();

        for pt in &int_pts {
            int_disp.push_str(format!("{}, ", pt).as_str());
            int_disp.push_str("\n");
        }

        println!("Intersections: {}", int_disp);

        assert!(int_pts.contains(&Point::new(1.0, 1.0)));
        assert!(int_pts.contains(&Point::new(1.0, 2.0)));
        assert!(int_pts.len() == 2);
    }

    #[test]
    fn h_intersect() {
        let _ = env_logger::init();

        debug!("Test on H");
        let line1 = [Point::new(1.0, 2.0), Point::new(1.0, 0.0)];
        let line2 = [Point::new(2.0, 2.0), Point::new(2.0, 0.0)];
        let line3 = [Point::new(0.0, 1.0), Point::new(3.0, 1.0)];

        let intersections = all_intersections(vec![line1, line2, line3]);
        let mut int_pts = vec![];
        for intersection in intersections {
            int_pts.push(intersection.0)
        }

        let mut int_disp = String::new();

        for pt in &int_pts {
            int_disp.push_str(format!("{}, ", pt).as_str());
            int_disp.push_str("\n");
        }

        println!("Intersections: {}", int_disp);

        assert!(int_pts.contains(&Point::new(1.0, 1.0)));
        assert!(int_pts.contains(&Point::new(2.0, 1.0)));
        assert!(int_pts.len() == 2);
    }

    #[test]
    #[ignore]
    fn multiple_lines_intersect() {
        // let _ = env_logger::init();

        debug!("Test on Multiple Intersecting Lines");
        let line1 = [Point::new(1.0, 0.0), Point::new(1.0, 3.0)];
        let line2 = [Point::new(2.0, 0.0), Point::new(2.0, 3.0)];
        let line3 = [Point::new(0.0, 1.0), Point::new(3.0, 1.0)];
        let line4 = [Point::new(0.0, 2.0), Point::new(3.0, 2.0)];

        let intersections = all_intersections(vec![line1, line2, line3, line4]);
        let mut int_pts = vec![];
        for intersection in intersections {
            int_pts.push(intersection.0)
        }

        let mut int_disp = String::new();

        for pt in &int_pts {
            int_disp.push_str(format!("{}, ", pt).as_str());
            int_disp.push_str("\n");
        }

        println!("Intersections: {}", int_disp);

        assert!(int_pts.contains(&Point::new(1.0, 1.0)));
        assert!(int_pts.contains(&Point::new(2.0, 2.0)));
        assert!(int_pts.contains(&Point::new(1.0, 2.0)));
        assert!(int_pts.contains(&Point::new(2.0, 1.0)));
        assert!(int_pts.len() == 4);
    }

    #[test]
    #[ignore]
    fn crossed_lines_intersect() {
        let line1 = [Point::new(0.0, 0.0), Point::new(4.0, 4.0)];
        let line2 = [Point::new(2.0, 0.0), Point::new(0.0, 2.0)];
        let line3 = [Point::new(4.0, 0.0), Point::new(0.0, 4.0)];
        let line4 = [Point::new(4.0, 2.0), Point::new(2.0, 4.0)];

        let intersections = all_intersections(vec![line1, line2, line3, line4]);
        let mut int_pts = vec![];
        for intersection in intersections {
            int_pts.push(intersection.0)
        }

        let mut int_disp = String::new();

        for pt in &int_pts {
            int_disp.push_str(format!("{}, ", pt).as_str());
            int_disp.push_str("\n");
        }

        println!("Intersections: {}", int_disp);

        assert!(int_pts.contains(&Point::new(1.0, 1.0)));
        assert!(int_pts.contains(&Point::new(2.0, 2.0)));
        assert!(int_pts.contains(&Point::new(3.0, 3.0)));
        assert!(int_pts.len() == 3);
    }
}