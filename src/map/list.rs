use crate::map::{PointType};

#[derive(Clone)]
pub struct OpenList {
    hash_points: Vec<PointType>,
}

impl OpenList {
    pub fn new() -> Self {
        OpenList{hash_points:Vec::new()}
    }

    pub fn insert(&mut self, value: PointType) {
        self.hash_points.push(value)
    }

    pub fn move_insert(&mut self, value: PointType) {
        self.hash_points.push(value)
    }

    pub fn remove(&mut self, value: &PointType) {
        for i in 0..self.hash_points.len() {
            if self.hash_points[i] == *value {
                self.hash_points.remove(i);
                break
            }
        }
    }

    pub fn len(&self) -> usize {
        self.hash_points.len()
    }

    pub fn min_f(&mut self) -> Option<PointType> {

        if self.hash_points.len() > 0 {
            self.hash_points.sort_by(|a, b| a.borrow().f().cmp(&b.borrow().f()) );
            let ret = self.hash_points[0].clone();
            self.hash_points.remove(0);
            return Some(ret);
        }

        None
    }

    pub fn contains_point(&self, point: &PointType) -> bool{
        for i in 0..self.hash_points.len() {
            if self.hash_points[i] == *point {
                return true;
            }
        }
        return false;
    }
}


pub struct CloseList {

}