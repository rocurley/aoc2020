use std::collections::HashMap;
pub fn solve1(input: &[String]) {
    let mut last_said = HashMap::new();
    let mut next = 0;
    for (i, line) in input.iter().enumerate() {
        let n: usize = line.parse().unwrap();
        next = i - *last_said.get(&n).unwrap_or(&i);
        last_said.insert(n, i);
    }
    for i in input.len()..(30000000 - 1) {
        let new_next = i - *last_said.get(&next).unwrap_or(&i);
        last_said.insert(next, i);
        next = new_next;
    }
    println!("{}", next);
}
