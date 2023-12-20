use std::cell::RefCell;
use std::rc::Rc;

pub type PointType = Rc<RefCell<Point>>;

#[derive(Debug, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,

    pub parent: Option<PointType>,
}

impl Point {

    pub fn new(x: i64, y:i64) -> Self {
        Point{x, y, parent:None}
    }

    pub fn with_more_details(x: i64, y:i64, parent: Option<PointType>) -> Self {
        Point{x, y, parent}
    }

    pub fn into_rc(self) -> PointType {
        Rc::new(RefCell::new(self))
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
