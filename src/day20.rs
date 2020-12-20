use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Debug, Clone, Eq, PartialEq)]
struct Tile {
    id: usize,
    left: String,
    right: String,
    top: String,
    bottom: String,
    interior: Vec<Vec<bool>>,
}
#[derive(Debug, Clone, Copy)]
enum Dir {
    Top,
    Left,
    Right,
    Bottom,
}
const DIRECTIONS: [Dir; 4] = [Dir::Top, Dir::Left, Dir::Right, Dir::Bottom];

impl Tile {
    fn get(&self, d: Dir) -> &str {
        match d {
            Dir::Top => &self.top,
            Dir::Left => &self.left,
            Dir::Right => &self.right,
            Dir::Bottom => &self.bottom,
        }
    }
}

fn normalize(s: &str) -> String {
    let rev = s.chars().rev().collect::<String>();
    if rev.as_str() < s {
        rev
    } else {
        s.to_owned()
    }
}

const SEA_MONSTER: &str = r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "#;

pub fn solve1(input: &[String]) {
    let tiles: Vec<Tile> = input
        .split(|line| line == "")
        .filter(|tile| !tile.is_empty())
        .map(|tile| {
            let id: usize = tile[0][..tile[0].len() - 1]
                .split(" ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let top = tile[1].chars().rev().collect();
            let bottom = tile[tile.len() - 1].clone();
            let left: String = tile[1..]
                .iter()
                .map(|line| line.chars().nth(0).unwrap())
                .collect();
            let right: String = tile[1..]
                .iter()
                .rev()
                .map(|line| line.chars().last().unwrap())
                .collect();
            let interior = tile[2..tile.len() - 1]
                .iter()
                .map(|line| {
                    let mut line: Vec<bool> = line.chars().skip(1).map(|c| c == '#').collect();
                    line.pop();
                    line
                })
                .collect();
            Tile {
                id,
                left,
                right,
                top,
                bottom,
                interior,
            }
        })
        .collect();
    let mut edge_counts: HashMap<String, Vec<&Tile>> = HashMap::new();
    for tile in tiles.iter() {
        for edge in &[&tile.left, &tile.right, &tile.top, &tile.bottom] {
            edge_counts
                .entry(normalize(edge))
                .or_insert(Vec::new())
                .push(&tile);
        }
    }
    let mut corners: Vec<Tile> = tiles
        .iter()
        .filter(|tile| {
            (&[&tile.left, &tile.right, &tile.top, &tile.bottom])
                .iter()
                .filter(|edge| edge_counts[&normalize(edge)].len() == 1)
                .count()
                == 2
        })
        .cloned()
        .collect();
    let id_prod: usize = corners.iter().map(|tile| tile.id).product();
    dbg!(id_prod);
    let mut edges: Vec<Tile> = tiles
        .iter()
        .filter(|tile| {
            (&[&tile.left, &tile.right, &tile.top, &tile.bottom])
                .iter()
                .filter(|edge| edge_counts[&normalize(edge)].len() == 1)
                .count()
                == 1
        })
        .cloned()
        .collect();
    let mut middle: Vec<Tile> = tiles
        .iter()
        .filter(|tile| {
            (&[&tile.left, &tile.right, &tile.top, &tile.bottom])
                .iter()
                .filter(|edge| edge_counts[&normalize(edge)].len() == 1)
                .count()
                == 0
        })
        .cloned()
        .collect();
    let top_left = corners.pop().unwrap();
    let top_left_dirs: Vec<Dir> = DIRECTIONS
        .iter()
        .copied()
        .filter(|dir| edge_is_unique(top_left.get(*dir), &edge_counts))
        .collect();
    let top_left = rotate_to_match(top_left_dirs[0], top_left_dirs[1], top_left);
    let mut top_edge = vec![top_left];
    while let Some((tile, tile_left_dir)) =
        get_matching_tile(top_edge.last().unwrap(), Dir::Right, &edge_counts)
    {
        let tile_top_dir = DIRECTIONS
            .iter()
            .copied()
            .filter(|d| edge_is_unique(tile.get(*d), &edge_counts))
            .filter(|d| !is_opposite(*d, tile_left_dir))
            .nth(0)
            .unwrap();
        top_edge.push(rotate_to_match(tile_left_dir, tile_top_dir, tile));
    }
    let mut grid = vec![top_edge];
    loop {
        let prior_row = grid.last().unwrap();
        let (first, first_top_dir) =
            match get_matching_tile(&prior_row[0], Dir::Bottom, &edge_counts) {
                Some(pair) => pair,
                None => break,
            };
        let first_left_dir = DIRECTIONS
            .iter()
            .copied()
            .filter(|d| edge_is_unique(first.get(*d), &edge_counts))
            .filter(|d| !is_opposite(*d, first_top_dir))
            .nth(0)
            .unwrap();
        let first = rotate_to_match(first_left_dir, first_top_dir, first);
        let mut row = vec![first];
        for top_tile in prior_row[1..].iter() {
            let mut current_grid = grid.clone();
            current_grid.push(row.clone());
            for row in current_grid {
                let ids: Vec<usize> = row.into_iter().map(|t| t.id).collect();
                //dbg!(ids);
            }

            let left_tile = row.last().unwrap();
            let (tile, left_dir) = get_matching_tile(left_tile, Dir::Right, &edge_counts).unwrap();
            let (tile_dup, top_dir) =
                get_matching_tile(top_tile, Dir::Bottom, &edge_counts).unwrap();
            assert_eq!(tile, tile_dup);
            let tile = rotate_to_match(left_dir, top_dir, tile);
            row.push(tile);
        }
        grid.push(row);
    }
    let tile_edge_len = grid[0][0].interior.len();
    let mut big_grid = vec![
        vec![false; grid[0].len() * grid[0][0].interior[0].len()];
        grid.len() * grid[0][0].interior.len()
    ];
    for (big_y, big_row) in grid.into_iter().enumerate() {
        for (big_x, tile) in big_row.into_iter().enumerate() {
            for (y, row) in tile.interior.into_iter().enumerate() {
                for (x, v) in row.into_iter().enumerate() {
                    big_grid[big_y * tile_edge_len + y][big_x * tile_edge_len + x] = v;
                }
            }
        }
    }
    let sea_monster_bmp: Vec<Vec<bool>> = SEA_MONSTER
        .split("\n")
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let sea_monster_pixel_count = sea_monster_bmp
        .iter()
        .flat_map(|row| row.iter())
        .filter(|p| **p)
        .count();
    let mut not_in_sea_monster = big_grid.clone();
    let sea_monsters: Vec<_> = (0..4)
        .map(|r| {
            let mut m = sea_monster_bmp.clone();
            for _ in (0..r) {
                m = rotate_bitmap_ccw(m);
            }
            m
        })
        .flat_map(|m| vec![m.clone(), m.into_iter().rev().collect()].into_iter())
        .collect();
    let grid_width = big_grid[0].len();
    for sea_monster in sea_monsters {
        for (start_y, rows) in big_grid.windows(sea_monster.len()).enumerate() {
            let monster_width = sea_monster[0].len();
            for start_x in 0..grid_width - monster_width + 1 {
                let overlap_pixels: usize = rows
                    .iter()
                    .zip(&sea_monster)
                    .map(|(row, sea_monster_row)| {
                        row[start_x..start_x + monster_width]
                            .iter()
                            .zip(sea_monster_row)
                            .filter(|(px, sea_monster_px)| **sea_monster_px && **px)
                            .count()
                    })
                    .sum();
                if overlap_pixels == sea_monster_pixel_count {
                    for (dy, row) in sea_monster.iter().enumerate() {
                        for (dx, px) in row.iter().enumerate() {
                            if *px {
                                not_in_sea_monster[start_y + dy][start_x + dx] = false;
                            }
                        }
                    }
                }
            }
        }
    }
    dbg!(&not_in_sea_monster);
    let ans = not_in_sea_monster
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|px| *px)
        .count();
    dbg!(ans);
}

