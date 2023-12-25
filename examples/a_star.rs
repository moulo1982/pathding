use std::sync::Arc;
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

    let map = Arc::clone(&MAP_MANAGER);
    let mm = map.write().await.new_astar().await;
    if let Err(err) = map.write().await.load(mm, map_info) {
        println!("{}", err);
        return;
    }

    let threads: Vec<_> = (0..1_000)
        .map(|_i| {
            let map = map.clone();
            tokio::spawn(async move {
                let _result = map.read().await.find_path(mm, &map::Point::new(1, 0), &map::Point::new(6, 7));
                //println!("{}, {:?}", i, result);
            })
        })
        .collect();

    println!("Total: {}", threads.len());
    for thread in threads {
        thread.await.unwrap()
    }

}
