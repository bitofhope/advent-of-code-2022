use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();
    part1(input.clone());
    part2(input);
}

fn part1(input: String) {
    let mut input = input.chars();
    let (mut a, mut b, mut c, mut d) =
        (input.next(), input.next(), input.next(), input.next());
    let mut answer = 4u64;
    while a == b || a == c || a == d || b == c || b == d || c == d {
        (a, b, c, d) = (b, c, d, input.next());
        answer += 1;
    }
    println!("{answer}");
}

fn part2(input: String) {
    let mut input = input.chars();
    let mut signal = VecDeque::new();
    let mut answer = 14u64;
    for _ in 0..answer {
        signal.push_back(input.next().unwrap());
    }
    while !all_distinct(signal.clone()) {
        signal.pop_front();
        signal.push_back(input.next().unwrap());
        answer += 1;
    }
    println!("{answer}");
}

fn all_distinct<T: std::cmp::PartialEq>(v: VecDeque<T>) -> bool {
    let mut v = v;
    if v.len() < 2 {
        return true;
    }
    let first = v.pop_front().unwrap();
    for item in v.iter() {
        if &first== item {
            return false;
        }
    }
    all_distinct(v)
}

