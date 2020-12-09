use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn step(&self) -> i32 {
        match self {
            Instruction::Jmp(j) => *j,
            _ => 1,
        }
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut split = line.split(" ");
    let instr = split.next().unwrap();
    let n = split.next().unwrap().parse().unwrap();
    match instr {
        "acc" => Instruction::Acc(n),
        "jmp" => Instruction::Jmp(n),
        "nop" => Instruction::Nop(n),
        _ => unimplemented!(),
    }
}

pub fn solve1(input: &[String]) {
    let instructions: Vec<_> = input.iter().map(|s| parse_instruction(&s)).collect();
    let mut seen = HashSet::new();
    let mut i = 0i32;
    let mut acc = 0;
    loop {
        if seen.contains(&i) {
            println!("{}", acc);
            return;
        }
        seen.insert(i);
        let instruction = instructions[i as usize];
        if let Instruction::Acc(x) = instruction {
            acc += x;
        }
        i += instruction.step();
    }
}

fn swap_instruction(i: Instruction) -> Instruction {
    match i {
        Instruction::Acc(x) => Instruction::Acc(x),
        Instruction::Jmp(x) => Instruction::Nop(x),
        Instruction::Nop(x) => Instruction::Jmp(x),
    }
}

pub fn solve2(input: &[String]) {
    let mut instructions: Vec<_> = input.iter().map(|s| parse_instruction(&s)).collect();
    let mut comefrom: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, instruction) in instructions.iter().copied().enumerate() {
        let dest = i as i32 + instruction.step();
        comefrom.entry(dest).or_insert_with(Vec::new).push(i as i32);
    }
    let mut can_reach_end: HashSet<i32> = HashSet::new();
    let mut stack = vec![instructions.len() as i32];
    while let Some(i) = stack.pop() {
        if can_reach_end.contains(&i) {
            continue;
        }
        can_reach_end.insert(i);
        if let Some(can_reach) = comefrom.get(&(i as i32)) {
            stack.extend(can_reach.iter().copied());
        }
    }
    let mut i = 0i32;
    let mut acc = 0;
    loop {
        let instruction = &mut instructions[i as usize];
        let swapped_next_i = i + swap_instruction(*instruction).step();
        if can_reach_end.contains(&swapped_next_i) {
            *instruction = swap_instruction(*instruction);
            break;
        }
        if let Instruction::Acc(x) = instruction {
            acc += *x;
        }
        i += instruction.step();
    }
    loop {
        if i == instructions.len() as i32 {
            println!("{}", acc);
            return;
        }
        let instruction = instructions[i as usize];
        if let Instruction::Acc(x) = instruction {
            acc += x;
        }
        i += instruction.step();
    }
}
