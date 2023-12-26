use game_pathfinding::{map, vec2d};
use game_pathfinding::map::MapManager;

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
    let map_id = map.write().await.new_astar().await;

    if let Err(err) = map.write().await.load(map_id, map_info) {
        println!("{}", err);
        return;
    }

    tokio::spawn(async move {
        let result = map.read().await.find_path(map_id, &map::Point::new(1, 0), &map::Point::new(6, 7));
        match result {
            Ok(ref v) => println!("寻路结果, {:?}", v),
            Err(e) => println!("{}", e)
        }
    }).await.unwrap();

}
