pub fn solve1(input: &[String]) {
    let mut prod = 1;
    for (h, v) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let trees = trees_hit(input, *h, *v);
        println!("{}", trees);
        prod *= trees;
    }
    println!("{}", prod);
}

fn trees_hit(input: &[String], h_step: usize, v_step: usize) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, row)| {
            if i % v_step != 0 {
                return false;
            }
            let c = (i / v_step * h_step) % row.len();
            row.chars().nth(c).unwrap() == '#'
        })
        .count()
}
