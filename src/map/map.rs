use std::cell::RefCell;
use std::ops::DerefMut;
use crate::map::{OpenList, Point};

pub struct Map {
    pub points: Vec<Vec<i32>>,
}

impl Drop for Map {
    fn drop(&mut self) {
        self.points.clear()
    }
}

impl Map {
    pub fn new() -> Self {
        Map { points: Vec::new() }
    }

    pub fn load(&mut self, points: Vec<Vec<i32>>) {
        self.points = points;
    }

    pub fn in_map(&self, point:&Point) -> bool {
        if point.x < 0 || point.y < 0 {return false}
        if point.x > self.points.len() as i64 || point.x > self.points[0].len()  as i64 {return false}
        if self.points[point.x as usize][point.y as usize] == 1 {return false}
        true
    }

    pub fn find(&self, start: Point, end: Point) -> Box<Vec<Point>> {
        let open_list = RefCell::new(OpenList::new());
        let mut close_list = OpenList::new();

        open_list.borrow_mut().insert(start.f(&start, &end), start.clone());


        //开放列表为空，并且重点不在开放列表中
        while open_list.borrow().len() > 0 && !open_list.borrow().contains_point(&end){


            let mut borrow_mut = open_list.borrow_mut();
            let min_f = borrow_mut.min_f().take();

            match min_f {
                None => {}
                Some(v) => {
                    //计算所有邻居的f
                    let neighbors = v.neighbors();
                    for mut one in neighbors {
                        //1：是个合法的点，2：openList中不存在，3：closeList不存在
                        if !self.in_map(&one) || open_list.borrow().contains_point(&one) || open_list.borrow().contains_point(&one) {
                            continue
                        }

                        one.parent(&v);
                        open_list.borrow_mut().insert(one.f(&start, &end), one);
                    }


                    close_list.insert(min_f.unwrap().f(&start, &end), min_f.unwrap());
                }
            }

        }

        Box::new(vec![])
        //Box::new(open_list.to_array())
    }
}
