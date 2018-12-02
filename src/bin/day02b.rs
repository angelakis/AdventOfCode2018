extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn matching_ids(id1: &str, id2: &str) -> bool {
    let chars1: Vec<u8> = id1.chars().map(|x| x as u8).collect();
    let chars2: Vec<u8> = id2.chars().map(|x| x as u8).collect();
    let different: usize = chars1.iter()
                                    .zip(chars2)
                                    .map(|(x, y)| (*x != y) as usize)
                                    .sum();
    different == 1
}

pub fn get_common_part<'a>(id1: &'a str, id2: &'a str) -> String {
    let chars1: Vec<char> = id1.chars().collect();
    let chars2: Vec<char> = id2.chars().collect();
    let common_string: String = chars1.iter().zip(chars2)
                 .map(|(x, y)| { if *x == y {*x} else {' '}})
                 .filter(|x| *x != ' ')
                 .collect();
    common_string
}

pub fn main() {
    let input = read_from_file("input/day02a.txt");
    let cloned_input = input.clone();
    let lines: Vec<&str> = cloned_input.lines().collect();
    let mut common = String::new();

    for (line_num, id) in input.lines().enumerate() {
        for id2 in lines.iter().skip(line_num) {
            if matching_ids(id, id2) {
                common = get_common_part(id, id2);
            }
        }
    } 
    println!("Common part is {}", common);
}
