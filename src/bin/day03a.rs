extern crate aoc2018;

use aoc2018::files::read_from_file;

pub struct Claim {
    id: usize,
    from_left: usize,
    from_top: usize,
    wide: usize,
    tall: usize
}

impl Claim {
    pub fn new(claim_line: &str) -> Claim {
        let mut numbers = Vec::new();
        let mut number = Vec::new();
        for ch in claim_line.chars() {
            match ch {
                '#' | '@' => {},
                ' ' | ',' | ':' | 'x' => {
                    if !number.is_empty() {
                        let cloned_number = number.clone();
                        number.clear();
                        let num: usize = cloned_number.into_iter()
                                               .collect::<String>()
                                               .parse()
                                               .unwrap();
                        numbers.push(num);
                    }
                },
                _ => number.push(ch),
            }
        }
        let cloned_number = number.clone();
        number.clear();
        let num: usize = cloned_number.into_iter()
                               .collect::<String>()
                               .parse()
                               .unwrap();
        Claim{
            id: numbers[0],
            from_left: numbers[1],
            from_top: numbers[2],
            wide: numbers[3],
            tall: num}
    }
}

pub fn main() {
    let input = read_from_file("input/day03a.txt");
    let mut claims = Vec::new();
    for claim_line in input.lines() {
        claims.push(Claim::new(claim_line));
    }
    println!("Overlapping square inches = {:?}", claims[3].from_left);
}
