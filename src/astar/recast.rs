use crate::map::{Map, Point, PointType};

pub trait Recast {
    fn new_recast() -> Box<dyn Recast> where Self: Sized;
    fn load(&mut self, points: Vec<Vec<i32>>);
    fn find_path(&self, start: &Point, end: &Point) -> Vec<PointType>;
}

impl Recast for Map {
    fn new_recast() -> Box<dyn Recast> {
        Box::new(Map { points: Vec::new() })
    }

    fn load(&mut self, points: Vec<Vec<i32>>) {
        self.points = points;
    }
    fn find_path(&self, _start: &Point, _end: &Point) -> Vec<PointType> {
        Vec::default()
        /*let open_list = RefCell::new(OpenList::new());
        let close_list = RefCell::new(OpenList::new());

        open_list.borrow_mut().insert(&start, &end, start.clone());

        while open_list.borrow().len() > 0 && !open_list.borrow().contains_point(end){

            let min_f = open_list.borrow_mut().min_f().unwrap();

            let neighbors = min_f.borrow().neighbors();
            for mut one in neighbors {

                if !self.in_map(&one) || open_list.borrow().contains_point(&one) || open_list.borrow().contains_point(&one) {
                    continue
                }

                one.set_parent(min_f.clone());
                open_list.borrow_mut().insert(&start, &end, one);//one直接移动到函数内，插入到列表中，后面不用了
            }

            close_list.borrow_mut().insert(&start, &end, min_f.take());//min_f直接移动到函数内，插入到列表中，后面不用了
        }

        let x = open_list.borrow().to_array(); x*/
    }
}