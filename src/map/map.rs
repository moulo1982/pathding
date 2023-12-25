use std::sync::{Arc, RwLock};
use crate::errors::my_errors::RetResult;
use crate::map::{Point, PointType};

pub trait Map: Send + Sync {
    fn new() -> Arc<RwLock<dyn Map>> where Self: Sized;
    fn load(&mut self, points: Vec<Vec<i32>>) -> RetResult<()>;

    fn find_path(&self, start: &Point, end: &Point) -> Vec<PointType>;

    fn in_map(&self, point:&Point) -> bool;
}