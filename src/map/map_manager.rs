use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use crate::astar::AStar;

use crate::map::{Map, Point, PointType};


/*lazy_static! {
    pub static ref MAP_MANAGER: MapManager = MapManager::new();
}
*/
pub struct MapManager {
    map_collections : Arc<RwLock<HashMap<i32, Arc<Mutex<dyn Map>>>>>,
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
    pub fn new() -> Self {
        MapManager{map_collections: Arc::new(RwLock::new(HashMap::new()))}
    }
    pub fn new_astar(&mut self) -> i32 {
        self.map_collections.clone().write().unwrap().insert(1, AStar::new());
        1
    }

    pub fn load(&self, map_id: i32, points: Vec<Vec<i32>>) {
        let binding = self.map_collections.clone();
        let binding2 = binding.read().unwrap();
        let res = binding2.get_key_value(&map_id);
        match res {
            None => { },
            Some(m) => m.1.lock().unwrap().load(points)
        }
    }
    pub fn find_path(&self, map_id: i32, start: &Point, end: &Point) -> Vec<PointType> {
        let binding = self.map_collections.clone();
        let binding = binding.read().unwrap();
        let res = binding.get_key_value(&map_id);
        match res {
            None => vec![],
            Some(m) => return m.1.lock().unwrap().find_path(start, end)
        }
    }
}
