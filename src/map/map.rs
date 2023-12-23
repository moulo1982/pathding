use std::sync::{Arc, Mutex};
use crate::map::{Point, PointType};

pub trait Map: Send {
    fn new() -> Arc<Mutex<dyn Map>> where Self: Sized;
    fn load(&mut self, points: Vec<Vec<i32>>);

    fn find_path(&self, start: &Point, end: &Point) -> Vec<PointType>;

    fn in_map(&self, point:&Point) -> bool;
}