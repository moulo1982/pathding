use std::cell::RefCell;
use std::rc::Rc;

pub type PointType = Rc<RefCell<Point>>;

#[derive(Debug, Clone, Default)]
pub struct Point {
    pub x: i64,
    pub y: i64,

    pub g: i64,
    pub h: i64,

    pub parent: Option<PointType>
}

impl PartialEq for Point {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Eq for Point {
}

impl Point {

    #[inline]
    pub fn new(x: i64, y:i64, g:i64, h:i64) -> PointType {
        Rc::new(RefCell::new(Point{x, y, g, h, parent:None}))
    }

    #[inline]
    pub fn f(&self) -> i64 {
        self.g + self.h
    }

    #[inline]
    pub fn neighbors(&self, start: &PointType, end: &PointType) -> Vec<PointType> {
        let sb = start.borrow();
        let eb = end.borrow();
        vec![
            Point::new(self.x - 1, self.y,
                       Point::dis(self.x - 1, self.y, sb.x, sb.y),
                       Point::dis(self.x - 1, self.y, eb.x, eb.y),
            ),
            Point::new(self.x - 1, self.y - 1,
                       Point::dis(self.x - 1, self.y - 1, sb.x, sb.y),
                       Point::dis(self.x - 1, self.y - 1, eb.x, eb.y),
            ),
            Point::new(self.x, self.y - 1,
                       Point::dis(self.x, self.y - 1, sb.x, sb.y),
                       Point::dis(self.x, self.y - 1, eb.x, eb.y),
            ),
            Point::new(self.x + 1, self.y - 1,
                       Point::dis(self.x + 1, self.y - 1, sb.x, sb.y),
                       Point::dis(self.x + 1, self.y - 1, eb.x, eb.y),
            ),
            Point::new(self.x + 1, self.y,
                       Point::dis(self.x + 1, self.y, sb.x, sb.y),
                       Point::dis(self.x + 1, self.y, eb.x, eb.y),
            ),
            Point::new(self.x + 1, self.y + 1,
                       Point::dis(self.x - 1, self.y + 1, sb.x, sb.y),
                       Point::dis(self.x - 1, self.y + 1, eb.x, eb.y),
            ),
            Point::new(self.x, self.y + 1,
                       Point::dis(self.x, self.y + 1, sb.x, sb.y),
                       Point::dis(self.x, self.y + 1, eb.x, eb.y),
            ),
            Point::new(self.x - 1, self.y + 1,
                       Point::dis(self.x - 1, self.y + 1, sb.x, sb.y),
                       Point::dis(self.x - 1, self.y + 1, eb.x, eb.y),
            ),
        ]
    }

    #[inline]
    pub fn dis(x: i64, y: i64, x1: i64, y1: i64) -> i64 {
        (x1 - x).abs() + (y - y1).abs()
    }

    #[inline]
    pub fn distance(&self, other: &PointType) -> i64 {
        let b = other.borrow();
        Point::dis(self.x, self.y, b.x, b.y)
    }

    #[inline]
    pub fn set_parent(&mut self, parent: PointType) {
        self.parent = Some(parent)
    }

    #[inline]
    pub fn set_g(&mut self, g: i64) {
        self.g = g;
    }
}