use std::collections::{BTreeMap, BTreeSet};
use crate::map::Point;

#[derive(Clone)]
pub struct OpenList<'a, 'b> {
    points: BTreeMap<i64, Point<'a, 'b>>,
}

impl<'a, 'b> OpenList<'a, 'b> {
    pub fn new() -> Self {
        OpenList{points: BTreeMap::new()}
    }

    pub fn insert(&mut self, key: i64, value: Point<'a, 'b>) -> Option<Point> {
        self.points.insert(key, value)
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn min_f(&mut self) -> Option<Point> {
        self.points.pop_first().map(|value| value.1 )
    }

    pub fn contains_point(&self, point: &Point) -> bool{
        let result = self.points.iter().find_map(|v| {
            if v.1 == point {
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

    pub fn to_array(&self) -> Vec<Point> {
        self.points.clone().into_values().collect()
    }
}

struct CloseList {

}