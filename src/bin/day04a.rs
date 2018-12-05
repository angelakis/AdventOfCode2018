use std::collections::HashMap;

extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn change_format(input: String) -> Vec<(usize, usize, String, String)> {
    let mut infos = Vec::new();
    for line in input.lines() {
        let simpler = line.replace("[", "")
                          .replace("]", "")
                          .replace("-", "")
                          .replace("#", "")
                          .replace(":", "");
        let words: Vec<&str> = simpler.split(' ').collect();
        let date: usize = words[0].parse().unwrap();
        let time: usize = words[1].parse().unwrap();
        infos.push((date, time, words[2].to_string(), words[3].to_string()));
    }
    infos.sort();
    infos
}

pub fn main() {
    let input = read_from_file("input/day04a.txt");
    let infos = change_format(input);
    let mut cur_guard = 0usize;
    let mut fell_asleep = 0;
    let mut cur_max_guard = 0;
    let mut cur_max_minutes = 0;
    let mut sums = HashMap::new();
    for info in infos.iter() {
        match *info {
            (_, _, ref s, ref g) if s == "Guard" => cur_guard = g.parse().unwrap(),
            (_, t, ref s, _) if s == "falls" => fell_asleep = t,
            (_, woke, ref s, _) if s == "wakes" => {
                let minutes = sums.entry(cur_guard).or_insert(0);
                *minutes += woke - fell_asleep;
                if *minutes >= cur_max_minutes {
                    cur_max_guard = cur_guard;
                    cur_max_minutes = *minutes;
                }
            },
            _ => panic!("Something went terribly wrong"),
        }
    }
    let mut minute_freqs = vec!(0; 60);
    for info in infos.iter() {
        match *info {
            (_, _, ref s, ref g) if s == "Guard" => cur_guard = g.parse().unwrap(),
            (_, t, ref s, _) if s == "falls" => fell_asleep = t,
            (_, woke, ref s, _) if s == "wakes" && cur_guard == cur_max_guard => {
                for m in fell_asleep..woke {
                    minute_freqs[m] += 1;
                }
            },
            _ => {},
        }

    }
    let best_minute = minute_freqs.iter().enumerate()
                                  .map(|(x, y)| (y, x)).max().unwrap().1;
    println!("Result is = {}", cur_max_guard * best_minute);
}
