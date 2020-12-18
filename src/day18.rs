pub fn solve1(input: &[String]) {
    let vals: Vec<i64> = input.iter().map(|s| eval(s)).collect();
    let s: i64 = vals.iter().sum();
    dbg!(vals, s);
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

fn eval(s: &str) -> i64 {
    let mut stack = vec![(Op::Add, 0i64)];
    let mut current = 0i64;
    for c in s.chars() {
        match c {
            ' ' => continue,
            '0'..='9' => {
                current *= 10;
                current += char::to_digit(c, 10).unwrap() as i64;
            }
            '+' => {
                let (op, stack_top) = stack.last_mut().unwrap();
                match op {
                    Op::Add => {
                        *stack_top += current;
                    }
                    Op::Mul => {
                        *stack_top *= current;
                    }
                }
                *op = Op::Add;
                current = 0;
            }
            '*' => {
                let (op, stack_top) = stack.last_mut().unwrap();
                match op {
                    Op::Add => {
                        *stack_top += current;
                    }
                    Op::Mul => {
                        *stack_top *= current;
                    }
                }
                *op = Op::Mul;
                current = 0;
            }
            '(' => {
                stack.push((Op::Add, 0));
            }
            ')' => {
                let (op, stack_top) = stack.last_mut().unwrap();
                match op {
                    Op::Add => {
                        *stack_top += current;
                    }
                    Op::Mul => {
                        *stack_top *= current;
                    }
                }
                current = stack.pop().unwrap().1;
            }
            _ => unreachable!(),
        }
    }
    let (op, stack_top) = stack.last_mut().unwrap();
    match op {
        Op::Add => {
            *stack_top += current;
        }
        Op::Mul => {
            *stack_top *= current;
        }
    }
    stack.pop().unwrap().1
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Add,
    Mul,
    Number(i64),
}

fn eval_2(s: &str) -> i64 {
    let mut chars = s.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '+' => tokens.push(Token::Add),
            '*' => tokens.push(Token::Mul),
            _ if c.is_digit(10) => {
                let mut n = c.to_digit(10).unwrap();
                while let Some(c) = chars.peek() {
                    match c.to_digit(10) {
                        None => break,
                        Some(x) => {
                            n *= 10;
                            n += x;
                            chars.next();
                        }
                    }
                }
                tokens.push(Token::Number(n as i64));
            }
            '(' => {
                let mut parens = 1;
                let mut inner = String::new();
                while parens > 0 {
                    let c = chars.next().unwrap();
                    match c {
                        '(' => parens += 1,
                        ')' => parens -= 1,
                        _ => {}
                    }
                    inner.push(c);
                }
                inner.pop();
                tokens.push(Token::Number(eval_2(&inner)));
            }
            _ => unreachable!(),
        }
    }
    while let Some(first_plus) = tokens.iter().position(|t| *t == Token::Add) {
        let n1 = if let Token::Number(n) = tokens[first_plus - 1] {
            n
        } else {
            unreachable!()
        };
        let n2 = if let Token::Number(n) = tokens[first_plus + 1] {
            n
        } else {
            unreachable!()
        };
        let sum = n1 + n2;
        tokens[first_plus] = Token::Number(sum);
        tokens.remove(first_plus + 1);
        tokens.remove(first_plus - 1);
    }
    while let Some(first_plus) = tokens.iter().position(|t| *t == Token::Mul) {
        let n1 = if let Token::Number(n) = tokens[first_plus - 1] {
            n
        } else {
            unreachable!()
        };
        let n2 = if let Token::Number(n) = tokens[first_plus + 1] {
            n
        } else {
            unreachable!()
        };
        let sum = n1 * n2;
        tokens[first_plus] = Token::Number(sum);
        tokens.remove(first_plus + 1);
        tokens.remove(first_plus - 1);
    }
    assert_eq!(tokens.len(), 1);
    if let Token::Number(n) = tokens[0] {
        n
    } else {
        unreachable!()
    }
}

pub fn solve2(input: &[String]) {
    let vals: Vec<i64> = input.iter().map(|s| eval_2(s)).collect();
    let s: i64 = vals.iter().sum();
    dbg!(vals, s);
}
