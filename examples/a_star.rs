use game_pathfinding::{map, vec2d};
use game_pathfinding::map::MapManager;
use game_pathfinding::id::instance_id::InstanceIdType;

#[tokio::main]
async fn main() {
    let map_info = vec2d![
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
    ];

    let map = MapManager::get_instance();
    let mm = map.write().await.new_astar();
    let mut _map_id: InstanceIdType = 0;
    match mm {
        Ok(v) => _map_id = v,
        Err(err) => {println!("{}", err);return;}
    }

    if let Err(err) = map.write().await.load(_map_id, map_info) {
        println!("{}", err);
        return;
    }

    tokio::spawn(async move {
        let result = map.read().await.find_path(_map_id, &map::Point::new(1, 0), &map::Point::new(6, 7));
        match result {
            Ok(ref v) => println!("寻路结果, {:?}", v),
            Err(e) => println!("{}", e)
        }
    }).await.unwrap();

}
