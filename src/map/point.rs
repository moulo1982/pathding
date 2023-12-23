use std::cell::RefCell;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type PointType = Arc<RwLock<RefCell<Point>>>;

#[derive(Debug, Clone, Default)]
pub struct Point {
    pub x: i64,
    pub y: i64,

    pub parent: Option<PointType>,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Eq for Point {
}

impl Point {

    pub fn new(x: i64, y:i64) -> Self {
        Point{x, y, parent:None}
    }

    pub fn with_more_details(x: i64, y:i64, parent: Option<PointType>) -> Self {
        Point{x, y, parent}
    }

    pub fn into_rc(self) -> PointType {
        Arc::new(RwLock::new(RefCell::new(self)))
    }

    pub fn f(&self, start: &Point, end: &Point) -> i64 {
        self.g(start) + self.h(end)
    }

    pub fn g(&self, start: &Point) -> i64 {
        (self.x - start.x).abs() + (self.y - start.y).abs()
    }

    pub fn h(&self, end: &Point) -> i64 {
        (end.x - self.x).abs() + (end.y - self.y).abs()
    }

    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::with_more_details(self.x - 1, self.y, None),
            Point::with_more_details(self.x + 1, self.y, None),
            Point::with_more_details(self.x, self.y - 1, None),
            Point::with_more_details(self.x, self.y + 1, None),
        ]
    }

    pub fn set_parent(&mut self, parent: PointType) {
        self.parent = Some(parent)
    }
}

impl PartialEq<PointType> for Point {
    fn eq(&self, other: &PointType) -> bool {
        self.x == other.try_read().unwrap().borrow().x && self.y == other.try_read().unwrap().borrow().y
    }
}

impl PartialEq<Point> for PointType {
    fn eq(&self, other: &Point) -> bool {
        other == self
    }
}