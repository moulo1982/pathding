use std::collections::BTreeMap;

use crate::map::PointType;

#[derive(Clone)]
pub struct OpenList {
    hash_points: BTreeMap<i64, Vec<PointType>>,
}

impl OpenList {
    pub fn new() -> Self {
        OpenList{hash_points: BTreeMap::new()}
    }

    pub fn insert(&mut self, value: PointType) {
        let f = value.borrow().f();
        if let Some(vec) = self.hash_points.get_mut(&f) {
            vec.push(value);
        } else {
            self.hash_points.insert(f, vec![value]);
        }
    }

    pub fn remove(&mut self, value: &PointType) {

        let f = value.borrow().f();
        if let Some(vec) = self.hash_points.get_mut(&f) {
            for i in 0..vec.len() {
                if vec[i] == *value {
                    vec.remove(i);
                    break
                }
            }

            if vec.len() == 0 {
                self.hash_points.remove(&f);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.hash_points.len()
    }

    pub fn min_f(&mut self) -> Option<PointType> {
        if let Some(mut v) = self.hash_points.first_entry() {
            return v.get_mut().pop();
        }
        None
    }

    pub fn contains_point(&mut self, point: &PointType) -> bool{
        let f = point.borrow().f();
        if let Some(vec) = self.hash_points.get(&f) {
            for i in 0..vec.len() {
                if vec[i] == *point {
                    return true;
                }
            }
        }
        return false;
    }
}


pub struct CloseList {

}