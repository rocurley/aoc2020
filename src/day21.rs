use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    alergens: Vec<String>,
}

fn parse_food(s: &str) -> Food {
    let mut split = s.split(" (contains ");
    let ingredients = split
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.to_owned())
        .collect();
    let alergens_str = split.next().unwrap();
    let alergens = alergens_str[..alergens_str.len() - 1]
        .split(", ")
        .map(|s| s.to_owned())
        .collect();
    Food {
        ingredients,
        alergens,
    }
}

pub fn solve1(input: &[String]) {
    let mut possible_alergens: HashMap<String, HashSet<String>> = HashMap::new();
    let mut foods: Vec<Food> = input.iter().map(|s| parse_food(s)).collect();
    for food in foods.iter() {
        for alergen in &food.alergens {
            match possible_alergens.entry(alergen.clone()) {
                Entry::Vacant(e) => {
                    e.insert(food.ingredients.iter().cloned().collect());
                }
                Entry::Occupied(mut e) => {
                    let set = e.get_mut();
                    *set = food
                        .ingredients
                        .iter()
                        .cloned()
                        .filter(|s| set.contains(s))
                        .collect();
                }
            }
        }
    }
    let possible_alergen_ingredients: HashSet<_> = possible_alergens
        .iter()
        .flat_map(|(_, ingredients)| ingredients.iter())
        .cloned()
        .collect();
    let impossible_alergen_count = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|ingredient| !possible_alergen_ingredients.contains(*ingredient))
        .count();
    dbg!(impossible_alergen_count);
    for food in foods.iter_mut() {
        food.ingredients
            .retain(|ingredient| possible_alergen_ingredients.contains(ingredient));
    }
    dbg!(foods);
    dbg!(&possible_alergens);
    let mut known_alergens = HashMap::new();
    while !possible_alergens.is_empty() {
        let mut iter = possible_alergens.iter();
        let alergen = loop {
            let (alergen, possible_ingredients) = iter.next().unwrap();
            if possible_ingredients.len() == 1 {
                break alergen.clone();
            }
        };
        drop(iter);
        let ingredient = possible_alergens
            .remove(&alergen)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        for (_, ingredients) in possible_alergens.iter_mut() {
            ingredients.remove(&ingredient);
        }
        known_alergens.insert(alergen, ingredient);
    }
    dbg!(&known_alergens);
    let mut sorted_alergens: Vec<(String, String)> = known_alergens.into_iter().collect();
    sorted_alergens.sort();
    let out = sorted_alergens
        .into_iter()
        .map(|(_, ing)| ing)
        .collect::<Vec<String>>()
        .join(",");
    dbg!(out);
}
