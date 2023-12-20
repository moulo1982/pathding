use crate::map::Point;

pub struct Map {
    pub points: Vec<Vec<i32>>,
}

impl Drop for Map {
    fn drop(&mut self) {
        self.points.clear()
    }
}

impl Map {

    pub fn in_map(&self, point:&Point) -> bool {
        let borrow = point;//.borrow();
        if borrow.x < 0 || borrow.y < 0 {return false}
        if borrow.x > self.points.len() as i64 || borrow.x > self.points[0].len()  as i64 {return false}
        if self.points[borrow.x as usize][borrow.y as usize] == 1 {return false}
        true
    }



}
