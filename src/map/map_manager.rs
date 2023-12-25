use std::collections::HashMap;
use std::sync::Arc;
//use async_once::AsyncOnce;
use tokio::sync::RwLock;
use lazy_static::lazy_static;
use crate::astar::AStar;
use crate::errors::my_errors::{MyError, RetResult};
use crate::id::id_generator::ID_GENERATOR;
use crate::id::instance_id::InstanceIdType;

use crate::map::{Map, Point, PointType};
use crate::map::map::MapType;


lazy_static! {
    pub static ref MAP_MANAGER: Arc<RwLock<MapManager>> = MapManager::new();

    //异步方式的生成单例，因为有些生成单例的代码，是await的，所以整体就需要await。 比如mongodb的client
    /*
    pub static ref MONGODB_CLIENT: AsyncOnce<mongodb::Database> = AsyncOnce::new( async {
        let config = &GLOBAL_CONFIG;
        let client_options = ClientOptions::parse(&config.mongodb.url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client.database(&config.mongodb.db)
    });
    */
}

pub struct MapManager {
    map_collections : HashMap<u128, MapType>,
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
    pub async fn new_astar(&mut self) -> InstanceIdType {
        let map_id = Arc::clone(&ID_GENERATOR).write().await.generate_instance_id();
        self.map_collections.insert(map_id, AStar::new());
        map_id
    }

    pub fn load(&self, map_id: InstanceIdType, points: Vec<Vec<i32>>) -> RetResult<()> {
        let res = self.map_collections.get(&map_id);
        match res {
            None => Err(MyError::MapNotExist(map_id).into()),
            Some(m) => m.clone().write().map_or_else(
                |e| Err(MyError::UnknownErr(e.to_string()).into()),
                |mut v| v.load(points))
        }
    }
    pub fn find_path(&self, map_id: InstanceIdType, start: &Point, end: &Point) -> RetResult<Vec<PointType>> {
        let res = self.map_collections.get(&map_id);
        match res {
            None => Err(MyError::MapNotExist(map_id).into()),
            Some(m) => m.clone().write().map_or_else(
                |e| Err(MyError::UnknownErr(e.to_string()).into()),
                |v| Ok(v.find_path(start, end)))
        }
    }
}
