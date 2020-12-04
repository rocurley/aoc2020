use std::collections::HashSet;
pub fn solve1(input: &[String]) {
    let mut seen = HashSet::new();
    for line in input {
        let n = line.parse::<i64>().expect("Couldn't parse line as int");
        let m = 2020 - n;
        if seen.contains(&m) {
            println!("{} x {} = {}", n, m, n * m);
            return;
        }
        seen.insert(n);
    }
    println!("No matches");
}

pub fn solve2(input: &[String]) {
    let mut seen = HashSet::new();
    for line in input {
        let n = line.parse::<i64>().expect("Couldn't parse line as int");
        for m in seen.iter() {
            let l = 2020 - n - m;
            if seen.contains(&l) {
                println!("{} x {} x {} = {}", n, m, l, n * m * l);
                return;
            }
        }
        seen.insert(n);
    }
    println!("No matches");
}
