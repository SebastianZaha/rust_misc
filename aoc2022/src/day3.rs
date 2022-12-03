use std::fs;

pub fn run_part1() {
    let contents = fs::read_to_string("3_input.txt").expect("Cannot read input file");

    let mut sum: usize = 0;

    for line in contents.lines() {
        
        let mut half1 = [0; 53];
        let mut half2 = [0; 53];
        let len = line.len();

        for (i, c) in line.char_indices() {
            let idx = char_val(c); 
            if i < len/2 { 
                half1[idx] += 1;
            } else if half1[idx] > 0 && half2[idx] == 0 {
                sum += idx;
                half2[idx] += 1;
            }
        }
    }

    println!("Day3.1: {sum}")
}

pub fn run_part2() {
    let contents = fs::read_to_string("3_input.txt").expect("Cannot read input file");
    let mut sum: usize = 0;

    let mut it = contents.lines();
    loop {
        let l = it.next();
        if l == None {
            break;
        }
        let mut chars = [0; 53];
        for c in l.unwrap().chars() {
            chars[char_val(c)] = 1;
        }
        for c in it.next().unwrap().chars() {
            if chars[char_val(c)] == 1 {
                chars[char_val(c)] = 2;
            }
        }
        for c in it.next().unwrap().chars() {
            let val = char_val(c);
            if chars[val] == 2 {
                sum += val;
                break;
            }
        }
    }

    println!("Day3.2: {sum}")
}

fn char_val(c: char) -> usize {
    let uc = c as usize;
    if uc >= 97 {
        uc - 96      // a-z = 01-26
    } else {
        uc - 65 + 27 // A-Z = 27-52
    }
}
