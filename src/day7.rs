use regex::Regex;
use std::collections::{HashMap, HashSet};
pub fn solve1(input: &[String]) {
    let re1: Regex = Regex::new(r"^(\w* \w*) bags contain (.*).$").unwrap();
    let re2: Regex = Regex::new(r"^(\d*) (\w* \w*) bags?$").unwrap();
    let edges: HashMap<_, _> = input
        .iter()
        .map(|line| {
            let captures = re1.captures(line).unwrap();
            let color = &captures[1];
            let contents = &captures[2];
            let contents: Vec<(String, i32)> = if contents == "no other bags" {
                Vec::new()
            } else {
                contents
                    .split(", ")
                    .map(|contained| {
                        let captures = re2.captures(contained).unwrap();
                        let count: i32 = captures[1].parse().unwrap();
                        let color = captures[2].to_owned();
                        (color, count)
                    })
                    .collect()
            };
            (color.to_owned(), contents)
        })
        .collect();
    {
        let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();
        for (container, contents) in edges.iter() {
            for (content_color, _) in contents {
                contained_by
                    .entry(content_color.clone())
                    .or_insert(HashSet::new())
                    .insert(container.clone());
            }
        }
        dbg!(&contained_by["shiny gold"]);
        let mut seen: HashSet<String> = HashSet::new();
        let mut stack = vec!["shiny gold".to_owned()];
        while let Some(color) = stack.pop() {
            if seen.contains(&color) {
                continue;
            }
            for new_color in contained_by.get(&color).unwrap_or(&HashSet::new()) {
                stack.push(new_color.clone());
            }
            seen.insert(color);
        }
        dbg!(seen.len() - 1);
    }
    let mut stack = vec![("shiny gold".to_owned(), 1)];
    let mut total = 0;
    while let Some((color, count)) = stack.pop() {
        for (new_color, new_count) in edges.get(&color).unwrap() {
            stack.push((new_color.clone(), count * new_count));
        }
        total += count;
    }
    dbg!(total - 1);
}
