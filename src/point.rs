use rand::{Rand, Rng, random};
use std::ops::Mul;
use std::fmt;
use ordered_float::OrderedFloat;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
	pub x: OrderedFloat<f64>,
	pub y: OrderedFloat<f64>
}

impl Point {
	pub fn new(x: f64, y: f64) -> Self {
		Point {x: OrderedFloat::<f64>(x), y: OrderedFloat::<f64>(y)}
	}
	pub fn x(&self) -> f64 {
		self.x.into_inner()
	}
	pub fn y(&self) -> f64 {
		self.y.into_inner()
	}
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0:.1}, {1:.1})", self.x(), self.y())
    }
}

#[allow(unused_variables)]
impl Rand for Point {
	fn rand<R: Rng>(rng: &mut R) -> Point {
		Point::new(random::<f64>(), random::<f64>())
	}
}

impl Mul<f64> for Point {
	type Output = Point;

	fn mul(self, _rhs: f64) -> Point {
		Point::new(self.x.into_inner() * _rhs, self.y.into_inner() * _rhs)
	}
}