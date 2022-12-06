use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::str::{Chars, Lines};

enum ReadCrateResult {
    Some(char),
    None,
    EndCrates,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();
    let mut input = input.chars().peekable();
    let mut stacks = Vec::new();
    while input.peek() != Some(&'\n') {
        if input.peek() == Some(&' ') {
            input.next();
        }
        match read_crate(&mut input) {
            ReadCrateResult::Some(c) => stacks.push(VecDeque::from([c])),
            ReadCrateResult::None => stacks.push(VecDeque::new()),
            ReadCrateResult::EndCrates => break,
        }
    }
    input.next();
    'build_stacks: loop {
        for stack in stacks.iter_mut() {
            match read_crate(&mut input) {
                ReadCrateResult::Some(c) => stack.push_front(c),
                ReadCrateResult::None => {},
                ReadCrateResult::EndCrates => break 'build_stacks,
            }
            input.next();
        }
    }
    let input: String = input.collect();
    let mut input = input.lines();
    input.next();
    input.next();
    part1(stacks.clone(), &mut input.clone());
    part2(stacks, &mut input);
}

fn read_crate(input: &mut Peekable<Chars>) -> ReadCrateResult {
    match input.peek() {
        Some('[') => {
            input.next();
            let letter = input.next().unwrap();
            input.next();
            ReadCrateResult::Some(letter)
        },
        _ => {
            input.next();
            if input.next() == Some('1') {
                input.next();
                ReadCrateResult::EndCrates
            } else {
                input.next();
                ReadCrateResult::None
            }
        },
    }
}

fn part1(mut stacks: Vec<VecDeque<char>>, input: &mut Lines) {
    for line in input {
        let mut words = line.split(' ');
        if words.next().unwrap() != "move" {
            panic!("bad input line: {line}");
        }
        let crates: usize = words.next().unwrap().parse().unwrap();
        if words.next().unwrap() != "from" {
            panic!("bad input line: {line}");
        }
        let from: usize = words.next().unwrap().parse::<usize>().unwrap() - 1;
        if words.next().unwrap() != "to" {
            panic!("bad input line: {line}");
        }
        let to: usize = words.next().unwrap().parse::<usize>().unwrap() - 1;
        for _ in 0..crates {
            let tmp = stacks[from].pop_back().unwrap();
            stacks[to].push_back(tmp);
        }
    }
    let result: String = stacks.iter().map(|s| s.back().unwrap()).collect();
    println!("{result}");
}

fn part2(mut stacks: Vec<VecDeque<char>>, input: &mut Lines) {
    for line in input {
        let mut words = line.split(' ');
        if words.next().unwrap() != "move" {
            panic!("bad input line: {line}");
        }
        let crates: usize = words.next().unwrap().parse().unwrap();
        if words.next().unwrap() != "from" {
            panic!("bad input line: {line}");
        }
        let from: usize = words.next().unwrap().parse::<usize>().unwrap() - 1;
        if words.next().unwrap() != "to" {
            panic!("bad input line: {line}");
        }
        let to: usize = words.next().unwrap().parse::<usize>().unwrap() - 1;
        let mut tmp_stack = VecDeque::new();
        for _ in 0..crates {
            let tmp = stacks[from].pop_back().unwrap();
            tmp_stack.push_front(tmp);
        }
        stacks[to].append(&mut tmp_stack);
    }
    let result: String = stacks.iter().map(|s| s.back().unwrap()).collect();
    println!("{result}");
}
