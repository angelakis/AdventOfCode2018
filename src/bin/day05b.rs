extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn make_reactions(polymer_str: String) -> usize {
    let mut polymer: Vec<char> = Vec::new();
    for elem in polymer_str.chars() {
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
    polymer.len()
}

pub fn main() {
    let input = read_from_file("input/day05a.txt");
    let mut types: Vec<String> = input.chars().map(|x| x.to_lowercase().to_string()).collect();
    types.sort();
    types.dedup();
    let mut best = input.len();
    for polymer_type in types {
        if polymer_type == "\n" {
            continue
        }
        let improved = input.replace(&polymer_type, "")
                            .replace(&polymer_type.to_uppercase(), "");
        let improved_len = make_reactions(improved);
        if improved_len < best {
            best = improved_len;
        }
    }
    println!("Length is = {}", best);
}
