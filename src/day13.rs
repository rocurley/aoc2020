pub fn solve1(input: &[String]) {
    let start = input[0].parse::<i32>().unwrap();
    let busses: Vec<i32> = input[1]
        .split(",")
        .filter(|s| *s != "x")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let (wait, id) = busses
        .iter()
        .map(|id| (id - (start % id), id))
        .min()
        .unwrap();
    dbg!(wait, id);
    println!("{}", wait * id);
}

fn real_mod(x: i64, y: i64) -> i64 {
    let mut r = x % y;
    if r < 0 {
        r += y;
    }
    r
}

fn bezout(n: i64, N: i64) -> (i64, i64) {
    let mut M = 0;
    let mut m = 0;
    use std::cmp::Ordering;
    loop {
        match (M * N + m * n).cmp(&1) {
            Ordering::Greater => m -= 1,
            Ordering::Equal => return (m, M),
            Ordering::Less => M += 1,
        }
    }
}

fn bezout_2(n: i64, N: i64) -> (i64, i64) {
    let mut M = 0;
    let mut m = 0;
    use std::cmp::Ordering;
    loop {
        let current = M * N + m * n;
        match (current).cmp(&1) {
            Ordering::Greater => m -= (current - 1 + n - 1) / n,
            Ordering::Equal => {
                return (m, M);
            }
            Ordering::Less => M += (1 - current + N - 1) / N,
        }
    }
}

fn crt(busses: Vec<(i64, i64)>) -> i64 {
    let N: i64 = busses.iter().map(|(_, id)| id).product();
    dbg!(N);
    let out: i64 = busses
        .iter()
        .map(|(a, n)| {
            let Ni = N / n;
            dbg!(*n, Ni);
            let (_, M) = dbg!(bezout_2(*n, Ni));
            dbg!(a * Ni * M)
        })
        .sum();
    out % N
}

pub fn solve2(input: &[String]) {
    let busses: Vec<(i64, i64)> = input[1]
        .split(",")
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i as i64, s.parse::<i64>().unwrap()))
        .map(|(i, s)| (real_mod(s - i, s), s))
        .collect();
    println!("{}", crt(busses));
    /*
    let (max_bus_id, max_bus_ix) = busses.iter().map(|(i, id)| (id, i)).max().unwrap();
    for (bus_ix, bus_id) in busses.iter() {
        println!(
            "? = {} mod {}",
            real_mod(bus_id - *bus_ix as i64, *bus_id),
            bus_id
        );
    }
    dbg!(max_bus_id, max_bus_ix);
    'outer: for i in 0.. {
        let t = max_bus_id * i + real_mod(max_bus_id - *max_bus_ix as i64, *max_bus_id as i64);
        for (bus_ix, bus_id) in busses.iter() {
            if t % bus_id != (bus_id - *bus_ix as i64) % bus_id {
                continue 'outer;
            }
        }
        for (bus_ix, bus_id) in busses.iter() {
            println!("{} = {} mod {}", t, bus_id - *bus_ix as i64, bus_id);
        }
        println!("{}", t);
        return;
    }
        */
}
