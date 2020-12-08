use std::collections::HashSet;
pub fn solve1(input: &[String]) {
    let str = input.join("\n");
    let total: usize = str
        .split("\n\n")
        .map(|set| {
            let mut xs = set.split("\n").map(|line| line.chars().collect());
            let mut all: HashSet<char> = xs.next().unwrap();
            for x in xs {
                all = all.intersection(&x).copied().collect();
            }
            all.len()
        })
        .sum();
    println!("{}", total);
}
