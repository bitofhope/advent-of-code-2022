use std::env;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = std::fs::File::open(file_path).unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let mut calories = 0u64;
    let mut elves = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u64>().unwrap();
        }
    }
    elves.push(calories);
    part_1(elves.clone());
    part_2(elves);
}

fn part_1(elves: Vec<u64>) {
    println!("{}", elves.iter().max().unwrap());
}

fn part_2(elves: Vec<u64>) {
    let (mut elf1, mut elf2, mut elf3) = (0, 0, 0);
    for &elf in elves.iter() {
        if elf > elf1 {
            (elf1, elf2, elf3) = (elf, elf1, elf2);
        } else if elf > elf2 {
            (elf2, elf3) = (elf, elf2);
        } else if elf > elf3 {
            elf3 = elf;
        }
    }
    println!("{}", elf1 + elf2 + elf3);
}
