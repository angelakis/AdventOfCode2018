extern crate aoc2018;

use std::collections::HashSet;
use aoc2018::files::read_from_file;

pub fn main() {
    let input = read_from_file("input/day01a.txt");
    let freqs: Vec<i64> = input.lines()
                               .map(|x| x.parse::<i64>().unwrap_or_default())
                               .collect();
    let mut found_freqs = HashSet::new();
    let mut current_freq: i64 = 0;
    loop {
        for shift in &freqs {
            current_freq += shift;
            if found_freqs.contains(&current_freq) {
                println!("Duplicate freq found {}", current_freq);
                return
            }
            found_freqs.insert(current_freq);
        }
    }
}
