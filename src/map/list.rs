use std::collections::BTreeMap;

use crate::map::PointType;

pub struct List {
    hash_points: BTreeMap<i64, Vec<PointType>>,
}

impl List {

    #[inline]
    pub fn new() -> Self {
        List {hash_points: BTreeMap::new()}
    }

    pub fn insert(&mut self, value: PointType) {
        let f = value.borrow().f();
        if let Some(vec) = self.hash_points.get_mut(&f) {
            vec.push(value);
        } else {
            let mut my_vec: Vec<PointType> = Vec::with_capacity(128);
            my_vec.push(value);
            self.hash_points.insert(f, my_vec);
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

    #[inline]
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