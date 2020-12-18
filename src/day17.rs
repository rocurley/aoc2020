use std::collections::HashSet;
fn neighbors([x, y, z, w]: [i32; 4]) -> impl Iterator<Item = [i32; 4]> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1).flat_map(move |dy| {
            (-1..=1).flat_map(move |dz| {
                (-1..=1).filter_map(move |dw| {
                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        return None;
                    }
                    Some([x + dx, y + dy, z + dz, w + dw])
                })
            })
        })
    })
}
fn step(grid: HashSet<[i32; 4]>) -> HashSet<[i32; 4]> {
    let mut candidates: HashSet<_> = grid.iter().cloned().flat_map(neighbors).collect();
    candidates.retain(|pos| {
        let active = grid.contains(pos);
        let active_nearby = neighbors(*pos).filter(|pos| grid.contains(pos)).count();
        if active {
            (2..=3).contains(&active_nearby)
        } else {
            active_nearby == 3
        }
    });
    candidates
}

pub fn solve1(input: &[String]) {
    let mut state: HashSet<_> = input
        .iter()
        .enumerate()
        .flat_map(move |(x, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y, _)| [x as i32, y as i32, 0, 0])
        })
        .collect();
    for _ in 0..6 {
        state = step(state);
    }
    dbg!(state.len());
}
