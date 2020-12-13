use std::convert::TryInto;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Seat,
    Floor,
    Occupied,
}
pub fn solve1(input: &[String]) {
    let mut seats: Vec<Vec<Tile>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Tile::Seat,
                    '.' => Tile::Floor,
                    '#' => Tile::Occupied,
                    _ => panic!("Unexpected char {}", c),
                })
                .collect()
        })
        .collect();
    loop {
        let next = step(&seats);
        //dbg!(&next);
        if next == seats {
            break;
        }
        seats = next;
    }
    let occupied_count = seats
        .iter()
        .flat_map(|row| row.iter())
        .filter(|seat| **seat == Tile::Occupied)
        .count();
    println!("{}", occupied_count);
}

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1).filter_map(move |dy| {
            if dx == 0 && dy == 0 {
                return None;
            }
            let x: usize = (x as i32 + dx).try_into().ok()?;
            let y: usize = (y as i32 + dy).try_into().ok()?;
            Some((x, y))
        })
    })
}

fn step(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, seat)| {
                    let occupied_neighbors = neighbors(x, y)
                        .filter_map(|(x, y)| Some(*map.get(y)?.get(x)? == Tile::Occupied))
                        .filter(|b| *b)
                        .count();
                    if x == 3 && y == 0 {
                        let ns: Vec<_> = neighbors(x, y)
                            .filter_map(|(x, y)| Some((x, y, *map.get(y)?.get(x)?)))
                            .collect();
                        //dbg!(*seat, ns, occupied_neighbors);
                    }
                    match (seat, occupied_neighbors) {
                        (Tile::Seat, 0) => Tile::Occupied,
                        (Tile::Occupied, n) if n >= 4 => Tile::Seat,
                        _ => *seat,
                    }
                })
                .collect()
        })
        .collect()
}

fn directions() -> impl Iterator<Item = (i32, i32)> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1).filter_map(move |dy| {
            if dx == 0 && dy == 0 {
                return None;
            }
            Some((dx, dy))
        })
    })
}

fn first_in_direction(
    mut x: usize,
    mut y: usize,
    (dx, dy): (i32, i32),
    seats: &Vec<Vec<Tile>>,
) -> Option<Tile> {
    loop {
        x = (x as i32 + dx).try_into().ok()?;
        y = (y as i32 + dy).try_into().ok()?;
        let tile = seats.get(y)?.get(x)?;
        if *tile != Tile::Floor {
            return Some(*tile);
        }
    }
}

pub fn solve2(input: &[String]) {
    let mut seats: Vec<Vec<Tile>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Tile::Seat,
                    '.' => Tile::Floor,
                    '#' => Tile::Occupied,
                    _ => panic!("Unexpected char {}", c),
                })
                .collect()
        })
        .collect();
    loop {
        let next = step2(&seats);
        //dbg!(&next);
        if next == seats {
            break;
        }
        seats = next;
    }
    let occupied_count = seats
        .iter()
        .flat_map(|row| row.iter())
        .filter(|seat| **seat == Tile::Occupied)
        .count();
    println!("{}", occupied_count);
}

fn step2(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, seat)| {
                    let occupied_neighbors = directions()
                        .filter_map(|d| first_in_direction(x, y, d, map))
                        .filter(|s| *s == Tile::Occupied)
                        .count();
                    match (seat, occupied_neighbors) {
                        (Tile::Seat, 0) => Tile::Occupied,
                        (Tile::Occupied, n) if n >= 5 => Tile::Seat,
                        _ => *seat,
                    }
                })
                .collect()
        })
        .collect()
}
