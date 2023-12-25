use game_pathfinding::{map, vec2d};
use game_pathfinding::map::MAP_MANAGER;

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

    let map = MAP_MANAGER.get().await;
    let mm = map.write().unwrap().new_astar();
    if let Err(err) = map.write().unwrap().load(mm, map_info) {
        println!("{}", err);
        return;
    }

    let threads: Vec<_> = (0..5)
        .map(|i| {
            let map = map.clone();
            tokio::spawn(async move {
                let result = map.read().unwrap().find_path(mm, &map::Point::new(1, 0), &map::Point::new(6, 7));
                println!("{}, {:?}", i, result);
            })
        })
        .collect();


    for thread in threads {
        thread.await.unwrap()
    }

}
