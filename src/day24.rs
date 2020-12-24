use std::collections::HashSet;

fn parse_path(s: &str) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    let mut ns = 0;
    for c in s.chars() {
        match (c, ns) {
            ('e', 1) => {
                out.push((0, 1));
                ns = 0;
            }
            ('e', 0) => {
                out.push((1, 0));
                ns = 0;
            }
            ('e', -1) => {
                out.push((1, -1));
                ns = 0;
            }
            ('w', 1) => {
                out.push((-1, 1));
                ns = 0;
            }
            ('w', 0) => {
                out.push((-1, 0));
                ns = 0;
            }
            ('w', -1) => {
                out.push((0, -1));
                ns = 0;
            }
            ('n', 0) => ns = 1,
            ('s', 0) => ns = -1,
            _ => panic!("wut"),
        }
    }
    out
}

fn add_tuple((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x1 + x2, y1 + y2)
}

fn neighbors((x, y): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    vec![
        (x + 0, y + 1),
        (x + 1, y + 0),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y + 0),
        (x + 0, y - 1),
    ]
    .into_iter()
}

fn step(grid: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut candidates: HashSet<(i32, i32)> = grid.iter().copied().flat_map(neighbors).collect();
    candidates.retain(|pt| {
        let black_neighbors = neighbors(*pt).filter(|pt| grid.contains(pt)).count();
        if grid.contains(pt) {
            !(black_neighbors == 0 || black_neighbors > 2)
        } else {
            black_neighbors == 2
        }
    });
    candidates
}

pub fn solve1(input: &[String]) {
    let visited = input
        .iter()
        .map(|s| parse_path(s))
        .map(|path| path.into_iter().fold((0, 0), add_tuple));
    let mut black = HashSet::new();
    for pt in visited {
        if black.contains(&pt) {
            black.remove(&pt);
        } else {
            black.insert(pt);
        }
    }
    dbg!(black.len());
    for _ in 0..100 {
        black = step(black);
    }
    dbg!(black.len());
}
pub fn solve2(input: &[String]) {}
