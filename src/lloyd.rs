use point::Point;
use voronoi::voronoi;
use dcel::make_polygons;

/// Computes the centroid of a polygon.
pub fn polygon_centroid(pts: &Vec<Point>) -> Point {
    let mut pt_sum = Point::new(0.0, 0.0);
    for pt in pts {
        pt_sum = *pt + pt_sum;
    }
    pt_sum * (1.0 / (pts.len() as f64))
}

/// Produces the Lloyd Relaxation of a set of points.
///
/// Each point is moved to the centroid of its Voronoi cell.
pub fn lloyd_relaxation(pts: Vec<Point>, box_size: f64) -> Vec<Point> {
    let voronoi = voronoi(pts, box_size);
    let faces = make_polygons(&voronoi);
    faces.iter().map(polygon_centroid).collect::<Vec<Point>>()
}
