use std::collections::{BTreeMap};
use crate::map::{PointType};

#[derive(Clone)]
pub struct OpenList {
    points: BTreeMap<i64, PointType>,
}

impl OpenList {
    pub fn new() -> Self {
        OpenList{points: BTreeMap::new()}
    }

    pub fn insert_by_key(&mut self, key: i64, value: PointType) -> Option<PointType> {
        self.points.insert(key, value)
    }

    pub fn insert(&mut self, start: &PointType, end: &PointType, value: PointType) -> Option<PointType> {
        let f = value.borrow().f(start, end);
        self.insert_by_key(f, value)
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn min_f(&mut self) -> Option<PointType> {
        self.points.pop_first().map(|value| value.1 )
    }

    pub fn contains_point(&self, point: &PointType) -> bool{
        let result = self.points.iter().find_map(|v| {
            if *v.1.borrow() == *point.borrow() {
                Some(v)
            } else {
                None
            }
        });

        match result {
            None => false,
            Some(_) => true
        }
    }

    pub fn to_array(&self) -> Vec<PointType> {
        self.points.clone().into_values().collect()
    }
}

/*struct CloseList {

}*/