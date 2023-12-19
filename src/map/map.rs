use std::cell::RefCell;
use crate::map::{OpenList, PointType};

pub struct Map {
    pub points: Vec<Vec<i32>>,
}

impl Drop for Map {
    fn drop(&mut self) {
        self.points.clear()
    }
}

impl Map {
    pub fn new() -> Box<Self> {
        Box::new(Map { points: Vec::new() })
    }

    pub fn load(&mut self, points: Vec<Vec<i32>>) {
        self.points = points;
    }

    pub fn in_map(&self, point:PointType) -> bool {
        let borrow = point.borrow();
        if borrow.x < 0 || borrow.y < 0 {return false}
        if borrow.x > self.points.len() as i64 || borrow.x > self.points[0].len()  as i64 {return false}
        if self.points[borrow.x as usize][borrow.y as usize] == 1 {return false}
        true
    }


    pub fn find(&self, start: PointType, end: PointType) -> Vec<PointType> {

        let open_list = RefCell::new(OpenList::new());
        let close_list = RefCell::new(OpenList::new());

        open_list.borrow_mut().insert(start.borrow().f(start.clone(), end.clone()), start.clone());

        while open_list.borrow().len() > 0 && !open_list.borrow().contains_point(end.clone()){

            let min_f = open_list.borrow_mut().min_f().unwrap();

            let neighbors = min_f.borrow().neighbors();
            for one in neighbors {
                //1：是个合法的点，2：openList中不存在，3：closeList不存在
                {
                    let borrow = open_list.borrow();
                    if !self.in_map(one.clone()) || borrow.contains_point(one.clone()) || borrow.contains_point(one.clone()) {
                        continue
                    }
                }

                one.borrow_mut().set_parent(min_f.clone());
                open_list.borrow_mut().insert(one.borrow().f(start.clone(), end.clone()), one.clone());
            }

            close_list.borrow_mut().insert(min_f.borrow().f(start.clone(), end.clone()), min_f.clone());
        }

        let x = open_list.borrow().to_array(); x
    }
}
