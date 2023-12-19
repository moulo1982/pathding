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

    pub fn new(x: i64, y:i64) -> PointType {
        Rc::new(RefCell::new(Point{x, y, parent:None}))
    }

    pub fn with_more_details(x: i64, y:i64, parent: Option<PointType>) -> PointType {
        Rc::new(RefCell::new(Point{x, y, parent}))
    }

    pub fn f(&self, start: &PointType, end: &PointType) -> i64 {
        self.g(start) + self.h(end)
    }

    pub fn g(&self, start: &PointType) -> i64 {
        (self.x - start.borrow().x).abs() + (self.y - start.borrow().y).abs()
    }

    pub fn h(&self, end: &PointType) -> i64 {
        (end.borrow().x - self.x).abs() + (end.borrow().y - self.y).abs()
    }

    pub fn neighbors(&self) -> Vec<PointType> {
        let neighbors = vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ];
        neighbors
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<Point>>)
    {
        self.parent = Some(parent)
    }
}
