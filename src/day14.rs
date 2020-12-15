use regex::Regex;
use std::collections::HashMap;

fn collect_bits<I: Iterator<Item = bool>>(bits: I) -> u64 {
    let mut out = 0;
    for b in bits {
        out *= 2;
        if b {
            out += 1;
        }
    }
    out
}

/*
pub fn addresses(ones : u64, zeros: u64, floating: u64) {

}
*/

fn bits(x: u64) -> Vec<bool> {
    let mut out = vec![false; 36];
    for i in 0..36 {
        if (x & 1 << (35 - i)) > 0 {
            out[i] = true;
        }
    }
    out
}

fn addresses<I: Iterator<Item = char>, I2: Iterator<Item = bool>>(mask: I, addr: I2) -> Vec<u64> {
    let mut out = vec![0];
    for (m, a) in mask.zip(addr) {
        match m {
            '0' => {
                for x in out.iter_mut() {
                    *x *= 2;
                    if a {
                        *x += 1;
                    }
                }
            }
            '1' => {
                for x in out.iter_mut() {
                    *x *= 2;
                    *x += 1;
                }
            }
            'X' => {
                let mut new = Vec::new();
                for x in out.iter() {
                    new.push(2 * x);
                    new.push(2 * x + 1);
                }
                out = new;
            }
            _ => unreachable!(),
        }
    }
    out
}

pub fn solve1(input: &[String]) {
    let re1: Regex = Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
    let mut mask_set = 0;
    let mut mask_clear = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in input {
        if let Some(mask_str) = line.strip_prefix("mask = ") {
            mask_set = collect_bits(mask_str.chars().map(|c| c == '1'));
            mask_clear = collect_bits(mask_str.chars().map(|c| c == '0'));
        } else {
            let captures = re1.captures(line).unwrap();
            let addr = captures[1].parse::<u64>().unwrap();
            let x = captures[2].parse::<u64>().unwrap();
            *mem.entry(addr).or_insert(0) = (x | mask_set) & !mask_clear;
        }
    }
    let s: u64 = mem.values().sum();
    println!("{}", s);
}

pub fn solve2(input: &[String]) {
    let re1: Regex = Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
    let mut mask = "000000000000000000000000000000000000";
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in input {
        if let Some(mask_str) = line.strip_prefix("mask = ") {
            mask = mask_str;
        } else {
            let captures = re1.captures(line).unwrap();
            let addr = captures[1].parse::<u64>().unwrap();
            let x = captures[2].parse::<u64>().unwrap();
            for addr in addresses(mask.chars(), bits(addr).into_iter()) {
                mem.insert(addr, x);
            }
        }
    }
    let s: u64 = mem.values().sum();
    println!("{}", s);
}
