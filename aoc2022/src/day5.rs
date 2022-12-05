use std::fs;
use regex::Regex;

pub fn run_part1() {
    let contents = fs::read_to_string("5_input.txt").expect("Cannot read input file");

    let mut cols: [Vec<char>; 9] = Default::default();
    let mut reading_positions = true;
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in contents.lines() {
        let mut chars = line.chars();
        if reading_positions {
            if line.is_empty() {
                println!("found empty line, finished reading stacks");
                reading_positions = false;
                for c in cols.iter_mut() {
                    c.reverse();
                    println!("read stack: {:?}", c);
                }
            } else {
                if let Some(c) = chars.nth(1) {
                    if c == '1' {
                        println!("found 'informative' line with numbers, skipping");
                        continue;
                    } else if c != ' ' {
                        cols[0].push(c);
                    }
                }
                for i in 1..9 {
                    if let Some(c) = chars.nth(3) { 
                        if c != ' ' { cols[i].push(c) }
                    }
                }
            }
        } else {
			// all state was read, we are processing move instructions one by one now
            let m = re.captures_iter(line).next().unwrap();
            let count = m[1].parse::<usize>().unwrap();
            let from  = m[2].parse::<usize>().unwrap() - 1;
            let to    = m[3].parse::<usize>().unwrap() - 1;

            let n = cols[from].len();
            let mut tosplit = cols[from].split_off(n - count);

            // remove reverse for part2 of the problem
            tosplit.reverse();

            cols[to].append(&mut tosplit);

            // println!("after move step: {} {} {} {:?}", count, from , to, cols);
        }
    }

	print!("Day5.1 result: ");
    for col in cols {
        print!("{}", col.last().unwrap());
    }
	println!("");
}

pub fn run_part2() {
    println!("Day5.2: same as part1, without reverse on the moved stack");
}