fn get_matching_tile(t: &Tile, d: Dir, index: &HashMap<String, Vec<&Tile>>) -> Option<(Tile, Dir)> {
    //dbg!(t, d);
    let e = normalize(t.get(d));
    let matching = dbg!(&index[&e]);
    let matching_tile = matching
        .iter()
        .copied()
        .filter(|t2| t2.id != t.id)
        .nth(0)?
        .clone();
    let dir = DIRECTIONS
        .iter()
        .copied()
        .filter(|d| e == normalize(matching_tile.get(*d)))
        .nth(0)
        .unwrap();
    Some((matching_tile, dir))
}

fn edge_is_unique(edge: &str, edge_counts: &HashMap<String, Vec<&Tile>>) -> bool {
    edge_counts[normalize(edge).as_str()].len() == 1
}

fn rotate_bitmap_ccw(bmp: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut interior = vec![vec![false; bmp.len()]; bmp[0].len()];
    let l = interior.len();
    for (y, row) in bmp.into_iter().enumerate() {
        for (x, v) in row.into_iter().enumerate() {
            interior[l - 1 - x][y] = v;
        }
    }
    interior
}

fn rotate_ccw(mut t: Tile, count: usize) -> Tile {
    for _ in 0..count {
        t = Tile {
            id: t.id,
            top: t.right,
            left: t.top,
            bottom: t.left,
            right: t.bottom,
            interior: rotate_bitmap_ccw(t.interior),
        }
    }
    t
}
fn v_flip(t: Tile) -> Tile {
    Tile {
        id: t.id,
        top: t.bottom,
        left: t.left,
        bottom: t.top,
        right: t.right,
        interior: t.interior.into_iter().rev().collect(),
    }
}

fn rotate_to_match(left: Dir, top: Dir, t: Tile) -> Tile {
    let t = match left {
        Dir::Left => t,
        Dir::Top => rotate_ccw(t, 1),
        Dir::Right => rotate_ccw(t, 2),
        Dir::Bottom => rotate_ccw(t, 3),
    };
    let out = match (left, top) {
        (Dir::Left, Dir::Top)
        | (Dir::Top, Dir::Right)
        | (Dir::Right, Dir::Bottom)
        | (Dir::Bottom, Dir::Left) => t,
        _ => v_flip(t),
    };
    out
}

fn is_opposite(d1: Dir, d2: Dir) -> bool {
    match (d1, d2) {
        (Dir::Top, Dir::Bottom) => true,
        (Dir::Bottom, Dir::Top) => true,
        (Dir::Left, Dir::Right) => true,
        (Dir::Right, Dir::Left) => true,
        _ => false,
    }
}
