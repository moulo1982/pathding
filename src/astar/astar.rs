use std::sync::{Arc, Mutex, RwLock};
use crate::map::{Map, OpenList, Point, PointType};

pub struct  AStar {
    pub points: Vec<Vec<i32>>,
}

impl Map for AStar {
    fn new() -> Arc<Mutex<dyn Map>> {
        Arc::new(Mutex::new(AStar { points: Vec::new() }))
    }
    fn load(&mut self, points: Vec<Vec<i32>>) {
        self.points = points;
    }
    fn find_path(&self, start: &Point, end: &Point) -> Vec<PointType> {

        let open_list = RwLock::new(OpenList::new());
        let close_list = RwLock::new(OpenList::new());

        open_list.try_write().unwrap().insert(&start, &end, start.clone());

        while open_list.try_read().unwrap().len() > 0 && !open_list.try_read().unwrap().contains_point(end){

            let min_f = open_list.try_write().unwrap().min_f().unwrap();

            let neighbors = min_f.try_read().unwrap().borrow().neighbors();
            for mut one in neighbors {

                if !self.in_map(&one) || open_list.try_read().unwrap().contains_point(&one) || open_list.try_read().unwrap().contains_point(&one) {
                    continue
                }

                one.set_parent(min_f.clone());
                open_list.try_write().unwrap().insert(&start, &end, one);//one直接移动到函数内，插入到列表中，后面不用了
            }

            let point: Point = min_f.try_write().unwrap().take();
            close_list.try_write().unwrap().insert(&start, &end, point);//min_f直接移动到函数内，插入到列表中，后面不用了
            break
        }

        let x = open_list.try_read().unwrap().to_array(); x
    }

    fn in_map(&self, point:&Point) -> bool {
        let borrow = point;//.borrow();
        if borrow.x < 0 || borrow.y < 0 {return false}
        if borrow.x > self.points.len() as i64 || borrow.x > self.points[0].len()  as i64 {return false}
        if self.points[borrow.x as usize][borrow.y as usize] == 1 {return false}
        true
    }
}

impl Drop for AStar {
    fn drop(&mut self) {
        self.points.clear()
    }
}