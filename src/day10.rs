use std::collections::HashMap;

pub fn solve1(input: &[String]) {
    let mut joltages: Vec<i32> = input
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()
        .unwrap();
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);
    joltages.insert(0, 0);
    let mut ones = 0;
    let mut threes = 0;
    for js in joltages.windows(2) {
        let (j1, j2) = if let [j1, j2] = js {
            (j1, j2)
        } else {
            unreachable!();
        };
        match j2 - j1 {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }
    println!("{}", ones * threes);
}
pub fn solve2(input: &[String]) {
    let mut joltages: Vec<i32> = input
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()
        .unwrap();
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);
    joltages.insert(0, 0);
    let mut ways_to: HashMap<i32, usize> = HashMap::new();
    ways_to.insert(0, 1);
    for j in &joltages[1..] {
        let ways = ways_to.get(&(j - 1)).unwrap_or(&0)
            + ways_to.get(&(j - 2)).unwrap_or(&0)
            + ways_to.get(&(j - 3)).unwrap_or(&0);
        ways_to.insert(*j, ways);
    }
    let mut ways_vec = ways_to.iter().collect::<Vec<_>>();
    ways_vec.sort();
    dbg!(ways_vec);
    println!("{}", ways_to[joltages.last().unwrap()]);
}
