use regex::Regex;

#[derive(Debug, Clone)]
struct FieldSpec {
    name: String,
    range1: (usize, usize),
    range2: (usize, usize),
}
impl FieldSpec {
    fn matches(&self, x: usize) -> bool {
        return (self.range1.0 <= x && x <= self.range1.1)
            || (self.range2.0 <= x && x <= self.range2.1);
    }
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(",").map(|n| n.parse().unwrap()).collect()
}

pub fn solve2(input: &[String]) {
    let str = input.join("\n");
    let mut split = str.split("\n\n");
    let fields = split.next().unwrap();
    let mine = split.next().unwrap();
    let nearby = split.next().unwrap();
    let fields_re: Regex = Regex::new(r"^([^:]*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
    let fields: Vec<FieldSpec> = fields
        .split("\n")
        .map(|line| {
            let captures = fields_re.captures(line).unwrap();
            FieldSpec {
                name: captures[1].to_owned(),
                range1: (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
                range2: (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
            }
        })
        .collect();
    let nearby: Vec<Vec<usize>> = nearby.split("\n").skip(1).map(parse_ticket).collect();
    let mine: Vec<usize> = parse_ticket(mine.split("\n").skip(1).next().unwrap());
    let valid: Vec<Vec<usize>> = nearby
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|x| fields.iter().any(|field| field.matches(*x)))
        })
        .collect();
    let mut possibilities: Vec<(String, Vec<usize>)> = fields
        .into_iter()
        .map(|field| {
            (
                field.name.clone(),
                (0..valid[0].len())
                    .filter(|i| valid.iter().all(|ticket| field.matches(ticket[*i])))
                    .collect(),
            )
        })
        .collect();
    let mut confirmed = Vec::new();
    while possibilities.len() > 0 {
        let i = possibilities
            .iter()
            .position(|(_, options)| options.len() == 1)
            .unwrap();
        let (name, options) = possibilities.swap_remove(i);
        let field_ix = options[0];
        confirmed.push((name, field_ix));
        for fields in possibilities.iter_mut() {
            fields.1.retain(|i| *i != field_ix);
        }
    }
    let mut out = 1;
    for (field, ix) in confirmed {
        if field.starts_with("departure") {
            out *= mine[ix];
        }
    }
    println!("{}", out);
}

pub fn solve1(input: &[String]) {
    let str = input.join("\n");
    let mut split = str.split("\n\n");
    let fields = split.next().unwrap();
    let _mine = split.next().unwrap();
    let nearby = split.next().unwrap();
    let fields_re: Regex = Regex::new(r"^([^:]*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
    let fields: Vec<FieldSpec> = fields
        .split("\n")
        .map(|line| {
            let captures = fields_re.captures(line).unwrap();
            FieldSpec {
                name: captures[1].to_owned(),
                range1: (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
                range2: (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
            }
        })
        .collect();
    let nearby: Vec<Vec<usize>> = nearby.split("\n").skip(1).map(parse_ticket).collect();
    let out: usize = nearby
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|x| !fields.iter().any(|field| field.matches(**x)))
        .sum();
    println!("{:?}", out);
}
