
#[derive(Debug, Hash, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
#[allow(unused_lifetimes)]
pub struct Point<'b, 'a:'b> {
    pub x: i64,
    pub y: i64,

    pub parent: Option<&'b Point<'a, 'b>>,
}

impl<'a, 'b> Point<'a, 'b> {

    pub fn new(x: i64, y:i64) -> Self {
        Point{x, y, parent:None}
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
        let neighbors = vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ];
        neighbors
    }

    pub fn parent(&mut self, parent: &'a Point) where 'a : 'b{
        self.parent = Some(parent)
    }
}
