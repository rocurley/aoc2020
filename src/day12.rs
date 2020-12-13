#[derive(Clone, Copy, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn left(d: Dir) -> Dir {
    match d {
        Dir::N => Dir::W,
        Dir::S => Dir::E,
        Dir::E => Dir::N,
        Dir::W => Dir::S,
    }
}
fn right(d: Dir) -> Dir {
    match d {
        Dir::N => Dir::E,
        Dir::S => Dir::W,
        Dir::E => Dir::S,
        Dir::W => Dir::N,
    }
}

fn mv(d: Dir, count: i32, x: i32, y: i32) -> (i32, i32) {
    match d {
        Dir::N => (x, y + count),
        Dir::S => (x, y - count),
        Dir::E => (x + count, y),
        Dir::W => (x - count, y),
    }
}

pub fn solve1(input: &[String]) {
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::E;
    for line in input {
        let count = line[1..].parse().unwrap();
        let d = match &line[0..1] {
            "N" => Dir::N,
            "S" => Dir::S,
            "E" => Dir::E,
            "W" => Dir::W,
            "F" => dir,
            "L" => {
                for _ in 0..count / 90 {
                    dir = left(dir);
                }
                continue;
            }
            "R" => {
                for _ in 0..count / 90 {
                    dir = right(dir);
                }
                continue;
            }
            c => panic!("Weird direction {}", c),
        };
        let (x2, y2) = mv(d, count, x, y);
        x = x2;
        y = y2;
    }
    dbg!(x, y);
    println!("{}", x.abs() + y.abs());
}

fn rotate_left((x, y): (i32, i32)) -> (i32, i32) {
    (-y, x)
}
fn rotate_right((x, y): (i32, i32)) -> (i32, i32) {
    (y, -x)
}

pub fn solve2(input: &[String]) {
    let mut x = 0;
    let mut y = 0;
    let mut wp = (10, 1);
    for line in input {
        dbg!(x, y, wp);
        let count = line[1..].parse().unwrap();
        let d = match &line[0..1] {
            "N" => Dir::N,
            "S" => Dir::S,
            "E" => Dir::E,
            "W" => Dir::W,
            "F" => {
                x += wp.0 * count;
                y += wp.1 * count;
                continue;
            }
            "L" => {
                for _ in 0..count / 90 {
                    wp = rotate_left(wp);
                }
                continue;
            }
            "R" => {
                for _ in 0..count / 90 {
                    wp = rotate_right(wp);
                }
                continue;
            }
            c => panic!("Weird direction {}", c),
        };
        wp = mv(d, count, wp.0, wp.1);
    }
    dbg!(x, y);
    println!("{}", x.abs() + y.abs());
}
