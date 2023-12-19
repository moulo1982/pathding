use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
}

fn random_empty_tile(arr: &mut [Tile]) -> &mut Tile {
    loop {
        let i = thread_rng().gen_range(0..arr.len());
        let tile = &mut arr[i];
        if Tile::Empty == *tile{
            return tile;
        }
    }
}