use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();
    let input = input.chars();

    let mut trees = Vec::new();
    let mut tree_row: Vec<u32> = Vec::new();
    for c in input {
        match c {
            '\n' => {
                trees.push(tree_row.clone());
                tree_row = Vec::new();
            },
            _ => {
                tree_row.push(c.to_digit(10).unwrap());
            },
        }
    }
    part1(&trees);
    part2(&trees);
}

fn visible(x: usize, y: usize, trees: &Vec<Vec<u32>>) -> bool {
    let height = trees[x][y];
    let (left, right) = trees[x].split_at(y);
    let (_, right) = right.split_first().unwrap();
    match left.iter().max() {
        Some(&max) => if max < height { return true; },
        None => return true,
    }
    match right.iter().max() {
        Some(&max) => if max < height { return true; },
        None => return true,
    }
    let vertical: Vec<u32> = trees.iter().map(|v| v[y]).collect();
    let (up, down) = vertical.split_at(x);
    let (_, down) = down.split_first().unwrap();
    match up.iter().max() {
        Some(&max) => if max < height { return true; },
        None => return true,
    }
    match down.iter().max() {
        Some(&max) => if max < height { return true; },
        None => return true,
    }
    false
}

fn scenic_score(x: usize, y: usize, trees: &Vec<Vec<u32>>) -> usize {
    let height = trees[x][y];
    let mut up = 0;
    for x in (0..x).rev() {
        up += 1;
        if trees[x][y] >= height { break; }
    }
    let mut down = 0;
    for x in x+1..trees.len() {
        down += 1;
        if trees[x][y] >= height { break; }
    }
    let mut left = 0;
    for y in (0..y).rev() {
        left += 1;
        if trees[x][y] >= height { break; }
    }
    let mut right = 0;
    for y in y+1..trees.len() {
        right += 1;
        if trees[x][y] >= height { break; }
    }
    up*left*right*down
}

fn part1(trees: &Vec<Vec<u32>>) {
    let mut total = 0;
    for x in 0..trees.len() {
        for y in 0..trees[x].len() {
            if visible(x, y, trees) {
                total += 1;
            } else {
            }
        }
    }
    println!("{total}");
}

fn part2(trees: &Vec<Vec<u32>>) {
    let mut max = 0;
    // Edge trees always see a 0 in one direction
    for x in 1..(trees.len()-1) {
        for y in 1..(trees[0].len()-1) {
            let score = scenic_score(x, y, trees);
            if score > max {
                max = score;
            }
        }
    }
    println!("{max}");
}

