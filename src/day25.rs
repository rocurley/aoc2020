fn crack(key: u64) -> u64 {
    let mut n = 1;
    let mut i = 0;
    while n != key {
        i += 1;
        n = (n * 7) % 20201227
    }
    i
}

fn merge(loop_num: u64, other_key: u64) -> u64 {
    let mut n = 1;
    for _ in 0..loop_num {
        n = (n * other_key) % 20201227
    }
    n
}

pub fn solve1(input: &[String]) {
    let door_key: u64 = input[0].parse().unwrap();
    let card_key: u64 = input[1].parse().unwrap();
    let door_loop_n = crack(door_key);
    let key = merge(door_loop_n, card_key);
    dbg!(key);
}
pub fn solve2(input: &[String]) {}
