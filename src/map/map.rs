use crate::map::Point;

pub struct Map {
    pub points: Vec<Vec<i32>>,
}

impl Map {
    pub fn new() -> Self {
        Map { points: Vec::new() }
    }

    pub fn load(&mut self, points: Vec<Vec<i32>>) {
        self.points = points;
    }

    pub fn find(&self, _start: Point, _end: Point) -> Vec<Point> {
        Vec::new()
    }
}
