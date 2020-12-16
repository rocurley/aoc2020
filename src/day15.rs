use std::collections::HashMap;
pub fn solve1(input: &[String]) {
    let mut last_said = HashMap::new();
    let mut next = 0;
    for (i, line) in input.iter().enumerate() {
        let n: usize = line.parse().unwrap();
        next = i - last_said.insert(n, i).unwrap_or(i);
    }
    for i in input.len()..(30000000 - 1) {
        next = i - last_said.insert(next, i).unwrap_or(i);
    }
    println!("{}", next);
}
