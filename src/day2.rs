use regex::Regex;

pub fn solve1(input: &[String]) {
    let re: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w*)$").unwrap();
    let valid = input
        .iter()
        .filter(|line| {
            let m = re.captures(line).unwrap();
            let min: usize = m[1].parse().unwrap();
            let max: usize = m[2].parse().unwrap();
            let c = m[3].chars().next().unwrap();
            let password = &m[4];
            let count = password.chars().filter(|c2| c == *c2).count();
            (min..=max).contains(&count)
        })
        .count();
    println!("{}", valid);
}

pub fn solve2(input: &[String]) {
    let re: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w*)$").unwrap();
    let valid = input
        .iter()
        .filter(|line| {
            let m = re.captures(line).unwrap();
            let ix1: usize = m[1].parse().unwrap();
            let ix2: usize = m[2].parse().unwrap();
            let c = m[3].chars().next().unwrap();
            let password = &m[4];
            let ix1_matches = password.chars().nth(ix1 - 1) == Some(c);
            let ix2_matches = password.chars().nth(ix2 - 1) == Some(c);
            ix1_matches != ix2_matches
        })
        .count();
    println!("{}", valid);
}
