extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn get_counts(mut chars: Vec<char>) -> (usize, usize) {
    chars.sort();
    let mut cur_char = ' ';
    let mut char_streak = 0;
    let mut count2 = false;
    let mut count3 = false;
    for ch in chars {
        if ch != cur_char {
            match char_streak {
                2 => count2 = true,
                3 => count3 = true,
                _ => {},
            }
            char_streak = 0;
            cur_char = ch;
        }
        if count2 && count3 {
            break
        }
        char_streak += 1;
    }
    match char_streak {
        2 => count2 = true,
        3 => count3 = true,
        _ => {},
    }
    (count2 as usize, count3 as usize)
}

pub fn main() {
    let input = read_from_file("input/day02a.txt");
    let mut count2 = 0;
    let mut count3 = 0;
    for word in input.lines() {
        let chars: Vec<char> = word.chars().collect();
        let (count2_inc, count3_inc) = get_counts(chars);
        count2 += count2_inc;
        count3 += count3_inc;
    }
    println!("Checksum is {}", count2 * count3);
}
