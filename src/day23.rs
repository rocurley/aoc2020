fn show(cups: &[(usize, usize)]) -> String {
    let mut out = String::new();
    let mut i = 1;
    loop {
        i = cups[i].1;
        if i == 1 {
            break;
        }
        out.push_str(&format!("{}", i));
    }
    out
}

fn to_linked(cups: &[usize]) -> Vec<(usize, usize)> {
    let mut cups_linked = vec![(0, 0); cups.len() + 1];
    for window in cups.windows(3) {
        cups_linked[window[1] as usize] = (window[0], window[2]);
    }
    cups_linked[cups[0] as usize] = (cups[cups.len() - 1], cups[1]);
    cups_linked[cups[cups.len() - 1] as usize] = (cups[cups.len() - 2], cups[0]);
    cups_linked
}

pub fn solve1(input: &[String]) {
    let cups: Vec<usize> = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut current_cup = cups[0];
    let mut cups = to_linked(&cups);
    for _ in 0..100 {
        step(&mut cups, &mut current_cup);
    }
    println!("{}", show(&cups));
}

fn step(cups: &mut Vec<(usize, usize)>, current_cup: &mut usize) {
    let mut dest_cup = if *current_cup == 1 {
        cups.len() - 1
    } else {
        *current_cup - 1
    };
    let grab1 = cups[*current_cup].1;
    let grab2 = cups[grab1].1;
    let grab3 = cups[grab2].1;
    while vec![grab1, grab2, grab3].contains(&dest_cup) {
        dest_cup = if dest_cup == 1 {
            cups.len() - 1
        } else {
            dest_cup - 1
        };
    }
    cups[*current_cup].1 = cups[grab3].1;
    let post_dest = cups[dest_cup].1;
    cups[dest_cup].1 = grab1;
    cups[grab1] = (dest_cup, grab2);
    cups[grab2] = (grab1, grab3);
    cups[grab3] = (grab2, post_dest);
    cups[post_dest].0 = grab3;
    *current_cup = cups[*current_cup].1;
}

pub fn solve2(input: &[String]) {
    let mut cups: Vec<usize> = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    cups.extend(cups.iter().copied().max().unwrap() + 1..=1_000_000);
    let mut current_cup = cups[0];
    let mut cups = to_linked(&cups);
    for _ in 0..10_000_000 {
        step(&mut cups, &mut current_cup);
    }
    let l1 = cups[1].1;
    let l2 = cups[l1].1;
    dbg!(l1 * l2);
}
