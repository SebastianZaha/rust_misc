use std::fs;

// rock   paper  scissors
// 0/A/X  1/B/Y  2/C/Z
// 0 > 2, 1 > 0, 2 > 1

pub fn run_part1() {
    let contents = fs::read_to_string("2_input.txt").expect("Cannot read input file");

    let mut score: u32 = 0;

    for line in contents.lines() {
        let abc = line.chars().nth(0).unwrap();
        let xyz = line.chars().nth(2).unwrap();
        score += if xyz == 'X' {
            if abc == 'A' {
                1 + 3
            } else if abc == 'C' {
                1 + 6
            } else {
                1
            }
        } else if xyz == 'Y' {
            if abc == 'B' {
                2 + 3
            } else if abc == 'A' {
                2 + 6
            } else {
                2
            }
        } else if xyz == 'Z' {
            if abc == 'C' {
                3 + 3
            } else if abc == 'B' {
                3 + 6
            } else {
                3
            }
        } else {
            panic!("unknown line format |{abc} {xyz}|")
        }
    }
    println!("Day2: {}", score)
}

pub fn run() {
    let contents = fs::read_to_string("2_input.txt").expect("Cannot read input file");

    let mut score: u32 = 0;

    for line in contents.lines() {
        let abc = line.chars().nth(0).unwrap();
        let xyz = line.chars().nth(2).unwrap();

        score += if xyz == 'X' {
            if abc == 'A' {
                3
            } else if abc == 'B' {
                1
            } else {
                2
            }
        } else if xyz == 'Y' {
            3 + if abc == 'A' {
                1
            } else if abc == 'B' {
                2 
            } else {
                3
            }
        } else if xyz == 'Z' {
            6 + if abc == 'A' {
                2
            } else if abc == 'B' {
                3
            } else {
                1
            }
        } else {
            panic!("unknown line format |{abc} {xyz}|")
        };
    }
    println!("Day2: {}", score)
}
