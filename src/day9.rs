pub fn solve2(input: &[String]) {
    let (numbers, target) = solve1(input);
    for i in 0..numbers.len() {
        let mut cur_sum = numbers[i] + numbers[i + 1];
        let mut j = i + 1;
        while cur_sum < target {
            j += 1;
            cur_sum += numbers[j];
        }
        if cur_sum == target {
            let range = &numbers[i..j];
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();
            println!("{}", min + max);
            return;
        }
    }
    println!("Failed to find thing");
}
pub fn solve1(input: &[String]) -> (Vec<i64>, i64) {
    let numbers: Vec<i64> = input
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<_, _>>()
        .unwrap();
    for i in (25..numbers.len()) {
        let trail = &numbers[i - 25..i];
        let x = numbers[i];
        if !can_sum(trail, x) {
            println!("{}", x);
            return (numbers, x);
        }
    }
    unreachable!();
}

fn can_sum(numbers: &[i64], target: i64) -> bool {
    for (i, x) in numbers.iter().enumerate() {
        for y in numbers[i + 1..].iter() {
            if x + y == target {
                return true;
            }
        }
    }
    return false;
}
