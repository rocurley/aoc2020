use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn play(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();
        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }
}

pub fn solve1(input: &[String]) {
    let mut decks = input.split(|line| line == "");
    let mut deck1 = decks.next().unwrap()[1..]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut deck2 = decks.next().unwrap()[1..]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
    play(&mut deck1, &mut deck2);
    let winning_deck = if deck1.is_empty() { deck2 } else { deck1 };
    let score: usize = winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) as usize * card)
        .sum();
    dbg!(score);
}

pub fn solve2(input: &[String]) {
    let mut decks = input.split(|line| line == "");
    let mut deck1 = decks.next().unwrap()[1..]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut deck2 = decks.next().unwrap()[1..]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut cache = HashMap::new();
    let p1_wins = play_rec(&mut deck1, &mut deck2, &mut cache);
    dbg!(&deck1, &deck2);
    let winning_deck = if p1_wins { deck1 } else { deck2 };
    let score: usize = winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) as usize * card)
        .sum();
    dbg!(score);
}

fn play_rec(
    deck1: &mut VecDeque<usize>,
    deck2: &mut VecDeque<usize>,
    cache: &mut HashMap<(VecDeque<usize>, VecDeque<usize>), bool>,
) -> bool {
    //let mut rounds: Vec<(VecDeque<usize>, VecDeque<usize>)> = Vec::new();
    let mut game_len = 0;
    let mut seen = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        game_len += 1;
        if game_len > 1000000 {
            panic!("too long");
        }
        let key = (deck1.clone(), deck2.clone());
        if seen.contains(&key) {
            return true;
        }
        seen.insert(key.clone());
        /*
        if let Some(hit) = cache.get(&key).copied() {
            for key in rounds {
                cache.insert(key, hit);
            }
            return hit;
        }
        */
        //rounds.push(key);
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();
        let p1_wins = if c1 <= deck1.len() && c2 <= deck2.len() {
            let mut subdeck_1 = deck1.iter().copied().take(c1).collect();
            let mut subdeck_2 = deck2.iter().copied().take(c2).collect();
            play_rec(&mut subdeck_1, &mut subdeck_2, cache)
        } else {
            c1 > c2
        };
        if p1_wins {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }
    let out = deck2.is_empty();
    /*
    for key in rounds {
        cache.insert(key, out);
    }
    */
    out
}
