use std::fs;
use regex::Regex;

pub fn run_part1() {
    let contents = fs::read_to_string("4_input.txt").expect("Cannot read input file");

    let mut count: usize = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    for line in contents.lines() {
        let m = re.captures_iter(line).next().unwrap(); 
        let a1: usize = m[1].parse().unwrap();
        let a2: usize = m[2].parse().unwrap();
        let b1: usize = m[3].parse().unwrap();
        let b2: usize = m[4].parse().unwrap();
        if (a1 <= b1 && a2 >= b2) || (a1 >= b1 && a2 <= b2) {
            count += 1;
        }
    }

    println!("Day3.1: {count}");
}

pub fn run_part2() {
    let contents = fs::read_to_string("4_input.txt").expect("Cannot read input file");
        let mut count: usize = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    for line in contents.lines() {
        let m = re.captures_iter(line).next().unwrap(); 
        let a1: usize = m[1].parse().unwrap();
        let a2: usize = m[2].parse().unwrap();
        let b1: usize = m[3].parse().unwrap();
        let b2: usize = m[4].parse().unwrap();
        if a2 >= b1 && a1 <= b2 {
            count += 1;
        }
    }

    println!("Day3.2: {count}");
}

