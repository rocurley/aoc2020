use std::collections::HashSet;
pub fn solve1(input: &[String]) {
    let max = input
        .iter()
        .map(|code| {
            let mut row = 0;
            let cs = code.as_bytes();
            for c in cs[..7].iter() {
                if *c == 'B' as u8 {
                    row += 1;
                }
                row *= 2;
            }
            row /= 2;
            let mut col = 0;
            for c in cs[7..].iter() {
                if *c == 'R' as u8 {
                    col += 1;
                }
                col *= 2;
            }
            col /= 2;
            row * 8 + col
        })
        .max();
    println!("{:?}", max);
}
pub fn solve2(input: &[String]) {
    let seats: Vec<(i32, i32)> = input
        .iter()
        .map(|code| {
            let mut row = 0;
            let cs = code.as_bytes();
            for c in cs[..7].iter() {
                if *c == 'B' as u8 {
                    row += 1;
                }
                row *= 2;
            }
            row /= 2;
            let mut col = 0;
            for c in cs[7..].iter() {
                if *c == 'R' as u8 {
                    col += 1;
                }
                col *= 2;
            }
            col /= 2;
            (row, col)
        })
        .collect();
    let max_row = seats.iter().map(|(row, _)| *row).max().unwrap();
    let min_row = seats.iter().map(|(row, _)| *row).min().unwrap();
    dbg!(max_row);
    let mut missing: HashSet<(i32, i32)> = (min_row + 1..max_row)
        .flat_map(|row| (0..8).map(move |col| (row, col)))
        .collect();
    for seat in seats.iter() {
        missing.remove(seat);
    }
    dbg!(missing);
}
