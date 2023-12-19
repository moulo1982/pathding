use std::cell::RefCell;
use crate::map::{OpenList, Point, PointType};

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

    pub fn in_map(&self, point:&Point) -> bool {
        let borrow = point;//.borrow();
        if borrow.x < 0 || borrow.y < 0 {return false}
        if borrow.x > self.points.len() as i64 || borrow.x > self.points[0].len()  as i64 {return false}
        if self.points[borrow.x as usize][borrow.y as usize] == 1 {return false}
        true
    }


    pub fn find(&self, start: &Point, end: &Point) -> Vec<PointType> {

        let open_list = RefCell::new(OpenList::new());
        let close_list = RefCell::new(OpenList::new());

        open_list.borrow_mut().insert(&start, &end, start.clone());

        while open_list.borrow().len() > 0 && !open_list.borrow().contains_point(end){

            let min_f = open_list.borrow_mut().min_f().unwrap();

            let neighbors = min_f.borrow().neighbors();
            for mut one in neighbors {
                //1：是个合法的点，2：openList中不存在，3：closeList不存在
                {
                    let borrow = open_list.borrow();
                    if !self.in_map(&one) || borrow.contains_point(&one) || borrow.contains_point(&one) {
                        continue
                    }
                }

                one.set_parent(min_f.clone());
                open_list.borrow_mut().insert(&start, &end, one);//one直接移动到函数内，插入到列表中，后面不用了
            }

            close_list.borrow_mut().insert(&start, &end, min_f.take());//min_f直接移动到函数内，插入到列表中，后面不用了
        }

        let x = open_list.borrow().to_array(); x
    }
}
