use crate::awrs::resources::tile::Tile;

pub fn get_index_from_tiles(
    before_tile: Option<Tile>,
    tile: Tile,
    after_tile: Option<Tile>,
) -> usize {
    let before_dir = before_tile.map_or(Dir::None, |before| get_direction(before, tile));
    let after_dir = after_tile.map_or(Dir::None, |after| get_direction(tile, after));
    return get_index_from_directions((before_dir, after_dir));
}

fn get_direction(tile_a: Tile, tile_b: Tile) -> Dir {
    if tile_b.x < tile_a.x {
        return Dir::Left;
    }
    if tile_b.x > tile_a.x {
        return Dir::Right;
    }
    if tile_b.y < tile_a.y {
        return Dir::Down;
    }
    if tile_b.y > tile_a.y {
        return Dir::Up;
    }
    panic!(
        "Tried to create arrow path between tiles which were not next to each other: {:?} and {:?}",
        tile_a, tile_b
    );
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn get_index_from_directions((from, to): (Dir, Dir)) -> usize {
    match (from, to) {
        (Dir::Down, Dir::Down) => 11,
        (Dir::Down, Dir::Right) => 15,
        (Dir::Down, Dir::Left) => 14,
        (Dir::Down, Dir::None) => 17,
        // -
        (Dir::Up, Dir::Right) => 7,
        (Dir::Up, Dir::Up) => 12,
        (Dir::Up, Dir::None) => 6,
        (Dir::Up, Dir::Left) => 10,
        // -
        (Dir::Left, Dir::Up) => 13,
        (Dir::Left, Dir::Down) => 9,
        (Dir::Left, Dir::Left) => 22,
        (Dir::Left, Dir::None) => 21,
        // -
        (Dir::Right, Dir::Right) => 1,
        (Dir::Right, Dir::None) => 2,
        (Dir::Right, Dir::Down) => 8,
        (Dir::Right, Dir::Up) => 16,
        // -
        (Dir::None, Dir::Right) => 0,
        (Dir::None, Dir::Down) => 5,
        (Dir::None, Dir::Up) => 18,
        (Dir::None, Dir::Left) => 23,
        // -
        (Dir::Right, Dir::Left)
        | (Dir::Left, Dir::Right)
        | (Dir::Up, Dir::Down)
        | (Dir::Down, Dir::Up)
        | (Dir::None, Dir::None) => 24,
    }
}
