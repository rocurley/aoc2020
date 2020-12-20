use std::collections::HashMap;
#[derive(Debug, Clone, Eq, PartialEq)]
struct Tile {
    id: usize,
    left: String,
    right: String,
    top: String,
    bottom: String,
    interior: Vec<Vec<bool>>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Top = 0,
    Left = 1,
    Bottom = 2,
    Right = 3,
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
    let mut tiles_by_edge: HashMap<String, Vec<Tile>> = HashMap::new();
    for tile_outer in tiles.iter() {
        for tile in vec![tile_outer.clone(), v_flip(tile_outer.clone())] {
            for d in DIRECTIONS.iter().copied() {
                let edge = tile.get(d).to_owned();
                tiles_by_edge
                    .entry(edge)
                    .or_insert(Vec::new())
                    .push(tile.clone());
            }
        }
    }
    let mut corners: Vec<Tile> = tiles
        .iter()
        .filter(|&tile| {
            ([&tile.left, &tile.right, &tile.top, &tile.bottom])
                .iter()
                .filter(|edge| tiles_by_edge[**edge].len() == 1)
                .count()
                == 2
        })
        .cloned()
        .collect();
    let id_prod: usize = corners.iter().map(|tile| tile.id).product();
    dbg!(id_prod);
    let top_left = corners.pop().unwrap();
    let mut top_left_dirs: Vec<Dir> = DIRECTIONS
        .iter()
        .copied()
        .filter(|dir| edge_is_unique(top_left.get(*dir), &tiles_by_edge))
        .collect();
    top_left_dirs.sort();
    let top_left_rot = match top_left_dirs.as_slice() {
        &[Dir::Top, Dir::Left] => 0,
        &[Dir::Left, Dir::Bottom] => 3,
        &[Dir::Bottom, Dir::Right] => 2,
        &[Dir::Top, Dir::Right] => 1,
        s => panic!("Impossible corner piece: {:?}", s),
    };
    let top_left = rotate_ccw(top_left, top_left_rot);
    let mut top_edge = vec![top_left];
    while let Some((tile, tile_left_dir)) =
        get_matching_tile(top_edge.last().unwrap(), Dir::Right, &tiles_by_edge)
    {
        top_edge.push(rotate_to_match(tile_left_dir, Dir::Left, tile));
    }
    let mut grid = vec![top_edge];
    while grid.iter().map(|row| row.len()).sum::<usize>() < tiles.len() {
        let prior_row = grid.last().unwrap();
        let row = prior_row
            .iter()
            .map(|top_tile| {
                let (tile, top_dir) =
                    get_matching_tile(top_tile, Dir::Bottom, &tiles_by_edge).unwrap();
                rotate_to_match(top_dir, Dir::Top, tile)
            })
            .collect();
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
            for _ in 0..r {
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
    let ans = not_in_sea_monster
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|px| *px)
        .count();
    dbg!(ans);
}

fn get_matching_tile(t: &Tile, d: Dir, index: &HashMap<String, Vec<Tile>>) -> Option<(Tile, Dir)> {
    //dbg!(t, d);
    let e = t.get(d);
    let rev_e = rev_str(e);
    let matching = &index[&rev_e];
    let matching_tile = matching.iter().filter(|t2| t2.id != t.id).nth(0)?.clone();
    let dir = DIRECTIONS
        .iter()
        .copied()
        .filter(|d| rev_e == matching_tile.get(*d))
        .nth(0)
        .unwrap();
    Some((matching_tile, dir))
}

fn edge_is_unique(edge: &str, tiles_by_edge: &HashMap<String, Vec<Tile>>) -> bool {
    tiles_by_edge[edge].len() == 1
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

fn rotate_ccw(mut t: Tile, count: u8) -> Tile {
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

fn rev_str(s: &str) -> String {
    s.chars().rev().collect()
}

fn v_flip(t: Tile) -> Tile {
    Tile {
        id: t.id,
        top: rev_str(&t.bottom),
        left: rev_str(&t.left),
        bottom: rev_str(&t.top),
        right: rev_str(&t.right),
        interior: t.interior.into_iter().rev().collect(),
    }
}

fn rotations_between(from: Dir, to: Dir) -> u8 {
    ((to as u8 + 4) - (from as u8)) % 4
}

fn rotate_to_match(from: Dir, to: Dir, t: Tile) -> Tile {
    rotate_ccw(t, rotations_between(from, to))
}
