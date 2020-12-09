use regex::Regex;
use std::collections::HashMap;
const MANDATORY: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn solve1(input: &[String]) {
    let re1: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let re2: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    let str = input.join("\n");
    let c = str
        .split("\n\n")
        .filter(|passport| {
            let seen: HashMap<&str, &str> = passport
                .split_whitespace()
                .map(|pair| {
                    let mut kv = pair.split(":");
                    let k = kv.next().unwrap();
                    let v = kv.next().unwrap();
                    (k, v)
                })
                .collect();
            if !MANDATORY.iter().all(|k| seen.contains_key(k)) {
                return false;
            }
            let byr = seen["byr"].parse::<i32>().unwrap();
            if !(1920..=2002).contains(&byr) {
                return false;
            }
            let iyr = seen["iyr"].parse::<i32>().unwrap();
            if !(2010..=2020).contains(&iyr) {
                return false;
            }
            let eyr = seen["eyr"].parse::<i32>().unwrap();
            if !(2020..=2030).contains(&eyr) {
                return false;
            }
            let hgt = seen["hgt"];
            let hgt_valid = match (hgt.strip_suffix("cm"), hgt.strip_suffix("in")) {
                (Some(cm), _) => (150..=193).contains(&cm.parse::<i32>().unwrap()),
                (_, Some(inches)) => (59..=76).contains(&inches.parse::<i32>().unwrap()),
                (None, None) => false,
            };
            if !hgt_valid {
                return false;
            }
            if !re1.is_match(seen["hcl"]) {
                return false;
            }
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&seen["ecl"]) {
                return false;
            }
            if !re2.is_match(seen["pid"]) {
                return false;
            }
            return true;
        })
        .count();
    println!("{}", c);
}
