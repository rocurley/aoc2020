use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
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
        match instructions[i as usize] {
            Instruction::Acc(x) => acc += x,
            Instruction::Jmp(j) => i += j - 1,
            Instruction::Nop(_) => {}
        }
        i += 1;
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
    for i in (0..instructions.len()) {
        instructions[i] = swap_instruction(instructions[i]);
        match terminates(&instructions) {
            (false, _) => {}
            (true, ret) => {
                println!("{}", ret);
                return;
            }
        }
        instructions[i] = swap_instruction(instructions[i]);
    }
}

fn terminates(instrs: &[Instruction]) -> (bool, i32) {
    let mut seen = HashSet::new();
    let mut i = 0i32;
    let mut acc = 0;
    loop {
        if seen.contains(&i) {
            return (false, acc);
        }
        seen.insert(i);
        if i as usize > instrs.len() {
            return (false, acc);
        }
        if i as usize == instrs.len() {
            return (true, acc);
        }
        match instrs[i as usize] {
            Instruction::Acc(x) => acc += x,
            Instruction::Jmp(j) => i += j - 1,
            Instruction::Nop(_) => {}
        }
        i += 1;
    }
}
