use std::env;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = std::fs::File::open(file_path).unwrap();
    let lines = std::io::BufReader::new(file).lines();

    let mut rucksacks = Vec::new();
    let mut rucksacks_split = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut bag1: Vec<char> = line.chars().collect();
        rucksacks.push(Box::from(bag1.clone()));
        let bag2 = bag1.split_off(bag1.len() / 2);
        rucksacks_split.push((Box::from(bag1), Box::from(bag2)));
    }
    part_1(rucksacks_split);
    part_2(rucksacks);
}

fn priority(item: char) -> Option<u32> {
    if item.is_ascii_lowercase() {
        Some((item as u32) - ('a' as u32) + 1)
    } else if item.is_ascii_uppercase() {
        Some((item as u32) - ('A' as u32) + 27)
    } else {
        None
    }
}

fn part_1(rucksacks: Vec<(Box<Vec<char>>, Box<Vec<char>>)>) {
    let mut total = 0u32;
    for (comp1, comp2) in rucksacks.iter() {
        'sack: for item1 in comp1.iter() {
            for item2 in comp2.iter() {
                if item1 == item2 {
                    total += priority(*item1).unwrap();
                    break 'sack;
                }
            }
        }
    }
    println!("{total}");
}

fn part_2(rucksacks:Vec<Box<Vec<char>>>) {
    let mut total = 0u32;
    let mut sacks = rucksacks.iter();
    while let Some(sack1) = sacks.next() {
        let (sack2, sack3) = (sacks.next().unwrap(), sacks.next().unwrap());
        for c in sack1.iter() {
            if sack2.contains(c) && sack3.contains(c) {
                total += priority(*c).unwrap();
                break;
            }
        }
    }
    println!("{total}");
}
