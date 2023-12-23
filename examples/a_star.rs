use std::thread;
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

    let mut map = MapManager::new();
    let mm = map.new_astar();
    map.load(mm, map_info);

    let threads: Vec<_> = (0..5)
        .map(|i| {
            let map = map.clone();
            thread::spawn(move || {
                let result = map.find_path(mm, &map::Point::new(1, 0), &map::Point::new(6, 7));
                println!("{}, {:?}", i, result);
            })
        })
        .collect();


    for thread in threads {
        thread.join().unwrap();
    }

}
