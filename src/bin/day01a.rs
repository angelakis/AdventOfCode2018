extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn main() {
    let input = read_from_file("input/day01a.txt");
    let result: i64 = input.split('\n')
                           .map(|x| x.parse::<i64>().unwrap_or_default())
                           .sum::<i64>();
    println!("Sum is {}", result);
}
