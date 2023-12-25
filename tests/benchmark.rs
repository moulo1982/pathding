use std::cell::RefCell;
use chrono::Utc;
use game_pathfinding::map::MapManager;
use game_pathfinding::{map, vec2d};
use game_pathfinding::id::instance_id::InstanceIdType;

#[tokio::test]
async fn many_test() {
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
        Err(err) => {panic!("{}", err);}
    }

    if let Err(err) = map.write().await.load(_map_id, map_info) {
        println!("{}", err);
        return;
    }

    let begin = Utc::now().timestamp_micros();

    let threads: Vec<_> = (0..1_000)
        .map(|_| {
            let map = map.clone();
            tokio::spawn(async move {
                map.read().await.find_path(_map_id, &map::Point::new(1, 0), &map::Point::new(6, 7)).expect("TODO: panic message");
            })
        })
        .collect();

    let len = threads.len();
    for thread in threads {
        thread.await.unwrap()
    }

    let end = (Utc::now().timestamp_micros() - begin) as f64 / 1000.0f64;
    println!("Total: {} times, Use: {}ms", len, end);
}

#[tokio::test]
async fn other_test() {
    let data = RefCell::new(0);

    let begin = Utc::now().timestamp_micros();

    let times = 1_000_000;
    for _ in 0..times {
        let mut borrow = data.borrow_mut();
        *borrow += 1;
    }

    println!("Final value: {}", *data.borrow());
    let end = (Utc::now().timestamp_micros() - begin) as f64 / 1000.0f64;
    println!("Total: {} times, Use: {}ms", times, end);
}

#[tokio::test]
async fn o_test() {
    let mut _i = 0;

    let begin = Utc::now().timestamp_micros();

    let times = 1_000_000;
    for _ in 0..times {
        _i += 1;
    }

    let end = (Utc::now().timestamp_micros() - begin) as f64 / 1000.0f64;
    println!("Total: {} times, Use: {}ms", times, end);
}