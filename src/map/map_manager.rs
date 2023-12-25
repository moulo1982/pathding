use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use crate::astar::AStar;
use crate::errors::my_errors::{MyError, RetResult};

use crate::map::{Map, Point, PointType};


lazy_static! {
    pub static ref MAP_MANAGER: AsyncOnce<Arc<RwLock<MapManager>>> = AsyncOnce::new( async {
        MapManager::new()
    });
}

pub struct MapManager {
    map_collections : HashMap<i32, Arc<RwLock<dyn Map>>>,
}

impl Clone for MapManager {
    fn clone(&self) -> Self {
        MapManager{map_collections: self.map_collections.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.map_collections = source.map_collections.clone()
    }
}

impl MapManager {
    pub fn new() -> Arc<RwLock<MapManager>> {
        Arc::new(RwLock::new(MapManager{map_collections: HashMap::new()}))
    }
    pub fn new_astar(&mut self) -> i32 {
        self.map_collections.insert(1, AStar::new());
        1
    }

    pub fn load(&self, map_id: i32, points: Vec<Vec<i32>>) -> RetResult<()> {
        let res = self.map_collections.get(&map_id);
        match res {
            None => Err(MyError::MapNotExist(map_id).into()),
            Some(m) => m.clone().write().unwrap().load(points)
        }
    }
    pub fn find_path(&self, map_id: i32, start: &Point, end: &Point) -> RetResult<Vec<PointType>> {
        let res = self.map_collections.get(&map_id);
        match res {
            None => Err(MyError::MapNotExist(map_id).into()),
            Some(m) => return Ok(m.clone().read().unwrap().find_path(start, end))
        }
    }
}
