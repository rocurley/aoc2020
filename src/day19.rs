use regex::Regex;
use std::collections::HashMap;
use std::str::Chars;

pub fn solve1(input: &[String]) {
    let mut split = input.split(|line| line == "");
    let rules_lines = split.next().unwrap();
    let strings = split.next().unwrap();
    let rules: HashMap<usize, Rule> = rules_lines.iter().map(parse_rule).collect();
    let match_count = strings
        .iter()
        .filter(|s| match_rule_terminal(0, &rules, s))
        .count();
    dbg!(match_count);
}

pub fn solve2(input: &[String]) {
    let mut split = input.split(|line| line == "");
    let rules_lines = split.next().unwrap();
    let strings = split.next().unwrap();
    let mut rules: HashMap<usize, Rule> = rules_lines.iter().map(parse_rule).collect();
    rules.insert(8, Rule::SubRules(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]));
    let match_count = strings
        .iter()
        .filter(|s| match_rule_terminal(0, &rules, s))
        .count();
    dbg!(match_count);
}

fn parse_rule(s: &String) -> (usize, Rule) {
    let compound_rule_re: Regex = Regex::new(r"^(\d+):(( \d+)+)( \|(( \d+)+))?$").unwrap();
    let lit_rule_re: Regex = Regex::new(r#"^(\d+): "(.)"$"#).unwrap();
    if let Some(captures) = compound_rule_re.captures(s) {
        let ix = captures[1].parse().unwrap();
        let rules_1 = captures[2]
            .trim()
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();
        let rules = match captures.get(5) {
            Some(cap) => {
                let rules_2 = cap
                    .as_str()
                    .trim()
                    .split(" ")
                    .map(|n| n.parse().unwrap())
                    .collect();
                vec![rules_1, rules_2]
            }
            None => vec![rules_1],
        };
        (ix, Rule::SubRules(rules))
    } else {
        let captures = lit_rule_re.captures(s).unwrap();
        let ix = captures[1].parse().unwrap();
        let c = captures[2].parse().unwrap();
        (ix, Rule::Lit(c))
    }
}

#[derive(Debug)]
enum Rule {
    Lit(char),
    SubRules(Vec<Vec<usize>>),
}

fn match_rule_terminal(rule_ix: usize, rules: &HashMap<usize, Rule>, s: &str) -> bool {
    match_rule(rule_ix, rules, s.chars())
        .into_iter()
        .any(|mut remaining| remaining.next().is_none())
}

fn match_rule<'a>(
    rule_ix: usize,
    rules: &HashMap<usize, Rule>,
    mut s: Chars<'a>,
) -> Vec<Chars<'a>> {
    let rule = &rules[&rule_ix];
    match rule {
        Rule::Lit(c) => {
            let c2 = match s.next() {
                Some(c2) => c2,
                None => return Vec::new(),
            };
            if *c == c2 {
                vec![s]
            } else {
                Vec::new()
            }
        }
        Rule::SubRules(or_rules) => or_rules
            .iter()
            .flat_map(|and_rules| {
                and_rules
                    .into_iter()
                    .fold(vec![s.clone()], |local_ss, sub_rule| {
                        local_ss
                            .into_iter()
                            .flat_map(|s| match_rule(*sub_rule, rules, s).into_iter())
                            .collect()
                    })
                    .into_iter()
            })
            .collect(),
    }
}
