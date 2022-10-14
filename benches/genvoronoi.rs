#![feature(test)]

extern crate test;
extern crate rand;
extern crate voronoi;

use rand::{Rng, thread_rng};
use voronoi::{voronoi, Point};

const BOX_SIZE: f64 = 800.;


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn generate_points(count: usize) -> Vec<Point> {
        let mut vec = Vec::with_capacity(count);
        let mut rng = thread_rng();

        for _ in 0..count {
            vec.push(Point::new(rng.next_f64() * BOX_SIZE, rng.next_f64() * BOX_SIZE));
        }

        vec
    }

    #[bench]
    fn bench_1_point(b: &mut Bencher) {
        let points = vec![Point::new(0.0, 1.0)];

        b.iter(|| {
            voronoi(points.clone(), (BOX_SIZE, BOX_SIZE));
        });
    }

    #[bench]
    fn bench_100_points(b: &mut Bencher) {
        let points = generate_points(100);

        b.iter(|| {
            voronoi(points.clone(), (BOX_SIZE, BOX_SIZE));
        });
    }


    #[bench]
    fn bench_10000_points(b: &mut Bencher) {
        let points = generate_points(10000);

        b.iter(|| {
            voronoi(points.clone(), (BOX_SIZE, BOX_SIZE));
        });
    }
}
