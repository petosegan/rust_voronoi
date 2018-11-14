use rand::{Rand, Rng, random};
use std::ops::{Sub, Mul, Add};
use std::fmt;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

/// A point in two dimensions
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Point {
    /// x coordinate
    pub x: OrderedFloat<f64>,
    /// y coordinate
    pub y: OrderedFloat<f64>
}

impl Point {
    /// Constructs a new `Point`.
    pub fn new(x: f64, y: f64) -> Self {
        Point {x: OrderedFloat::<f64>(x), y: OrderedFloat::<f64>(y)}
    }

    /// Getter for the x coordinate.
    pub fn x(&self) -> f64 {
        self.x.into_inner()
    }

    /// Getter for the y coordinate.
    pub fn y(&self) -> f64 {
        self.y.into_inner()
    }
}

impl fmt::Debug for Point {
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

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, _rhs: Point) -> Point {
        Point::new(self.x() - _rhs.x(), self.y() - _rhs.y())
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point::new(self.x() + _rhs.x(), self.y() + _rhs.y())
    }
}

impl Point {
    /// Computes the cross product of two points, viewed as vectors from the origin.
    pub fn cross(self, rhs: Point) -> f64 {
        self.x() * rhs.y() - self.y() * rhs.x()
    }

    /// Computes the dot product of two points, viewed as vectors from the origin.
    pub fn dot(self, rhs: Point) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y > other.y { return Ordering::Greater; }
        else if self.y == other.y {
            if self.x < other.x { return Ordering::Greater; }
            else if self.x == other.x { return Ordering::Equal; }
            else { return Ordering::Less; }
        } else { return Ordering::Less; }
    }
}
