use std::fs;
use std::collections::VecDeque;
use anyhow::{anyhow, Result};

pub fn run_part1() -> Result<()> {
    let contents = fs::read_to_string("6_input.txt")?;
    let i = find_start_of_packet_marker(&contents, 4)?;
    println!("Day6.1 result: {}", i);
    Ok(())
}

pub fn run_part2() -> Result<()> {
    let contents = fs::read_to_string("6_input.txt")?;
    let i = find_start_of_packet_marker(&contents, 14)?;
    println!("Day6.2 result: {}", i);
    Ok(())
}

pub fn find_start_of_packet_marker(msg: &str, marker_len: usize) -> Result<usize> {

    // checked "clean" buffer with no duplicates
    // trails max 4 behind current cursor
    let mut buf: VecDeque<char> = VecDeque::new();

    for (i, c) in msg.char_indices() {
        if buf.len() == marker_len { 
            return Ok(i) 
        }

        if let Some(buf_i) = buf.iter().position(|&b| b == c) {
            for _ in 0..=buf_i { buf.pop_front(); }
        }

        buf.push_back(c);
    }

    Err(anyhow!("not found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn all() {
        assert!(find_start_of_packet_marker("123", 4).is_err());

        for (msg, res1, res2) in INPUT {
            assert_eq!(
                res1, 
                find_start_of_packet_marker(msg, 4).unwrap(),
                "for {}", msg);
            assert_eq!(
                res2, 
                find_start_of_packet_marker(msg, 14).unwrap(),
                "for {}", msg);
        }
    }
}
