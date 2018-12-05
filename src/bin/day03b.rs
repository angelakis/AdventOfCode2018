extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn extract_corners(claim: &str) -> ((usize, usize), (usize, usize)) {
    let words: Vec<&str> = claim.split(':').collect();
    let first_two: Vec<usize> = words[0].split(' ').collect::<Vec<&str>>()[2]
                                       .split(',')
                                       .map(|x| x.parse().unwrap()).collect();
    let second_two: Vec<usize> = words[1].trim().split('x')
                                         .map(|x| x.parse().unwrap()).collect();
    ((first_two[0], first_two[1]),
     (first_two[0] + second_two[0], first_two[1] + second_two[1]))
}

pub fn main() {
    let input = read_from_file("input/day03a.txt");
    let mut matrix = vec!(vec!(0; 1000); 1000);
    for claim_line in input.lines() {
        let ((x1, y1), (x2, y2)) = extract_corners(claim_line);
        for x in x1..x2 {
            for y in y1..y2 {
                match matrix[x][y] {
                    0 => matrix[x][y] += 1,
                    1 => {
                        matrix[x][y] += 1;
                    },
                    _ => {},
                }
            }
        }
    }
    for (claim_no, claim_line) in input.lines().enumerate() {
        let ((x1, y1), (x2, y2)) = extract_corners(claim_line);
        let mut overlapped = false;
        for x in x1..x2 {
            for y in y1..y2 {
                match matrix[x][y] {
                    2 => overlapped = true,
                    _ => {},
                }
            }
        }
        if !overlapped {
            println!("Claim not overlapping is {}", claim_no + 1);
        }
    }
}
