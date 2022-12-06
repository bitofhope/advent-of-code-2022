use std::env;
use std::io::BufRead;

#[derive(Debug,Clone)]
struct Elf {
    begin: usize,
    end: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = std::fs::File::open(file_path).unwrap();
    let lines = std::io::BufReader::new(file).lines();

    let mut assignments = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let numbers = line.split(&['-', ',']);
        let numbers: Vec<usize> = numbers.map(|s| s.parse().unwrap())
            .collect();
        let elf1 = Elf { begin: numbers[0], end: numbers[1] };
        let elf2 = Elf { begin: numbers[2], end: numbers[3] };
        assignments.push((elf1, elf2));
    }
    part_1(assignments.clone());
    part_2(assignments);
}

fn full_overlap(a: Elf, b: Elf) -> bool {
    (a.begin <= b.begin && a.end >= b.end) ||
        (b.begin <= a.begin && b.end >= a.end)
}

fn partial_overlap(a: Elf, b: Elf) -> bool {
    (a.begin <= b.begin && a.end >= b.begin) ||
        (b.begin <= a.begin && b.end >= a.begin)
}

fn part_1(assignments: Vec<(Elf, Elf)>) {
    let mut total = 0u32;
    for (a, b) in assignments.iter() {
        if full_overlap((*a).clone(), (*b).clone()) {
            total +=1;
        }
    }
    println!("{total}");
}

fn part_2(assignments: Vec<(Elf, Elf)>) {
    let mut total = 0u32;
    for (a, b) in assignments.iter() {
        if partial_overlap((*a).clone(), (*b).clone()) {
            total +=1;
        }
    }
    println!("{total}");
}
