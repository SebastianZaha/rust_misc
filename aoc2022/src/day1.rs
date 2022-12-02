use std::fs;

fn check_max(max: &mut(u32,u32,u32), current: u32) {
    if current > max.0 { 
        max.2 = max.1;
        max.1 = max.0;
        max.0 = current;
    } else if current > max.1 { 
        max.2 = max.1;
        max.1 = current;
    } else if current > max.2 {
        max.2 = current;
    }
}

pub fn run() {
    let contents = fs::read_to_string("1_input.txt")
        .expect("Cannot read input file");

    let mut max: (u32,u32,u32) = (0,0,0);
    let mut current: u32 = 0;

    for line in contents.lines() {
        if line.len() == 0 {
            check_max(&mut max, current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }
    check_max(&mut max, current);

    println!("Day1: Top 3 max values: {:?}: {}\n", max, max.0+max.1+max.2);
}
