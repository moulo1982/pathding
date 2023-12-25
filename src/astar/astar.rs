use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use crate::errors::my_errors::RetResult;
use crate::map::{Map, OpenList, Point, PointType};

pub struct  AStar {
    pub map: Arc<RwLock<Vec<Vec<i32>>>>,
}

impl Map for AStar {
    fn new() -> Arc<RwLock<dyn Map>> {
        Arc::new(RwLock::new(AStar { map: Arc::new(RwLock::new(vec![])) }))
    }
    fn load(&mut self, points: Vec<Vec<i32>>) -> RetResult<()> {
        self.map = Arc::new(RwLock::new(points));
        Ok(())
    }
    fn find_path(&self, start: &Point, end: &Point) -> Vec<PointType> {

        let open_list = RefCell::new(OpenList::new());
        let close_list = RefCell::new(OpenList::new());

        open_list.borrow_mut().insert(&start, &end, start.clone());

        while open_list.borrow().len() > 0 && !open_list.borrow().contains_point(end){

            let min_f = open_list.borrow_mut().min_f();
            if min_f.is_none() {
                break
            }

            let min_f = min_f.unwrap();

            let neighbors = min_f.borrow().neighbors();
            for mut one in neighbors {

                if !self.in_map(&one) || open_list.borrow().contains_point(&one) || open_list.borrow().contains_point(&one) {
                    continue
                }

                one.set_parent(min_f.clone());
                open_list.borrow_mut().insert(&start, &end, one);//one直接移动到函数内，插入到列表中，后面不用了
            }

            close_list.borrow_mut().insert(&start, &end, min_f.take());//min_f直接移动到函数内，插入到列表中，后面不用了

            //没写完，直接跳出，避免死循环
            break
        }

        let x = open_list.borrow().to_array(); x
    }

    fn in_map(&self, point:&Point) -> bool {
        let borrow = point;//.borrow();
        if borrow.x < 0 || borrow.y < 0 {return false}
        if borrow.x > self.map.read().unwrap().len() as i64 || borrow.x > self.map.read().unwrap()[0].len()  as i64 {return false}
        if self.map.read().unwrap()[borrow.x as usize][borrow.y as usize] == 1 {return false}
        true
    }
}

impl Drop for AStar {
    fn drop(&mut self) {
        self.map.write().unwrap().clear()
    }
}