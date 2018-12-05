extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn main() {
    let input = read_from_file("input/day05a.txt");
    let mut polymer: Vec<char> = Vec::new();
    for elem in input.chars() {
        if !elem.is_alphabetic() {
            continue;
        }
        if !polymer.is_empty() &&
                elem != polymer[polymer.len()-1] &&
                (elem.to_lowercase().to_string() ==
                 polymer[polymer.len()-1].to_lowercase().to_string()) {
            polymer.pop();
            continue;
        }
        polymer.push(elem);
    }
    println!("Length is = {}", polymer.len());
}
